// Five-layer conservative finite-volume convergence driver.
// Standalone Rust; no external crates. Generated from canonical TRISO-FV equations.
use std::f64::consts::PI;

const R:[f64;6]=[0.0,1.0,2.0,3.0,4.0,5.0];
const D:[f64;5]=[1.0,0.5,2.0,0.25,1.5];
const S0:f64=1.0;
const H:f64=0.8;

fn layer(r:f64)->usize{for i in 0..5{if r<=R[i+1]+1e-13{return i}}4}
fn vol(a:f64,b:f64)->f64{4.0*PI/3.0*(b.powi(3)-a.powi(3))}
fn centroid(a:f64,b:f64)->f64{0.75*(b.powi(4)-a.powi(4))/(b.powi(3)-a.powi(3))}
fn generation()->f64{4.0*PI*S0*R[1].powi(3)/3.0}

fn exact_steady(r:f64)->f64{
 let q=generation(); let surf=q/(4.0*PI*R[5]*R[5]*H); let mut ci=[0.0;6]; ci[5]=surf;
 for i in (1..5).rev(){ci[i]=ci[i+1]+q/(4.0*PI*D[i])*(1.0/R[i]-1.0/R[i+1]);}
 if r<=R[1]{ci[1]+S0/(6.0*D[0])*(R[1]*R[1]-r*r)}
 else{let i=layer(r);ci[i+1]+q/(4.0*PI*D[i])*(1.0/r-1.0/R[i+1])}
}
fn exact_avg(a:f64,b:f64)->f64{
 const X:[f64;8]=[-0.9602898564975363,-0.7966664774136267,-0.5255324099163290,-0.1834346424956498,0.1834346424956498,0.5255324099163290,0.7966664774136267,0.9602898564975363];
 const W:[f64;8]=[0.1012285362903763,0.2223810344533745,0.3137066458778873,0.3626837833783620,0.3626837833783620,0.3137066458778873,0.2223810344533745,0.1012285362903763];
 let m=0.5*(a+b);let hh=0.5*(b-a);let mut z=0.0;
 for k in 0..8{let r=m+hh*X[k];z+=W[k]*exact_steady(r)*4.0*PI*r*r*hh;}z/vol(a,b)
}
struct Mesh{n:usize,faces:Vec<f64>,v:Vec<f64>,g:Vec<f64>,gr:f64,src:Vec<f64>}
fn mesh(n:usize)->Mesh{
 assert!(n%5==0);let dr=R[5]/n as f64;let faces:Vec<f64>=(0..=n).map(|i|i as f64*dr).collect();
 let rc:Vec<f64>=(0..n).map(|i|centroid(faces[i],faces[i+1])).collect();
 let v:Vec<f64>=(0..n).map(|i|vol(faces[i],faces[i+1])).collect();let mut g=vec![0.0;n-1];
 for i in 0..n-1{let rf=faces[i+1];let dl=rf-rc[i];let de=rc[i+1]-rf;let dp=D[layer(rc[i])];let dn=D[layer(rc[i+1])];g[i]=4.0*PI*rf*rf/(dl/dp+de/dn);}
 let delta=R[5]-rc[n-1];let heff=1.0/(delta/D[4]+1.0/H);let gr=4.0*PI*R[5]*R[5]*heff;
 let src=rc.iter().map(|&x|if x<R[1]{S0}else{0.0}).collect();Mesh{n,faces,v,g,gr,src}
}
fn steady(m:&Mesh)->Vec<f64>{
 let n=m.n;let mut a=vec![0.0;n];let mut b=vec![0.0;n];let mut c=vec![0.0;n];let mut rhs=vec![0.0;n];
 for i in 0..n{let gw=if i==0{0.0}else{m.g[i-1]};let ge=if i==n-1{m.gr}else{m.g[i]};a[i]=if i==0{0.0}else{-gw};b[i]=gw+ge;c[i]=if i==n-1{0.0}else{-ge};rhs[i]=m.src[i]*m.v[i];}
 for i in 1..n{let w=a[i]/b[i-1];b[i]-=w*c[i-1];rhs[i]-=w*rhs[i-1];}
 let mut x=vec![0.0;n];x[n-1]=rhs[n-1]/b[n-1];for i in (0..n-1).rev(){x[i]=(rhs[i]-c[i]*x[i+1])/b[i];}x
}
fn vnorm(m:&Mesh,x:&[f64])->f64{m.v.iter().zip(x).map(|(v,e)|v*e*e).sum::<f64>().sqrt()}
fn spatial(n:usize)->(f64,f64,f64){
 let m=mesh(n);let x=steady(&m);let err:Vec<f64>=x.iter().enumerate().map(|(i,&z)|z-exact_avg(m.faces[i],m.faces[i+1])).collect();
 let ev=vnorm(&m,&err);let gen:f64=m.src.iter().zip(&m.v).map(|(s,v)|s*v).sum();let rel=m.gr*x[n-1];(ev,rel,(gen-rel).abs())
}
fn dtmax(m:&Mesh)->f64{let mut z=f64::INFINITY;for i in 0..m.n{let gw=if i==0{0.0}else{m.g[i-1]};let ge=if i==m.n-1{m.gr}else{m.g[i]};z=z.min(m.v[i]/(gw+ge));}z}
fn integrate(m:&Mesh,req:f64,tend:f64)->(f64,Vec<f64>,f64){
 let steps=(tend/req).ceil() as usize;let dt=tend/steps as f64;assert!(dt<=dtmax(m)*(1.0+1e-12));let mut x=vec![0.0;m.n];let mut nx=vec![0.0;m.n];let gen:f64=m.src.iter().zip(&m.v).map(|(s,v)|s*v).sum();let mut max_cons=0.0_f64;
 for _ in 0..steps{let old_inv:f64=x.iter().zip(&m.v).map(|(z,v)|z*v).sum();for i in 0..m.n{let gw=if i==0{0.0}else{m.g[i-1]};let ge=if i==m.n-1{m.gr}else{m.g[i]};let w=if i==0{0.0}else{x[i-1]};let e=if i==m.n-1{0.0}else{x[i+1]};nx[i]=x[i]+dt/m.v[i]*(gw*w-(gw+ge)*x[i]+if i==m.n-1{0.0}else{ge*e})+dt*m.src[i];}let new_inv:f64=nx.iter().zip(&m.v).map(|(z,v)|z*v).sum();let release=m.gr*x[m.n-1];let residual=((new_inv-old_inv)/dt-gen+release).abs();max_cons=max_cons.max(residual);std::mem::swap(&mut x,&mut nx);} (dt,x,max_cons)
}
fn main(){
 println!("SPATIAL");let mut prev=None;for n in [25usize,50,100,200,400]{let (e,rel,cons)=spatial(n);let p=prev.map(|pe:f64|(pe/e).ln()/2f64.ln());println!("N={n} h={:.12e} E_V={e:.12e} p={} release={rel:.12e} cons={cons:.3e}",R[5]/n as f64,p.map(|v|format!("{v:.6}")).unwrap_or("-".into()));prev=Some(e);}
 println!("TEMPORAL");let m=mesh(200);let dm=dtmax(&m);let tend=0.1;let qs=[4.0,8.0,16.0,32.0,64.0,128.0];let sols:Vec<(f64,f64,Vec<f64>,f64)>=qs.iter().map(|q|{let req=dm/q;let (dt,x,cons)=integrate(&m,req,tend);(req,dt,x,cons)}).collect();let mut prev_diff:Option<f64>=None;
 for k in 0..sols.len()-1{let (dt_req,dt,ref x,cons)=sols[k];let (_,dt_fine,ref fine,_)=sols[k+1];let er:Vec<f64>=x.iter().zip(fine).map(|(a,b)|a-b).collect();let diff=vnorm(&m,&er);let p=prev_diff.map(|pd|(pd/diff).ln()/(dt/dt_fine).ln());println!("dt_req={dt_req:.12e} dt_actual={dt:.12e} dt_actual_fine={dt_fine:.12e} pair_diff_V={diff:.12e} p={} max_cons={cons:.3e}",p.map(|v|format!("{v:.6}")).unwrap_or("-".into()));prev_diff=Some(diff);}
}
