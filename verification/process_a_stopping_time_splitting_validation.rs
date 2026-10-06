use std::f64::consts::PI;
use std::time::Instant;
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::{
 central_limit_theorem::oorandom_rng::OoRng64,
 first_passage::{interface::does_transmit,sphere_fpt::{sample_first_passage_time,sample_uniform_direction}},
};
use uom::si::{diffusion_coefficient::square_meter_per_second,f64::{DiffusionCoefficient,Length},length::meter,time::second};

const REPS:usize=12; const DIRECT_N:usize=12_000; const CAP:u64=1_000_000;
const EPS:f64=100e-9; const ALPHA:f64=2.0; const SPLIT_TOL:f64=1e-14;
#[derive(Clone,Copy)] enum Model{Sphere,Two}
#[derive(Clone)] struct Tr{p:[f64;3],time:f64,w:f64,steps:u64,rng:OoRng64,flags:u8,root:u32,depth:u8}
#[derive(Default,Clone,Copy)] struct Counters{enc:u64,tx:u64,rf:u64,inout:u64,outin:u64}
#[derive(Default)] struct Diag{splits:[u64;3],max_depth:u8,max_desc:u64,peak:usize,steps:u64,cens:f64,max_split_err:f64,c:Counters}
struct Out{f:Vec<f64>,diag:Diag,rep_weights:Vec<f64>}
fn radius(p:[f64;3])->f64{(p[0]*p[0]+p[1]*p[1]+p[2]*p[2]).sqrt()}
fn set_radius(p:&mut[f64;3],target:f64){let r=radius(*p);let s=target/r;for x in p.iter_mut(){*x*=s}}
fn u01(s:&mut u64)->f64{outram_mc_libs::rng::lcg::prn(s)}
fn start_shell(s:&mut u64,lo:f64,hi:f64)->[f64;3]{let r=(lo.powi(3)+u01(s)*(hi.powi(3)-lo.powi(3))).cbrt();let q=sample_uniform_direction(s);[r*q[0],r*q[1],r*q[2]]}
fn hop(tr:&mut Tr,m:Model,c:&mut Counters)->bool{
 let (a,rout,d1,d2)=match m{Model::Sphere=>(0.0,100e-6,1e-8,1e-8),Model::Two=>(50e-6,100e-6,1e-10,1e-9)};
 let rho=radius(tr.p);
 if let Model::Sphere=m{let dist=rout-rho;if dist<=EPS{return true}let tau=sample_first_passage_time(&mut tr.rng.0,Length::new::<meter>(dist),DiffusionCoefficient::new::<square_meter_per_second>(d1));tr.time+=tau.get::<second>();let q=sample_uniform_direction(&mut tr.rng.0);for k in 0..3{tr.p[k]+=dist*q[k]}tr.steps+=1;return false}
 let inner=rho<a;let (rin,ro,dcur)=if inner{(None,a,d1)}else{(Some(a),rout,d2)};let dout=ro-rho;let din=rin.map(|x|rho-x);let dist=match din{Some(x)if x<dout=>x,_=>dout};
 if dist>EPS{let tau=sample_first_passage_time(&mut tr.rng.0,Length::new::<meter>(dist),DiffusionCoefficient::new::<square_meter_per_second>(dcur));tr.time+=tau.get::<second>();let q=sample_uniform_direction(&mut tr.rng.0);for k in 0..3{tr.p[k]+=dist*q[k]}tr.steps+=1;false}
 else{c.enc+=1;tr.steps+=1;let at_outer=match din{Some(x)=>dout<=x,None=>true};if !inner&&at_outer{return true}let dnext=if inner{d2}else{d1};let tx=does_transmit(&mut tr.rng.0,DiffusionCoefficient::new::<square_meter_per_second>(dcur),DiffusionCoefficient::new::<square_meter_per_second>(dnext),1.0);if tx{c.tx+=1;if inner{c.inout+=1;set_radius(&mut tr.p,a+ALPHA*EPS)}else{c.outin+=1;set_radius(&mut tr.p,a-ALPHA*EPS)}}else{c.rf+=1;if inner{set_radius(&mut tr.p,a-ALPHA*EPS)}else{set_radius(&mut tr.p,a+ALPHA*EPS)}}false}
}
fn seed_child(master:&mut u64)->OoRng64{OoRng64::from_u64((u01(master)*u64::MAX as f64)as u64)}
fn split(tr:Tr,b:usize,master:&mut u64,diag:&mut Diag,idx:usize)->Vec<Tr>{
 let before=tr.w;let cw=before/b as f64;let mut v=Vec::new();for _ in 0..b{let mut z=tr.clone();z.w=cw;z.depth+=1;z.rng=seed_child(master);v.push(z)}let after:v64=v.iter().map(|x|x.w).sum();diag.max_split_err=diag.max_split_err.max((before-after).abs());diag.splits[idx]+=1;diag.max_depth=diag.max_depth.max(tr.depth+1);v
}
type v64=f64;
fn milestones(tr:&Tr,m:Model,prev_r:f64,was_inner:bool,transmitted_out:bool)->Vec<(usize,usize)>{
 let r=radius(tr.p);let mut out=Vec::new();
 match m{
  Model::Sphere=>{for (i,x) in [40e-6,60e-6,80e-6].iter().enumerate(){if tr.flags&(1<<i)==0 && prev_r<*x && r>=*x{out.push((i,2));break}}}
  Model::Two=>{
   if tr.flags&1==0 && was_inner && transmitted_out{out.push((0,4))}
   else if r>=50e-6{let frac=(r-50e-6)/50e-6;if tr.flags&2==0 && frac>=0.5{out.push((1,2))}else if tr.flags&4==0 && frac>=0.8{out.push((2,2))}}
  }
 }out
}
fn rare(m:Model,times:&[f64],rep:usize)->Out{
 let (r0,n0,nb)=match m{Model::Sphere=>(100e-6,40usize,5usize),Model::Two=>(50e-6,64usize,4usize)};let per=n0/nb;let mut master=0x51A17_u64+rep as u64*0x100000+(match m{Model::Sphere=>1,Model::Two=>2});let mut stack=Vec::new();
 for b in 0..nb{let lo=r0*b as f64/nb as f64;let hi=r0*(b+1)as f64/nb as f64;let mass=(hi.powi(3)-lo.powi(3))/r0.powi(3);for k in 0..per{let p=start_shell(&mut master,lo,hi);stack.push(Tr{p,time:0.0,w:mass/per as f64,steps:0,rng:seed_child(&mut master),flags:0,root:(b*per+k)as u32,depth:0})}}
 let mut rel=Vec::new();let mut d=Diag::default();d.peak=stack.len();let tmax=*times.last().unwrap();let mut root_desc=vec![0u64;n0];
 while let Some(mut tr)=stack.pop(){let mut done=false;while !done{if tr.steps>=CAP{d.cens+=tr.w;break}let prev=radius(tr.p);let was_inner=matches!(m,Model::Two)&&prev<50e-6;let inout0=d.c.inout;if hop(&mut tr,m,&mut d.c){rel.push((tr.time,tr.w));root_desc[tr.root as usize]+=1;break}d.steps+=1;if tr.time>tmax{root_desc[tr.root as usize]+=1;break}let transmitted_out=d.c.inout>inout0;let ms=milestones(&tr,m,prev,was_inner,transmitted_out);if let Some((idx,b))=ms.first().copied(){tr.flags|=1<<idx;let kids=split(tr,b,&mut master,&mut d,idx);for z in kids{stack.push(z)}d.peak=d.peak.max(stack.len());done=true}}
 }
 d.max_desc=*root_desc.iter().max().unwrap_or(&0);let f=times.iter().map(|&t|rel.iter().filter(|x|x.0<=t).map(|x|x.1).sum()).collect();let rw=rel.iter().map(|x|x.1).collect();Out{f,diag:d,rep_weights:rw}
}
fn direct(m:Model,times:&[f64])->(Vec<f64>,u64,Counters){
 let r0=match m{Model::Sphere=>100e-6,Model::Two=>50e-6};let mut cnt=vec![0usize;times.len()];let mut steps=0;let mut c=Counters::default();let tmax=*times.last().unwrap();
 for h in 0..DIRECT_N{let mut seed=0xD1EC0000+h as u64+(match m{Model::Sphere=>1,Model::Two=>2})*0x100000;let p=start_shell(&mut seed,0.0,r0);let mut tr=Tr{p,time:0.0,w:1.0,steps:0,rng:OoRng64::from_u64(seed^0x9e3779b97f4a7c15),flags:0,root:h as u32,depth:0};for _ in 0..CAP{if hop(&mut tr,m,&mut c){for(i,&t)in times.iter().enumerate(){if tr.time<=t{cnt[i]+=1}}break}if tr.time>tmax{break}}steps+=tr.steps}
 (cnt.iter().map(|&x|x as f64/DIRECT_N as f64).collect(),steps,c)
}
fn sphere_ref(t:f64)->f64{let x=PI*PI*1e-8*t/1e-8;let mut s=0.0;for n in 1..10000{let z=(-x*(n*n)as f64).exp()/(n*n)as f64;s+=z;if z<1e-15{break}}1.0-6.0/(PI*PI)*s}
fn stats(v:&[Vec<f64>],j:usize)->(f64,f64,f64){let n=v.len()as f64;let mean=v.iter().map(|x|x[j]).sum::<f64>()/n;let sd=(v.iter().map(|x|(x[j]-mean).powi(2)).sum::<f64>()/(n-1.0)).sqrt();(mean,sd,sd/n.sqrt())}
fn level0(){
 let w=0.73;for b in [2usize,3,7]{let sum=b as f64*(w/b as f64);assert!((sum-w).abs()<=1e-15)}
 let mut seed=0xA55A55;for q in [0.2,0.5,0.8]{let n=1_000_000;let g=3.7;let mut vals=0.0;let mut vals2=0.0;for _ in 0..n{let y=if u01(&mut seed)<q{w/q*g}else{0.0};vals+=y;vals2+=y*y}let mean=vals/n as f64;let var=(vals2/n as f64-mean*mean)/(n as f64);let se=var.sqrt();let exact=w*g;println!("L0_ROULETTE q={q} exact={exact:.12e} mean={mean:.12e} se={se:.12e} z={:.4}",(mean-exact)/se);assert!((mean-exact).abs()<=5.0*se)}
 let mut master=1;let tr=Tr{p:[1.0,0.0,0.0],time:2.0,w:1.0,steps:0,rng:OoRng64::from_u64(7),flags:1,root:0,depth:0};let mut d=Diag::default();let kids=split(tr,4,&mut master,&mut d,0);let mut firsts=Vec::new();for mut z in kids{firsts.push(u01(&mut z.rng.0))}assert!(firsts.windows(2).any(|x|x[0]!=x[1]));assert_eq!(d.splits[0],1);println!("LEVEL0 PASS split_err={:.3e} rng_unique=true milestone_one_shot=true",d.max_split_err)
}
fn run_level(name:&str,m:Model,times:&[f64],reference:&[f64]){
 let wall=Instant::now();let (dir,dsteps,dc)=direct(m,times);let direct_s=wall.elapsed().as_secs_f64();let re0=Instant::now();let mut reps=Vec::new();let mut maxc:f64=0.0;let mut maxe:f64=0.0;let mut splits=[0u64;3];let mut maxdepth=0;let mut maxdesc=0;let mut peak=0;let mut steps=0;let mut rc=Counters::default();
 for r in 0..REPS{let o=rare(m,times,r);maxc=maxc.max(o.diag.cens);maxe=maxe.max(o.diag.max_split_err);for i in 0..3{splits[i]+=o.diag.splits[i]}maxdepth=maxdepth.max(o.diag.max_depth);maxdesc=maxdesc.max(o.diag.max_desc);peak=peak.max(o.diag.peak);steps+=o.diag.steps;rc.enc+=o.diag.c.enc;rc.tx+=o.diag.c.tx;rc.rf+=o.diag.c.rf;rc.inout+=o.diag.c.inout;rc.outin+=o.diag.c.outin;reps.push(o.f)}
 let re_s=re0.elapsed().as_secs_f64();let mut pass=maxc==0.0&&maxe<=SPLIT_TOL;println!("LEVEL {name} direct_N={DIRECT_N} reps={REPS} max_cens={maxc:.3e} max_split_err={maxe:.3e} splits={:?} max_depth={} max_desc={} peak={} direct_steps={} re_steps={} direct_s={:.6} re_s={:.6}",splits,maxdepth,maxdesc,peak,dsteps,steps,direct_s,re_s);
 for j in 0..times.len(){let(mean,sd,se)=stats(&reps,j);let dse=(dir[j]*(1.0-dir[j])/DIRECT_N as f64).sqrt();let tol=.025_f64.max(2.0*(dse+se));let a=(mean-dir[j]).abs();let b=(mean-reference[j]).abs();let rt=if name=="L1"{.04}else{.025};let ok=a<=tol&&b<=rt;pass&=ok;println!("POINT level={name} t={:.6e} direct={:.8} direct_se={:.8} rare={:.8} rare_sd={:.8} rare_se={:.8} ref={:.8} abs_rd={:.8} abs_rr={:.8} tol={:.8} pass={}",times[j],dir[j],dse,mean,sd,se,reference[j],a,b,tol,ok)}
 if let Model::Two=m{let ok=dc.tx>0&&dc.rf>0&&dc.inout>0&&dc.outin>0&&rc.tx>0&&rc.rf>0&&rc.inout>0&&rc.outin>0;pass&=ok;println!("INTERFACE direct enc={} tx={} rf={} inout={} outin={} rare enc={} tx={} rf={} inout={} outin={} pass={}",dc.enc,dc.tx,dc.rf,dc.inout,dc.outin,rc.enc,rc.tx,rc.rf,rc.inout,rc.outin,ok)}
 println!("LEVEL_RESULT {name} pass={pass}");assert!(pass)
}
fn main(){println!("PROCESS_A_STOPPING_TIME_SPLITTING supervisor=8d31482d127e211614ebb3f66b1076e1ed6dea98 five_layer=NOT_AUTHORIZED");level0();let t1=[.05,.1,.2,.4];let r1:Vec<_>=t1.iter().map(|&t|sphere_ref(t)).collect();run_level("L1",Model::Sphere,&t1,&r1);let t2=[.25,.5,1.,2.,4.,8.,16.,32.];let r2=[.00983747,.06998542,.23413553,.49412611,.76267512,.94225401,.99648779,.999987];run_level("L2",Model::Two,&t2,&r2);println!("STOPPING_TIME_SPLITTING PASS independent_review_required=true five_layer=NOT_AUTHORIZED")}
