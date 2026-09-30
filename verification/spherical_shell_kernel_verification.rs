use std::f64::consts::PI;
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::{
 central_limit_theorem::oorandom_rng::OoRng64,
 first_passage::sphere_fpt::{sample_first_passage_time,sample_uniform_direction},
};
use outram_mc_libs::rng::lcg::prn;
use uom::si::{diffusion_coefficient::square_meter_per_second,f64::{DiffusionCoefficient,Length},length::meter,time::second};

const A:f64=50e-6; const B:f64=100e-6; const R0:f64=75e-6; const D:f64=1e-8;
const N:usize=20_000; const TERMS:usize=2_000; const EPS:f64=50e-9;

fn exact_p_outer()->f64{B*(R0-A)/(R0*(B-A))}
fn exact_cond_mean(x:f64)->f64{((B-A).powi(2)-x.powi(2))/(6.0*D)}

fn sample_conditional(seed:&mut u64,x:f64)->f64{
 let l=B-A;let q=(x/l).powi(2);let scale=l*l/(D*PI*PI);let mut t=0.0;
 for n in 1..=TERMS{
  if prn(seed)>=q{let u=prn(seed).max(f64::MIN_POSITIVE);t+=-u.ln()*scale/(n as f64).powi(2);}
 }
 t
}

fn direct_shell(seed:u64)->(bool,f64){
 let mut rng=OoRng64::from_u64(seed);let mut p=[Length::new::<meter>(R0),Length::new::<meter>(0.0),Length::new::<meter>(0.0)];let mut t=0.0;
 let diff=DiffusionCoefficient::new::<square_meter_per_second>(D);
 for _ in 0..1_000_000u64{
  let r=(p[0]*p[0]+p[1]*p[1]+p[2]*p[2]).sqrt().get::<meter>();let di=r-A;let do_=B-r;let hop=di.min(do_);
  if hop<=EPS{return (do_<=di,t)}
  t+=sample_first_passage_time(&mut rng.0,Length::new::<meter>(hop),diff).get::<second>();
  let u=sample_uniform_direction(&mut rng.0);p=[p[0]+Length::new::<meter>(hop*u[0]),p[1]+Length::new::<meter>(hop*u[1]),p[2]+Length::new::<meter>(hop*u[2])];
 }
 panic!("direct shell exceeded cap")
}

fn main(){
 let p=exact_p_outer();let mo=exact_cond_mean(R0-A);let mi=exact_cond_mean(B-R0);
 let l=B-A;let q=((R0-A)/l).powi(2);let tail_bound=(1.0-q)*l*l/(D*PI*PI*TERMS as f64);
 println!("SHELL_KERNEL_VERIFY a={A:.12e} b={B:.12e} r0={R0:.12e} D={D:.12e} N={N} terms={TERMS} eps_direct={EPS:.12e}");
 println!("exact p_outer={p:.12e} mean_outer_s={mo:.12e} mean_inner_s={mi:.12e} truncation_mean_tail_bound_s={tail_bound:.12e}");
 let mut seed=0x51E1_0000_u64;let mut ko=0usize;let mut so=0.0;let mut si=0.0;let mut no=0usize;let mut ni=0usize;
 for _ in 0..N{let outer=prn(&mut seed)<p;let x=if outer{R0-A}else{B-R0};let t=sample_conditional(&mut seed,x);if outer{ko+=1;no+=1;so+=t}else{ni+=1;si+=t}}
 println!("accelerated p_outer={:.12e} mean_outer_s={:.12e} mean_inner_s={:.12e}",ko as f64/N as f64,so/no as f64,si/ni as f64);
 let mut dko=0usize;let mut dso=0.0;let mut dsi=0.0;let mut dno=0usize;let mut dni=0usize;
 for i in 0..N{let(o,t)=direct_shell(0xD1CE_0000+i as u64);if o{dko+=1;dno+=1;dso+=t}else{dni+=1;dsi+=t}}
 println!("direct_wos p_outer={:.12e} mean_outer_s={:.12e} mean_inner_s={:.12e}",dko as f64/N as f64,dso/dno as f64,dsi/dni as f64);
 println!("errors accelerated dp={:.6e} dmean_outer={:.6e} dmean_inner={:.6e}",ko as f64/N as f64-p,so/no as f64-mo,si/ni as f64-mi);
 println!("errors direct dp={:.6e} dmean_outer={:.6e} dmean_inner={:.6e}",dko as f64/N as f64-p,dso/dno as f64-mo,dsi/dni as f64-mi);
}