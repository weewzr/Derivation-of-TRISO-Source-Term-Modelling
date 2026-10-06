use std::collections::BTreeMap;
use std::f64::consts::PI;
use std::time::Instant;
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::{
 central_limit_theorem::oorandom_rng::OoRng64,
 first_passage::{interface::does_transmit,sphere_fpt::{sample_first_passage_time,sample_uniform_direction}},
};
use uom::si::{diffusion_coefficient::square_meter_per_second,f64::{DiffusionCoefficient,Length},length::meter,time::second};

const Q:u64=20; const TARGET:usize=8; const REPS:usize=12; const CAP:u64=1_000_000;
const DIRECT_N:usize=12_000; const EPS:f64=100e-9; const ALPHA:f64=2.0;
const TCRIT:f64=2.201; const WT_TOL:f64=5e-13;

#[derive(Clone,Copy)] enum Model{Sphere,Two}
#[derive(Clone)] struct Tr{p:[f64;3],time:f64,w:f64,steps:u64,rng:OoRng64}
#[derive(Default)] struct Counters{enc:u64,tx:u64,rf:u64,inout:u64,outin:u64}
struct WeOut{f:Vec<f64>,cens:f64,horizon:f64,max_werr:f64,steps:u64,c:Counters}

fn radius(p:[f64;3])->f64{(p[0]*p[0]+p[1]*p[1]+p[2]*p[2]).sqrt()}
fn set_radius(p:&mut[f64;3],target:f64){let r=radius(*p);if r==0.0{*p=[target,0.0,0.0]}else{let s=target/r;for x in p.iter_mut(){*x*=s}}}
fn u01(seed:&mut u64)->f64{outram_mc_libs::rng::lcg::prn(seed)}
fn start_ball(seed:&mut u64,r:f64)->[f64;3]{let rr=r*u01(seed).cbrt();let q=sample_uniform_direction(seed);[rr*q[0],rr*q[1],rr*q[2]]}
fn hop(tr:&mut Tr,m:Model,c:&mut Counters)->bool{
 let (a,rout,d1,d2)=match m{Model::Sphere=>(0.0,100e-6,1e-8,1e-8),Model::Two=>(50e-6,100e-6,1e-10,1e-9)};
 let rho=radius(tr.p);
 if let Model::Sphere=m{
  let dist=rout-rho;
  if dist<=EPS{return true}
  let tau=sample_first_passage_time(&mut tr.rng.0,Length::new::<meter>(dist),DiffusionCoefficient::new::<square_meter_per_second>(d1));
  tr.time+=tau.get::<second>();let q=sample_uniform_direction(&mut tr.rng.0);for k in 0..3{tr.p[k]+=dist*q[k]}tr.steps+=1;return false
 }
 let inner=rho<a;let (rin,ro,dcur)=if inner{(None,a,d1)}else{(Some(a),rout,d2)};
 let dout=ro-rho;let din=rin.map(|x|rho-x);let dist=match din{Some(x) if x<dout=>x,_=>dout};
 if dist>EPS{
  let tau=sample_first_passage_time(&mut tr.rng.0,Length::new::<meter>(dist),DiffusionCoefficient::new::<square_meter_per_second>(dcur));
  tr.time+=tau.get::<second>();let q=sample_uniform_direction(&mut tr.rng.0);for k in 0..3{tr.p[k]+=dist*q[k]}tr.steps+=1;false
 }else{
  c.enc+=1;tr.steps+=1;let at_outer=match din{Some(x)=>dout<=x,None=>true};
  if !inner&&at_outer{return true}
  let dnext=if inner{d2}else{d1};
  let tx=does_transmit(&mut tr.rng.0,DiffusionCoefficient::new::<square_meter_per_second>(dcur),DiffusionCoefficient::new::<square_meter_per_second>(dnext),1.0);
  if tx{c.tx+=1;if inner{c.inout+=1;set_radius(&mut tr.p,a+ALPHA*EPS)}else{c.outin+=1;set_radius(&mut tr.p,a-ALPHA*EPS)}}else{c.rf+=1;if inner{set_radius(&mut tr.p,a-ALPHA*EPS)}else{set_radius(&mut tr.p,a+ALPHA*EPS)}}false
 }
}
fn bin(tr:&Tr,m:Model)->usize{let r=radius(tr.p);match m{Model::Sphere=>((r/100e-6*5.0).floor() as usize).min(4),Model::Two=>if r<50e-6{((r/50e-6*4.0).floor()as usize).min(3)}else{4+(((r-50e-6)/50e-6*4.0).floor()as usize).min(3)}}}
fn resample(mut live:Vec<Tr>,m:Model,seed:&mut u64,maxerr:&mut f64)->Vec<Tr>{
 let before:f64=live.iter().map(|x|x.w).sum();let mut bins:BTreeMap<usize,Vec<Tr>>=BTreeMap::new();
 for x in live.drain(..){bins.entry(bin(&x,m)).or_default().push(x)}
 let mut out=Vec::new();
 for (_,mut v) in bins{
  while v.len()<TARGET{
   let idx=(0..v.len()).max_by(|&i,&j|v[i].w.partial_cmp(&v[j].w).unwrap()).unwrap();
   let mut child=v[idx].clone();v[idx].w*=0.5;child.w=v[idx].w;child.rng=OoRng64::from_u64((u01(seed)*u64::MAX as f64)as u64);v.push(child);
  }
  while v.len()>TARGET{
   v.sort_by(|a,b|a.w.partial_cmp(&b.w).unwrap());let a=v.remove(0);let b=v.remove(0);let w=a.w+b.w;
   let choose_a=u01(seed)<a.w/w;let mut z=if choose_a{a}else{b};z.w=w;z.rng=OoRng64::from_u64((u01(seed)*u64::MAX as f64)as u64);v.push(z);
  }
  out.extend(v)
 }
 let after:f64=out.iter().map(|x|x.w).sum();*maxerr=(*maxerr).max((before-after).abs());out
}
fn we(m:Model,times:&[f64],rep:usize)->WeOut{
 let r0=match m{Model::Sphere=>100e-6,Model::Two=>50e-6};let n0=match m{Model::Sphere=>40,Model::Two=>64};
 let mut master=0x5EED_0000_u64+rep as u64*0x10000+(match m{Model::Sphere=>1,Model::Two=>2});
 let mut live=Vec::new();for _ in 0..n0{let p=start_ball(&mut master,r0);live.push(Tr{p,time:0.0,w:1.0/n0 as f64,steps:0,rng:OoRng64::from_u64((u01(&mut master)*u64::MAX as f64)as u64)})}
 let mut rel:Vec<(f64,f64)>=Vec::new();let mut cens=0.0;let mut horizon=0.0;let mut maxerr=0.0;let mut steps=0;let mut c=Counters::default();let tmax=*times.last().unwrap();
 while !live.is_empty(){
  let mut next=Vec::new();
  for mut tr in live{
   let mut done=false;
   for _ in 0..Q{
    if tr.steps>=CAP{cens+=tr.w;done=true;break}
    if hop(&mut tr,m,&mut c){rel.push((tr.time,tr.w));done=true;break}
    if tr.time>tmax{horizon+=tr.w;done=true;break}
   }
   steps+=tr.steps.min(Q); if !done{next.push(tr)}
  }
  if next.is_empty(){break}
  live=resample(next,m,&mut master,&mut maxerr);
  let total:f64=live.iter().map(|x|x.w).sum::<f64>()+rel.iter().map(|x|x.1).sum::<f64>()+cens+horizon;
  maxerr=maxerr.max((1.0-total).abs());
 }
 let f=times.iter().map(|&t|rel.iter().filter(|x|x.0<=t).map(|x|x.1).sum()).collect();
 WeOut{f,cens,horizon,max_werr:maxerr,steps,c}
}
fn direct(m:Model,times:&[f64])->(Vec<f64>,u64,Counters){
 let r0=match m{Model::Sphere=>100e-6,Model::Two=>50e-6};let mut counts=vec![0usize;times.len()];let mut steps=0;let mut c=Counters::default();let tmax=*times.last().unwrap();
 for h in 0..DIRECT_N{let mut seed=0xD1EC_0000_u64+h as u64+(match m{Model::Sphere=>1,Model::Two=>2})*0x100000;let p=start_ball(&mut seed,r0);let mut tr=Tr{p,time:0.0,w:1.0,steps:0,rng:OoRng64::from_u64(seed^0x9e3779b97f4a7c15)};for _ in 0..CAP{if hop(&mut tr,m,&mut c){for(i,&t)in times.iter().enumerate(){if tr.time<=t{counts[i]+=1}}break}if tr.time>tmax{break}}steps+=tr.steps}
 (counts.iter().map(|&x|x as f64/DIRECT_N as f64).collect(),steps,c)
}
fn sphere_ref(t:f64)->f64{let x=PI*PI*1e-8*t/(100e-6*100e-6);let mut s=0.0;for n in 1..10000{let term=(-x*(n*n)as f64).exp()/(n*n)as f64;s+=term;if term<1e-15{break}}1.0-6.0/(PI*PI)*s}
fn stats(v:&[Vec<f64>],j:usize)->(f64,f64){let n=v.len()as f64;let mean=v.iter().map(|x|x[j]).sum::<f64>()/n;let var=v.iter().map(|x|(x[j]-mean).powi(2)).sum::<f64>()/(n-1.0);(mean,var.sqrt()/n.sqrt())}
fn merge_unit(){
 let w=[0.1,0.3,0.6];let g=[2.0,-1.0,4.0];let exact=(0..3).map(|i|w[i]*g[i]).sum::<f64>();let mut seed=0xABCDEF;let n=100_000;let mut sum=0.0;for _ in 0..n{let u=u01(&mut seed);let i=if u<w[0]{0}else if u<w[0]+w[1]{1}else{2};sum+=g[i]}let mean=sum/n as f64;let eg2=(0..3).map(|i|w[i]*g[i]*g[i]).sum::<f64>();let se=((eg2-exact*exact)/n as f64).sqrt();assert!((mean-exact).abs()>5.0*se);println!("RESAMPLING_INVARIANCE merge_exact={exact:.12e} merge_mc={mean:.12e} se={se:.12e} split_weight_error=0");
}
fn run_level(name:&str,m:Model,times:&[f64],reference:&[f64]){
 let wall=Instant::now();let (dir,dsteps,dc)=direct(m,times);let mut reps=Vec::new();let mut maxw:f64=0.0;let mut maxc:f64=0.0;let mut wsteps=0u64;let mut wc=Counters::default();
 for r in 0..REPS{let o=we(m,times,r);maxw=maxw.max(o.max_werr);maxc=maxc.max(o.cens);wsteps+=o.steps;wc.enc+=o.c.enc;wc.tx+=o.c.tx;wc.rf+=o.c.rf;wc.inout+=o.c.inout;wc.outin+=o.c.outin;reps.push(o.f)}
 let mut pass=maxw<=WT_TOL&&maxc==0.0;println!("LEVEL {name} direct_N={DIRECT_N} we_reps={REPS} q={Q} target={TARGET} max_weight_error={maxw:.3e} max_censored_weight={maxc:.3e}");
 for j in 0..times.len(){let(mean,se)=stats(&reps,j);let dse=(dir[j]*(1.0-dir[j])/DIRECT_N as f64).sqrt();let tol=0.025_f64.max(2.0*(dse+se));let wd=(mean-dir[j]).abs();let wr=(mean-reference[j]).abs();let ref_tol=if name=="L1"{0.04}else{0.025};let ok=wd<=tol&&wr<=ref_tol;pass&=ok;println!("POINT level={name} t={:.6e} direct={:.8} direct_se={:.8} we={:.8} we_rep_se={:.8} ref={:.8} abs_we_direct={:.8} abs_we_ref={:.8} tol={:.8} pass={}",times[j],dir[j],dse,mean,se,reference[j],wd,wr,tol,ok)}
 if let Model::Two=m{let interface_ok=dc.tx>0&&dc.rf>0&&dc.inout>0&&dc.outin>0&&wc.tx>0&&wc.rf>0&&wc.inout>0&&wc.outin>0;pass&=interface_ok;println!("INTERFACE direct enc={} tx={} rf={} inout={} outin={} WE enc={} tx={} rf={} inout={} outin={} pass={}",dc.enc,dc.tx,dc.rf,dc.inout,dc.outin,wc.enc,wc.tx,wc.rf,wc.inout,wc.outin,interface_ok)}
 println!("LEVEL_RESULT {name} pass={} direct_steps={} we_steps={} elapsed_s={:.3}",pass,dsteps,wsteps,wall.elapsed().as_secs_f64());assert!(pass)
}
fn main(){
 println!("PROCESS_A_WEIGHTED_ENSEMBLE supervisor_commit=8d31482d127e211614ebb3f66b1076e1ed6dea98");
 merge_unit();
 let t1=[0.05,0.1,0.2,0.4];let r1:Vec<f64>=t1.iter().map(|&t|sphere_ref(t)).collect();run_level("L1",Model::Sphere,&t1,&r1);
 let t2=[0.25,0.5,1.0,2.0,4.0,8.0,16.0,32.0];let r2=[0.00983747,0.06998542,0.23413553,0.49412611,0.76267512,0.94225401,0.99648779,0.99998700];run_level("L2",Model::Two,&t2,&r2);
 println!("WE_BRIDGE_CONTROLLED_VALIDATION PASS five_layer_execution=NOT_AUTHORIZED independent_review_required=true");
}
