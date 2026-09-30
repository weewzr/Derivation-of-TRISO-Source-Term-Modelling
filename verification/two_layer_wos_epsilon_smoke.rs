use std::time::Instant;
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::{
 central_limit_theorem::oorandom_rng::OoRng64,
 first_passage::{
  interface::does_transmit,
  sphere_fpt::{sample_first_passage_time,sample_uniform_direction},
 },
};
use uom::si::{diffusion_coefficient::square_meter_per_second,f64::{DiffusionCoefficient,Length},length::{meter,nanometer},time::second};

const A:f64=50e-6; const R:f64=100e-6; const D1:f64=1e-10; const D2:f64=1e-9; const ALPHA:f64=2.0; const K:f64=1.0;
const EPS_NM:[f64;4]=[200.0,100.0,50.0,25.0]; const N:usize=32; const CAP:u64=1_000_000;

fn radius(p:[Length;3])->f64{let x=p[0].get::<meter>();let y=p[1].get::<meter>();let z=p[2].get::<meter>();(x*x+y*y+z*z).sqrt()}
fn set_radius(p:&mut[Length;3],target:f64){let rho=radius(*p);if rho==0.0{*p=[Length::new::<meter>(target),Length::new::<meter>(0.0),Length::new::<meter>(0.0)];return}let s=target/rho;*p=[p[0]*s,p[1]*s,p[2]*s]}
fn start_inner(seed:&mut u64)->[Length;3]{let u=outram_mc_libs::rng::lcg::prn(seed);let rr=A*u.cbrt();let q=sample_uniform_direction(seed);[Length::new::<meter>(rr*q[0]),Length::new::<meter>(rr*q[1]),Length::new::<meter>(rr*q[2])]}

fn main(){
 println!("TWO_LAYER_WOS_SMOKE a_m={A:.12e} R_m={R:.12e} D1={D1:.12e} D2={D2:.12e} N={N} cap={CAP} alpha={ALPHA} K={K} eps_nm={EPS_NM:?}");
 for (ei,eps_nm) in EPS_NM.iter().enumerate(){
  let eps=eps_nm*1e-9;let re=ALPHA*eps;let wall=Instant::now();let mut released=0usize;let mut censored=0usize;let mut total_steps=0u64;let mut max_steps=0u64;
  let mut encounters=0u64;let mut transmitted=0u64;let mut reflected=0u64;let mut in_to_out=0u64;let mut out_to_in=0u64;let mut zero_dt=0u64;let mut releases=Vec::new();
  for h in 0..N{
   let mut rng=OoRng64::from_u64(0x2A00_0000+(ei as u64)*0x10000+h as u64);let mut p=start_inner(&mut rng.0);let mut time=0.0_f64;let mut done=false;let mut used=0u64;
   for step in 1..=CAP{
    used=step;let rho=radius(p);let inner=rho<A;let (rin,rout,dcur)=if inner{(None,A,D1)}else{(Some(A),R,D2)};let dout=rout-rho;let din=rin.map(|x|rho-x);let hop=match din{Some(x) if x<dout=>x,_=>dout};
    if hop>eps{
     let tau=sample_first_passage_time(&mut rng.0,Length::new::<meter>(hop),DiffusionCoefficient::new::<square_meter_per_second>(dcur));time+=tau.get::<second>();let q=sample_uniform_direction(&mut rng.0);p=[p[0]+Length::new::<meter>(hop*q[0]),p[1]+Length::new::<meter>(hop*q[1]),p[2]+Length::new::<meter>(hop*q[2])];
    }else{
     encounters+=1;zero_dt+=1;let at_outer=match din{Some(x)=>dout<=x,None=>true};
     if !inner && at_outer{released+=1;releases.push(time);done=true;break}
     let target_outer=if inner{true}else{false};let dnext=if target_outer{D2}else{D1};let tx=does_transmit(&mut rng.0,DiffusionCoefficient::new::<square_meter_per_second>(dcur),DiffusionCoefficient::new::<square_meter_per_second>(dnext),K);
     if tx{transmitted+=1;if inner{in_to_out+=1;set_radius(&mut p,A+re)}else{out_to_in+=1;set_radius(&mut p,A-re)}}else{reflected+=1;if inner{set_radius(&mut p,A-re)}else{set_radius(&mut p,A+re)}}
    }
   }
   if !done{censored+=1} total_steps+=used;max_steps=max_steps.max(used);
   if (h+1)%8==0{println!("eps_nm={eps_nm:.1} progress={} released={} censored={} elapsed_s={:.3} mean_steps={:.1}",h+1,released,censored,wall.elapsed().as_secs_f64(),total_steps as f64/(h+1)as f64)}
  }
  println!("eps_nm={eps_nm:.1} summary released={released} censored={censored} censor_fraction={:.8} elapsed_s={:.3} mean_steps={:.1} max_steps={} interface_encounters={} transmitted={} reflected={} inner_to_outer={} outer_to_inner={} reinsertion_events={} zero_dt_events={}",censored as f64/N as f64,wall.elapsed().as_secs_f64(),total_steps as f64/N as f64,max_steps,encounters,transmitted,reflected,in_to_out,out_to_in,transmitted+reflected,zero_dt);
  if !releases.is_empty(){let mn=releases.iter().copied().fold(f64::INFINITY,f64::min);let mx=releases.iter().copied().fold(0.0_f64,f64::max);let av=releases.iter().sum::<f64>()/releases.len()as f64;println!("eps_nm={eps_nm:.1} release_time_s min={mn:.12e} mean={av:.12e} max={mx:.12e}")}
 }
}