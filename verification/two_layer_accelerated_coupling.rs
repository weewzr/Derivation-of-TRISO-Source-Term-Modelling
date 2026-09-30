use std::f64::consts::PI;
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::{
 central_limit_theorem::oorandom_rng::OoRng64,
 first_passage::{interface::does_transmit,sphere_fpt::sample_uniform_direction},
};
use outram_mc_libs::rng::lcg::prn;
use uom::si::{diffusion_coefficient::square_meter_per_second,f64::DiffusionCoefficient};

const A:f64=50e-6;const R:f64=100e-6;const D1:f64=1e-10;const D2:f64=1e-9;
const EPS:f64=100e-9;const ALPHA:f64=2.0;const K:f64=1.0;const N:usize=10_000;const TERMS:usize=2_000;
const TIMES:[f64;8]=[0.25,0.5,1.0,2.0,4.0,8.0,16.0,32.0];

fn shell_exit(seed:&mut u64,a:f64,b:f64,r:f64,d:f64)->(bool,f64){
 let l=b-a;let x=r-a;let p=b*x/(r*l);let outer=prn(seed)<p;let y=if outer{x}else{b-r};let q=(y/l).powi(2);let scale=l*l/(d*PI*PI);let mut t=0.0;
 for n in 1..=TERMS{if prn(seed)>=q{let u=prn(seed).max(f64::MIN_POSITIVE);t+=-u.ln()*scale/(n as f64).powi(2)}}(outer,t)
}
fn ball_radius(seed:&mut u64,r:f64)->f64{r*prn(seed).cbrt()}
fn ball_exit_time(seed:&mut u64,r:f64,b:f64,d:f64)->f64{
 let q=(r/b).powi(2);let scale=b*b/(d*PI*PI);let mut t=0.0;
 for n in 1..=TERMS{if prn(seed)>=q{let u=prn(seed).max(f64::MIN_POSITIVE);t+=-u.ln()*scale/(n as f64).powi(2)}}t
}
fn main(){
 println!("TWO_LAYER_ACCELERATED_COUPLING N={N} eps_nm=100 alpha={ALPHA} K={K} terms={TERMS} a={A:.12e} R={R:.12e} D1={D1:.12e} D2={D2:.12e} times={TIMES:?}");
 let mut releases=Vec::with_capacity(N);let mut interface_events=0u64;let mut tx=0u64;let mut refl=0u64;let mut renewal_events=0u64;
 for h in 0..N{
  let mut rng=OoRng64::from_u64(0xACC3_0000+h as u64);let mut r=ball_radius(&mut rng.0,A);let mut inner=true;let mut time=0.0;
  for _ in 0..1_000_000u64{
   renewal_events+=1;
   if inner{
    // Exact centered-ball first exit to radius A, preserving physical first-passage time.
    time+=ball_exit_time(&mut rng.0,r,A,D1);r=A-EPS;
    interface_events+=1;
    let ok=does_transmit(&mut rng.0,DiffusionCoefficient::new::<square_meter_per_second>(D1),DiffusionCoefficient::new::<square_meter_per_second>(D2),K);
    if ok{tx+=1;inner=false;r=A+ALPHA*EPS}else{refl+=1;r=A-ALPHA*EPS}
   }else{
    let (outer,dt)=shell_exit(&mut rng.0,A,R,r,D2);time+=dt;
    if outer{releases.push(time);break}
    interface_events+=1;
    let ok=does_transmit(&mut rng.0,DiffusionCoefficient::new::<square_meter_per_second>(D2),DiffusionCoefficient::new::<square_meter_per_second>(D1),K);
    if ok{tx+=1;inner=true;r=A-ALPHA*EPS}else{refl+=1;r=A+ALPHA*EPS}
   }
  }
  if (h+1)%2000==0{println!("progress={} released={} interface_events={} renewals={}",h+1,releases.len(),interface_events,renewal_events)}
 }
 println!("summary released={} censored={} interface_events={} transmitted={} reflected={} renewal_events={} mean_renewals={:.3}",releases.len(),N-releases.len(),interface_events,tx,refl,renewal_events,renewal_events as f64/N as f64);
 for &t in &TIMES{let k=releases.iter().filter(|&&x|x<=t).count();let f=k as f64/N as f64;let se=(f*(1.0-f)/N as f64).sqrt();println!("t_s={t:.6} released_by_t={k} F_ACCEL={f:.8} SE={se:.8}")}
}