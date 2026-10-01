use std::f64::consts::PI;
use boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::{central_limit_theorem::oorandom_rng::OoRng64,first_passage::interface::does_transmit};
use outram_mc_libs::rng::lcg::prn;
use uom::si::{diffusion_coefficient::square_meter_per_second,f64::DiffusionCoefficient};

const A:f64=50e-6;const R:f64=100e-6;const D1:f64=1e-10;const D2:f64=1e-9;const EPS:f64=100e-9;const ALPHA:f64=2.0;const KPART:f64=1.0;
const N:usize=20_000;const TERMS:usize=2000;const S:[f64;6]=[0.0,0.25,0.5,1.0,2.0,4.0];

fn sinh_ratio(x:f64,y:f64)->f64{if y.abs()<1e-7{x/y}else if y>700.0{((x-y).exp())}else{x.sinh()/y.sinh()}}
fn ball_lt(r:f64,b:f64,d:f64,s:f64)->f64{if s==0.0{return 1.0}let l=(s/d).sqrt();if r==0.0{let z=l*b;if z>700.0{return 0.0}return z/z.sinh()}(b/r)*sinh_ratio(l*r,l*b)}
fn shell_lt(a:f64,b:f64,r:f64,d:f64,s:f64)->(f64,f64){if s==0.0{return(a*(b-r)/(r*(b-a)),b*(r-a)/(r*(b-a)))}let l=(s/d).sqrt();let den=l*(b-a);let gm=(a/r)*sinh_ratio(l*(b-r),den);let gp=(b/r)*sinh_ratio(l*(r-a),den);(gm,gp)}
fn solve2(s:f64)->([f64;2],f64,f64){
 let delta=ALPHA*EPS;let h=ball_lt(A-delta,A,D1,s);let(gm,gp)=shell_lt(A,R,A+delta,D2,s);
 let p00=D1/(D1+D2);let p01=D2/(D1+D2);let p10=p00;let p11=p01;
 let a00=1.0-h*p00;let a01=-h*p01;let a10=-gm*p10;let a11=1.0-gm*p11;let det=a00*a11-a01*a10;
 let x0=(-a01*gp)/det;let x1=(a00*gp)/det;
 let r0=a00*x0+a01*x1;let r1=a10*x0+a11*x1-gp;let resid=(r0*r0+r1*r1).sqrt().max(0.0);
 // exact 2-norm condition number from eigenvalues of A^T A
 let tr=a00*a00+a01*a01+a10*a10+a11*a11;let detata=det*det;let disc=(tr*tr-4.0*detata).max(0.0).sqrt();let lmax=(tr+disc)/2.0;let lmin=(tr-disc)/2.0;let cond=(lmax/lmin).sqrt();
 ([x0,x1],cond,resid)
}
fn integrate_init(s:f64,phi:[f64;2])->f64{
 let p00=D1/(D1+D2);let p01=D2/(D1+D2);let q=p00*phi[0]+p01*phi[1];
 if s==0.0{return q}
 let n=20000usize;let dr=A/n as f64;let mut sum=0.0;for i in 0..n{let r=(i as f64+0.5)*dr;sum+=3.0*r*r/(A*A*A)*ball_lt(r,A,D1,s)}sum*dr*q
}
fn sample_time(seed:&mut u64,x:f64,l:f64,d:f64)->f64{let q=(x/l).powi(2);let sc=l*l/(d*PI*PI);let mut t=0.0;for n in 1..=TERMS{if prn(seed)>=q{let u=prn(seed).max(f64::MIN_POSITIVE);t+=-u.ln()*sc/(n as f64).powi(2)}}t}
fn explicit(seed0:u64)->f64{
 let mut rng=OoRng64::from_u64(seed0);let mut r=A*prn(&mut rng.0).cbrt();let mut inner=true;let mut time=0.0;let delta=ALPHA*EPS;
 for _ in 0..1_000_000u64{
  if inner{time+=sample_time(&mut rng.0,r,A,D1);let ok=does_transmit(&mut rng.0,DiffusionCoefficient::new::<square_meter_per_second>(D1),DiffusionCoefficient::new::<square_meter_per_second>(D2),KPART);if ok{inner=false;r=A+delta}else{r=A-delta}}
  else{let l=R-A;let x=r-A;let po=R*x/(r*l);let out=prn(&mut rng.0)<po;let y=if out{x}else{R-r};time+=sample_time(&mut rng.0,y,l,D2);if out{return time}let ok=does_transmit(&mut rng.0,DiffusionCoefficient::new::<square_meter_per_second>(D2),DiffusionCoefficient::new::<square_meter_per_second>(D1),KPART);if ok{inner=true;r=A-delta}else{r=A+delta}}
 }
 panic!("explicit renewal cap")
}
fn main(){
 let capmass=1.0-(1.0-EPS/A).powi(3);println!("TWO_LAYER_MATRIX_RECON N={N} s={S:?} capture_mass={capmass:.12e}");
 let times:Vec<f64>=(0..N).map(|i|explicit(0xA8C0_0000+i as u64)).collect();
 for &s in &S{let(phi,cond,resid)=solve2(s);let init=integrate_init(s,phi);let vals:Vec<f64>=times.iter().map(|&t|(-s*t).exp()).collect();let mean=vals.iter().sum::<f64>()/N as f64;let var=vals.iter().map(|x|(x-mean)*(x-mean)).sum::<f64>()/(N-1)as f64;let se=(var/N as f64).sqrt();println!("s={s:.6} phi0={:.12e} phi1={:.12e} phi_init_matrix={init:.12e} explicit_mean={mean:.12e} explicit_se={se:.12e} delta={:.12e} z={:.6} cond2={cond:.12e} residual={resid:.12e}",phi[0],phi[1],mean-init,if se>0.0{(mean-init)/se}else{0.0});}
}