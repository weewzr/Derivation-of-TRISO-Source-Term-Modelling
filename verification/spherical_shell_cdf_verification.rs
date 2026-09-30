use std::f64::consts::PI;
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::{
 central_limit_theorem::oorandom_rng::OoRng64,
 first_passage::sphere_fpt::{sample_first_passage_time,sample_uniform_direction},
};
use outram_mc_libs::rng::lcg::prn;
use uom::si::{diffusion_coefficient::square_meter_per_second,f64::{DiffusionCoefficient,Length},length::meter,time::second};

const A:f64=50e-6;const B:f64=100e-6;const R0:f64=75e-6;const D:f64=1e-8;
const N:usize=20_000;const TERMS:usize=2_000;const EPS:f64=50e-9;
const TIMES:[f64;7]=[0.005,0.01,0.02,0.03,0.05,0.08,0.12];

fn p_outer()->f64{B*(R0-A)/(R0*(B-A))}
fn sample_cond(seed:&mut u64,x:f64)->f64{let l=B-A;let q=(x/l).powi(2);let scale=l*l/(D*PI*PI);let mut t=0.0;for n in 1..=TERMS{if prn(seed)>=q{let u=prn(seed).max(f64::MIN_POSITIVE);t+=-u.ln()*scale/(n as f64).powi(2)}}t}
fn direct(seed:u64)->(bool,f64){let mut rng=OoRng64::from_u64(seed);let mut p=[Length::new::<meter>(R0),Length::new::<meter>(0.0),Length::new::<meter>(0.0)];let mut t=0.0;let diff=DiffusionCoefficient::new::<square_meter_per_second>(D);for _ in 0..1_000_000u64{let r=(p[0]*p[0]+p[1]*p[1]+p[2]*p[2]).sqrt().get::<meter>();let di=r-A;let do_=B-r;let h=di.min(do_);if h<=EPS{return(do_<=di,t)}t+=sample_first_passage_time(&mut rng.0,Length::new::<meter>(h),diff).get::<second>();let u=sample_uniform_direction(&mut rng.0);p=[p[0]+Length::new::<meter>(h*u[0]),p[1]+Length::new::<meter>(h*u[1]),p[2]+Length::new::<meter>(h*u[2])]}panic!("cap")}
fn analytic_cond_cdf(t:f64)->f64{
 // Symmetric x=L/2 case. Conditional LT = 1/cosh((L/2)*sqrt(s/D)).
 // Inverse transform survival: S(t)=4/pi sum_{k>=0} (-1)^k/(2k+1) exp[-D*pi^2(2k+1)^2*t/L^2].
 let l=B-A;let mut s=0.0;for k in 0..10_000usize{let m=(2*k+1)as f64;let term=4.0/PI*(if k%2==0{1.0}else{-1.0})/m*(-D*PI*PI*m*m*t/(l*l)).exp();s+=term;if term.abs()<1e-15{break}}(1.0-s).clamp(0.0,1.0)
}
fn main(){
 println!("SHELL_CDF_VERIFY N={N} terms={TERMS} times_s={TIMES:?}");
 let mut seed=0xCDF0_0000u64;let mut ao=Vec::new();let mut ai=Vec::new();
 for _ in 0..N{let o=prn(&mut seed)<p_outer();let t=sample_cond(&mut seed,if o{R0-A}else{B-R0});if o{ao.push(t)}else{ai.push(t)}}
 let mut do_=Vec::new();let mut di=Vec::new();for i in 0..N{let(o,t)=direct(0xD1CE_0000+i as u64);if o{do_.push(t)}else{di.push(t)}}
 println!("counts accelerated_outer={} accelerated_inner={} direct_outer={} direct_inner={}",ao.len(),ai.len(),do_.len(),di.len());
 for &t in &TIMES{let exact=analytic_cond_cdf(t);let c=|v:&Vec<f64>|v.iter().filter(|&&x|x<=t).count()as f64/v.len()as f64;let aoc=c(&ao);let aic=c(&ai);let doc=c(&do_);let dic=c(&di);println!("t_s={t:.6} exact={exact:.8} accelerated_outer={aoc:.8} accelerated_inner={aic:.8} direct_outer={doc:.8} direct_inner={dic:.8} da_outer={:.8} dd_outer={:.8}",aoc-exact,doc-exact)}
}