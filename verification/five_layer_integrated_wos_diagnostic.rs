use std::time::Instant;
use boon_lay::{
 lagrangian_decay_simulator::lagrangian_diffusion::{
  central_limit_theorem::oorandom_rng::OoRng64,
  first_passage::walk_on_spheres::{sample_uniform_in_ball,HopOutcome,WalkParams,WoSWalker},
  single_particle_simulator::constructive_solid_geometry::{TrisoCell,TrisoRegion},
 },Nuclide,
};
use uom::si::{f64::Length,length::{meter,nanometer},time::second};

const N:usize=8; const CAP:u64=1_000_000; const PROGRESS:u64=250_000;
fn idx(r:TrisoRegion)->usize{match r{TrisoRegion::Fuel=>0,TrisoRegion::Buffer=>1,TrisoRegion::IPyC=>2,TrisoRegion::SiC=>3,TrisoRegion::OPyC=>4,TrisoRegion::Outside=>5}}
fn rname(i:usize)->&'static str{["Fuel","Buffer","IPyC","SiC","OPyC","Outside"][i]}
fn radius(p:[Length;3])->f64{(p[0]*p[0]+p[1]*p[1]+p[2]*p[2]).sqrt().get::<meter>()}
fn iface_index(rb:usize,ra:usize)->Option<usize>{if rb==ra{return None} let lo=rb.min(ra); if lo<5{Some(lo)}else{None}}

fn main(){
 let cell=TrisoCell::new_crp6_geometry();
 let params=WalkParams{capture_eps:Length::new::<nanometer>(10.0),reinsert_factor:2.0,partition_k:1.0,max_steps:CAP};
 println!("FIVE_LAYER_INTEGRATED_DIAGNOSTIC N={N} cap={CAP} capture_nm=10 alpha=2 K=1 nuclide=Cs137");
 println!("radii_m={:?}",[cell.get_fuel_radius().get::<meter>(),cell.get_buffer_radius().get::<meter>(),cell.get_ipyc_radius().get::<meter>(),cell.get_sic_radius().get::<meter>(),cell.get_opyc_radius().get::<meter>()]);
 let wall=Instant::now();
 for h in 0..N{
  let mut master=OoRng64::from_u64(0x5A17_0000+h as u64);
  let start=sample_uniform_in_ball(&mut master.0,cell.get_fuel_radius());
  let child=OoRng64::from_u64(master.next_u64());
  let mut w=WoSWalker::new(start,Nuclide::Cs137,child);
  let mut layer_steps=[0u64;6]; let mut encounters=[0u64;5]; let mut transmit=[0u64;5]; let mut reflect=[0u64;5];
  let mut outward=[0u64;5]; let mut inward=[0u64;5]; let mut zero_dt=0u64; let mut positive_dt=0u64;
  let mut sum_dr=0.0_f64;let mut max_dr=0.0_f64;let mut min_dr=f64::INFINITY;let mut max_r=radius(w.position);let mut deepest=0usize;
  let mut released=false;let mut used=0u64;
  for step in 1..=CAP{
   used=step;let before=w.position;let rb=idx(cell.get_triso_region(before));let tb=w.time.get::<second>();let r0=radius(before);
   layer_steps[rb]+=1;deepest=deepest.max(rb.min(4));
   let out=w.step_multilayer(&cell,&params);
   let after=w.position;let ra=idx(cell.get_triso_region(after));let ta=w.time.get::<second>();let r1=radius(after);max_r=max_r.max(r1);deepest=deepest.max(ra.min(4));
   let dr=((after[0]-before[0])*(after[0]-before[0])+(after[1]-before[1])*(after[1]-before[1])+(after[2]-before[2])*(after[2]-before[2])).sqrt().get::<meter>();
   if dr>0.0{sum_dr+=dr;max_dr=max_dr.max(dr);min_dr=min_dr.min(dr)}
   if ta==tb{zero_dt+=1}else{positive_dt+=1}
   if out==HopOutcome::ReachedInterface{
    let iface=if rb<5 && ra<5 { if rb==ra {
      // Reflection: infer nearest material boundary from current layer and pre-event radius.
      let bounds=[cell.get_fuel_radius().get::<meter>(),cell.get_buffer_radius().get::<meter>(),cell.get_ipyc_radius().get::<meter>(),cell.get_sic_radius().get::<meter>(),cell.get_opyc_radius().get::<meter>()];
      let inner=if rb==0{f64::INFINITY}else{r0-bounds[rb-1]};let outer=bounds[rb]-r0;if inner<outer{rb-1}else{rb}
     } else {iface_index(rb,ra).unwrap_or(4)}} else {4};
    encounters[iface]+=1;
    if rb!=ra{transmit[iface]+=1;if ra>rb{outward[iface]+=1}else{inward[iface]+=1}}else{reflect[iface]+=1}
   }
   if out==HopOutcome::Released{released=true;break}
   if step%PROGRESS==0{println!("history={h} progress_steps={step} layer={} deepest={} physical_time_s={:.6e} max_radius_m={:.9e} interface_events={} elapsed_wall_s={:.3}",rname(ra),rname(deepest),ta,max_r,encounters.iter().sum::<u64>(),wall.elapsed().as_secs_f64());}
  }
  let term=if released{"Released"}else{"CensoredMaxSteps"};
  println!("history={h} termination={term} steps={used} physical_time_s={:.12e} current_layer={} deepest_layer={} max_radius_m={max_r:.12e} layer_steps={layer_steps:?}",w.time.get::<second>(),rname(idx(cell.get_triso_region(w.position))),rname(deepest));
  println!("history={h} interface_encounters={encounters:?} transmitted={transmit:?} reflected={reflect:?} outward={outward:?} inward={inward:?} zero_dt={zero_dt} positive_dt={positive_dt}");
  println!("history={h} displacement_m mean={:.12e} min={:.12e} max={:.12e}",if positive_dt+zero_dt>0{sum_dr/(positive_dt+zero_dt)as f64}else{0.0},if min_dr.is_finite(){min_dr}else{0.0},max_dr);
 }
 println!("diagnostic_wall_s={:.3}",wall.elapsed().as_secs_f64());
 println!("NOTE transmission/reflection counts are verification-harness classifications of unchanged production step_multilayer outcomes; supervisor source is unmodified.");
}