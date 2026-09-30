use boon_lay::{
    lagrangian_decay_simulator::lagrangian_diffusion::{
        first_passage::walk_on_spheres::{sample_uniform_in_ball, WalkParams, WoSWalker},
        single_particle_simulator::constructive_solid_geometry::TrisoCell,
    }, Nuclide,
};
use uom::si::{diffusion_coefficient::square_meter_per_second,f64::Length,length::{meter,nanometer},time::second};

fn layer_data(cell:&TrisoCell)->([f64;6],[f64;5]){
 let r=[0.0,cell.get_fuel_radius().get::<meter>(),cell.get_buffer_radius().get::<meter>(),cell.get_ipyc_radius().get::<meter>(),cell.get_sic_radius().get::<meter>(),cell.get_opyc_radius().get::<meter>()];
 let x=[0.5*r[1],0.5*(r[1]+r[2]),0.5*(r[2]+r[3]),0.5*(r[3]+r[4]),0.5*(r[4]+r[5])];
 let mut d=[0.0;5];
 for i in 0..5{let p=[Length::new::<meter>(x[i]),Length::new::<meter>(0.0),Length::new::<meter>(0.0)];d[i]=cell.try_get_diffusion_coefficient(p,Nuclide::Cs137).unwrap().get::<square_meter_per_second>();}
 (r,d)
}
fn vol(a:f64,b:f64)->f64{4.0*std::f64::consts::PI/3.0*(b.powi(3)-a.powi(3))}
fn centroid(a:f64,b:f64)->f64{0.75*(b.powi(4)-a.powi(4))/(b.powi(3)-a.powi(3))}
struct Fv{n:usize,v:Vec<f64>,g:Vec<f64>,gr:f64,c:Vec<f64>}
fn fv(r:[f64;6],d:[f64;5],m:usize)->Fv{
 let mut f=vec![0.0];for l in 0..5{for j in 1..=m{f.push(r[l]+(r[l+1]-r[l])*j as f64/m as f64);}}
 let n=f.len()-1;let rc:Vec<f64>=(0..n).map(|i|centroid(f[i],f[i+1])).collect();let v:Vec<f64>=(0..n).map(|i|vol(f[i],f[i+1])).collect();
 let layer=|x:f64|->usize{(0..5).find(|&i|x<r[i+1]||i==4).unwrap()};let mut g=vec![0.0;n-1];
 for i in 0..n-1{let rf=f[i+1];g[i]=4.0*std::f64::consts::PI*rf*rf/((rf-rc[i])/d[layer(rc[i])]+(rc[i+1]-rf)/d[layer(rc[i+1])]);}
 let gr=4.0*std::f64::consts::PI*r[5]*r[5]*d[4]/(r[5]-rc[n-1]);let kv=vol(0.0,r[1]);
 let c=(0..n).map(|i|if f[i+1]<=r[1]*(1.0+1e-12){1.0/kv}else{0.0}).collect();Fv{n,v,g,gr,c}
}
fn dtmax(m:&Fv)->f64{let mut z=f64::INFINITY;for i in 0..m.n{let gw=if i==0{0.0}else{m.g[i-1]};let ge=if i==m.n-1{m.gr}else{m.g[i]};z=z.min(m.v[i]/(gw+ge));}z}
fn curve(mut m:Fv,times:&[f64])->Vec<f64>{let dt=0.8*dtmax(&m);let mut out=Vec::new();let mut t=0.0;let mut rel=0.0;let mut k=0;let mut nx=vec![0.0;m.n];while k<times.len(){if t>=times[k]{out.push(rel.min(1.0));k+=1;continue}let h=dt.min(times[k]-t);let q=m.gr*m.c[m.n-1];for i in 0..m.n{let gw=if i==0{0.0}else{m.g[i-1]};let ge=if i==m.n-1{m.gr}else{m.g[i]};let w=if i==0{0.0}else{m.c[i-1]};let e=if i==m.n-1{0.0}else{m.c[i+1]};nx[i]=m.c[i]+h/m.v[i]*(gw*w-(gw+ge)*m.c[i]+if i==m.n-1{0.0}else{ge*e});}rel+=h*q;std::mem::swap(&mut m.c,&mut nx);t+=h;}out}
fn main(){
 let cell=TrisoCell::new_crp6_geometry();let(r,d)=layer_data(&cell);println!("WOS_FV_COMPARISON");println!("radii_m={r:?}");println!("D_m2_s={d:?}");
 let scale=r[5]*r[5]/d.iter().copied().fold(f64::INFINITY,f64::min);let times:Vec<f64>=[0.01,0.03,0.1,0.3,1.0,3.0].iter().map(|x|x*scale).collect();let fref=curve(fv(r,d,40),&times);
 let params=WalkParams{capture_eps:Length::new::<nanometer>(10.0),reinsert_factor:2.0,partition_k:1.0,max_steps:5_000_000};let n=2000usize;let mut release=Vec::with_capacity(n);let mut censored=0;
 for i in 0..n{let mut master=boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::central_limit_theorem::oorandom_rng::OoRng64::from_u64(0xA110_0000+i as u64);let start=sample_uniform_in_ball(&mut master.0,cell.get_fuel_radius());let child=boon_lay::lagrangian_decay_simulator::lagrangian_diffusion::central_limit_theorem::oorandom_rng::OoRng64::from_u64(master.next_u64());let mut w=WoSWalker::new(start,Nuclide::Cs137,child);match w.walk_until_released(&cell,&params){Some(t)=>release.push(t.get::<second>()),None=>censored+=1}}
 println!("N={n} censored={censored} capture_nm=10 reinsert_factor=2 max_steps=5000000");assert_eq!(censored,0);
 for(j,&t)in times.iter().enumerate(){let count=release.iter().filter(|&&x|x<=t).count();let fw=count as f64/n as f64;let se=(fw*(1.0-fw)/n as f64).sqrt();println!("t_s={t:.12e} F_FV={:.8} F_WOS={fw:.8} MC_SE={se:.8} z={:.4}",fref[j],if se>0.0{(fw-fref[j])/se}else{0.0});}
}