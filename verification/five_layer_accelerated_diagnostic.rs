use std::{f64::consts::PI,time::Instant};
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::{
 central_limit_theorem::oorandom_rng::OoRng64,
 first_passage::interface::does_transmit,
};
use outram_mc_libs::rng::lcg::prn;
use uom::si::{diffusion_coefficient::square_meter_per_second,f64::DiffusionCoefficient};

const N:usize=16;const CAP:u64=100_000;const TERMS:usize=2000;const EPS:f64=100e-9;const ALPHA:f64=2.0;const K:f64=1.0;
const RAD:[f64;5]=[2.125e-4,3.125e-4,3.525e-4,3.875e-4,4.275e-4];
const D:[f64;5]=[1.2502982636347968e-13,1.0e-8,4.062299125614697e-14,9.227773168241615e-17,4.062299125614697e-14];

fn lname(i:usize)->&'static str{["Kernel","Buffer","IPyC","SiC","OPyC"][i]}
fn ball_time(seed:&mut u64,r:f64,b:f64,d:f64)->f64{let q=(r/b).powi(2);let sc=b*b/(d*PI*PI);let mut t=0.0;for n in 1..=TERMS{if prn(seed)>=q{let u=prn(seed).max(f64::MIN_POSITIVE);t+=-u.ln()*sc/(n as f64).powi(2)}}t}
fn shell(seed:&mut u64,a:f64,b:f64,r:f64,d:f64)->(bool,f64){let l=b-a;let x=r-a;let po=b*x/(r*l);let outer=prn(seed)<po;let y=if outer{x}else{b-r};let q=(y/l).powi(2);let sc=l*l/(d*PI*PI);let mut t=0.0;for n in 1..=TERMS{if prn(seed)>=q{let u=prn(seed).max(f64::MIN_POSITIVE);t+=-u.ln()*sc/(n as f64).powi(2)}}(outer,t)}
fn reinsert(layer:usize,iface:usize)->f64{let delta=ALPHA*EPS;if layer==iface{RAD[iface]-delta}else{RAD[iface]+delta}}
fn main(){
 println!("FIVE_LAYER_ACCEL_DIAGNOSTIC N={N} cap={CAP} terms={TERMS} eps_nm=100 alpha={ALPHA} K={K} radii={RAD:?} D={D:?}");
 let wall=Instant::now();let mut released=0usize;let mut cens=0usize;let mut errors=0usize;let mut total=0u64;let mut agg_layer=[0u64;5];let mut enc=[0u64;4];let mut tx=[0u64;4];let mut rf=[0u64;4];let mut deepest_hist=[0u64;5];let mut release_times=Vec::new();
 for h in 0..N{
  let mut rng=OoRng64::from_u64(0xF1A5_0000+h as u64);let r0=RAD[0]*prn(&mut rng.0).cbrt();let mut r=r0;let mut layer=0usize;let mut time=0.0;let mut deepest=0usize;let mut per=[0u64;5];let mut term="CensoredMaxRenewals";let mut used=0u64;let mut bad=false;
  for k in 1..=CAP{
   used=k;per[layer]+=1;agg_layer[layer]+=1;deepest=deepest.max(layer);
   let before=layer;
   let (outer,dt)=if layer==0{(true,ball_time(&mut rng.0,r,RAD[0],D[0]))}else{shell(&mut rng.0,RAD[layer-1],RAD[layer],r,D[layer])};
   if !dt.is_finite()||dt<0.0{term="ErrorInvalidTime";errors+=1;bad=true;break}
   time+=dt;
   if h<2 && k<=20{println!("trace h={h} renewal={k} layer={} r={r:.12e} exit={} dt_s={dt:.12e} cumulative_s={time:.12e}",lname(before),if outer{"outer"}else{"inner"});}
   if layer==4 && outer{released+=1;release_times.push(time);term="Released";break}
   let iface=if outer{layer}else{layer-1};enc[iface]+=1;
   let next=if outer{layer+1}else{layer-1};
   let ok=does_transmit(&mut rng.0,DiffusionCoefficient::new::<square_meter_per_second>(D[layer]),DiffusionCoefficient::new::<square_meter_per_second>(D[next]),K);
   if ok{tx[iface]+=1;layer=next}else{rf[iface]+=1}
   r=reinsert(layer,iface);
   if h<2 && k<=20{println!("trace h={h} interface={iface} outcome={} from={} to={} reinsert_r={r:.12e} interface_dt=0",if ok{"transmit"}else{"reflect"},lname(before),lname(layer));}
  }
  if term=="CensoredMaxRenewals"{cens+=1} deepest_hist[deepest]+=1;total+=used;
  println!("history={h} initial_r={r0:.12e} termination={term} renewals={used} final_layer={} deepest={} physical_time_s={time:.12e} renewals_by_layer={per:?}",lname(layer),lname(deepest));
  if bad{continue}
 }
 println!("summary released={released} censored={cens} errors={errors} mean_renewals={:.3} renewals_by_layer={agg_layer:?} interface_encounters={enc:?} transmitted={tx:?} reflected={rf:?} deepest_distribution={deepest_hist:?} wall_s={:.3}",total as f64/N as f64,wall.elapsed().as_secs_f64());
 if !release_times.is_empty(){let mn=release_times.iter().copied().fold(f64::INFINITY,f64::min);let mx=release_times.iter().copied().fold(0.0_f64,f64::max);let av=release_times.iter().sum::<f64>()/release_times.len()as f64;println!("release_time_s min={mn:.12e} mean={av:.12e} max={mx:.12e}");}
}