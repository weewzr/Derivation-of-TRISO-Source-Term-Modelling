use std::{f64::consts::PI,time::Instant};

const A:f64=50e-6; const R:f64=100e-6; const D1:f64=1e-10; const D2:f64=1e-9;
fn vol(a:f64,b:f64)->f64{4.0*PI/3.0*(b.powi(3)-a.powi(3))}
fn centroid(a:f64,b:f64)->f64{0.75*(b.powi(4)-a.powi(4))/(b.powi(3)-a.powi(3))}
fn d(r:f64)->f64{if r<A{D1}else{D2}}
struct Mesh{n:usize,v:Vec<f64>,g:Vec<f64>,gr:f64,c:Vec<f64>}
fn mesh(per_layer:usize)->Mesh{
 let mut f=Vec::with_capacity(2*per_layer+1);for j in 0..=per_layer{f.push(A*j as f64/per_layer as f64)}for j in 1..=per_layer{f.push(A+(R-A)*j as f64/per_layer as f64)}
 let n=f.len()-1;let rc:Vec<f64>=(0..n).map(|i|centroid(f[i],f[i+1])).collect();let v:Vec<f64>=(0..n).map(|i|vol(f[i],f[i+1])).collect();let mut g=vec![0.0;n-1];
 for i in 0..n-1{let rf=f[i+1];g[i]=4.0*PI*rf*rf/((rf-rc[i])/d(rc[i])+(rc[i+1]-rf)/d(rc[i+1]));}
 let gr=4.0*PI*R*R*D2/(R-rc[n-1]);let v0=vol(0.0,A);let c=(0..n).map(|i|if f[i+1]<=A*(1.0+1e-12){1.0/v0}else{0.0}).collect();Mesh{n,v,g,gr,c}
}
fn dtmax(m:&Mesh)->f64{let mut z=f64::INFINITY;for i in 0..m.n{let gw=if i==0{0.0}else{m.g[i-1]};let ge=if i==m.n-1{m.gr}else{m.g[i]};z=z.min(m.v[i]/(gw+ge));}z}
fn curve(mut m:Mesh,times:&[f64],q:f64)->(Vec<f64>,f64,f64){
 let req=q*dtmax(&m);let mut out=Vec::new();let mut t=0.0;let mut rel=0.0;let mut k=0;let mut nx=vec![0.0;m.n];let mut max_cons=0.0_f64;
 while k<times.len(){if t>=times[k]-1e-15{out.push(rel);k+=1;continue}let h=req.min(times[k]-t);let old:f64=m.c.iter().zip(&m.v).map(|(c,v)|c*v).sum();let qr=m.gr*m.c[m.n-1];
 for i in 0..m.n{let gw=if i==0{0.0}else{m.g[i-1]};let ge=if i==m.n-1{m.gr}else{m.g[i]};let w=if i==0{0.0}else{m.c[i-1]};let e=if i==m.n-1{0.0}else{m.c[i+1]};nx[i]=m.c[i]+h/m.v[i]*(gw*w-(gw+ge)*m.c[i]+if i==m.n-1{0.0}else{ge*e});}
 let new:f64=nx.iter().zip(&m.v).map(|(c,v)|c*v).sum();rel+=h*qr;max_cons=max_cons.max((new-old+h*qr).abs());std::mem::swap(&mut m.c,&mut nx);t+=h;} (out,req,max_cons)
}
fn main(){
 let times=[0.25,0.5,1.0,2.0,4.0,8.0,16.0,32.0];
 println!("TWO_LAYER_FV_REFINEMENT a_m={A:.12e} R_m={R:.12e} D1={D1:.12e} D2={D2:.12e} times_s={times:?}");
 let start=Instant::now();let mut prev:Option<Vec<f64>>=None;
 for npl in [10usize,20,40,80]{
  let (f,dt,cons)=curve(mesh(npl),&times,0.8);
  let diff=prev.as_ref().map(|p|p.iter().zip(&f).map(|(a,b)|(a-b).abs()).fold(0.0_f64,f64::max));
  println!("spatial cells_per_layer={npl} total_cells={} dt_s={dt:.12e} max_inventory_residual={cons:.3e} F={f:?} max_change_from_prev={}",2*npl,diff.map(|x|format!("{x:.8e}")).unwrap_or("-".into()));prev=Some(f);
 }
 let m=mesh(80);let mut prev_t:Option<Vec<f64>>=None;
 for q in [0.8,0.4,0.2]{
  let (f,dt,cons)=curve(mesh(80),&times,q);let diff=prev_t.as_ref().map(|p|p.iter().zip(&f).map(|(a,b)|(a-b).abs()).fold(0.0_f64,f64::max));
  println!("temporal q={q:.3} dt_s={dt:.12e} max_inventory_residual={cons:.3e} F={f:?} max_change_from_prev={}",diff.map(|x|format!("{x:.8e}")).unwrap_or("-".into()));prev_t=Some(f);
 }
 println!("elapsed_s={:.3}",start.elapsed().as_secs_f64());
}