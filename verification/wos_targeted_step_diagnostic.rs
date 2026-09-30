use boon_lay::{
    lagrangian_decay_simulator::lagrangian_diffusion::{
        central_limit_theorem::oorandom_rng::OoRng64,
        first_passage::walk_on_spheres::{sample_uniform_in_ball, HopOutcome, WalkParams, WoSWalker},
        single_particle_simulator::constructive_solid_geometry::{TrisoCell, TrisoRegion},
    }, Nuclide,
};
use uom::si::{f64::Length,length::{meter,nanometer},time::second};

fn region(cell:&TrisoCell,p:[Length;3])->TrisoRegion{cell.get_triso_region(p)}
fn ridx(r:TrisoRegion)->usize{match r{TrisoRegion::Fuel=>0,TrisoRegion::Buffer=>1,TrisoRegion::IPyC=>2,TrisoRegion::SiC=>3,TrisoRegion::OPyC=>4,TrisoRegion::Outside=>5}}

fn main(){
 const N:usize=3; const CAP:u64=200_000; const TRACE:usize=40;
 let cell=TrisoCell::new_crp6_geometry();
 let params=WalkParams{capture_eps:Length::new::<nanometer>(10.0),reinsert_factor:2.0,partition_k:1.0,max_steps:CAP};
 println!("WOS_TARGETED_STEP_DIAGNOSTIC N={N} cap={CAP} capture_nm=10 reinsert=2 K=1");
 for i in 0..N{
  let mut master=OoRng64::from_u64(0xD1A6_0000+i as u64);
  let start=sample_uniform_in_ball(&mut master.0,cell.get_fuel_radius());
  let child=OoRng64::from_u64(master.next_u64());
  let mut w=WoSWalker::new(start,Nuclide::Cs137,child);
  let mut layer_steps=[0u64;6]; let mut interface_events=0u64; let mut transitions=[[0u64;6];6];
  let mut zero_dt=0u64; let mut max_r=w.radius().get::<meter>(); let mut min_positive_dr=f64::INFINITY;
  let mut released=false; let mut used=0u64;
  for step in 1..=CAP{
   let rb=region(&cell,w.position); let ib=ridx(rb); let before=w.position; let tb=w.time.get::<second>(); let radb=w.radius().get::<meter>();
   let out=w.step_multilayer(&cell,&params); let ra=region(&cell,w.position); let ia=ridx(ra); let ta=w.time.get::<second>(); let rada=w.radius().get::<meter>();
   layer_steps[ib]+=1; used=step; max_r=max_r.max(rada);
   let dr=((w.position[0]-before[0])*(w.position[0]-before[0])+(w.position[1]-before[1])*(w.position[1]-before[1])+(w.position[2]-before[2])*(w.position[2]-before[2])).sqrt().get::<meter>();
   if dr>0.0{min_positive_dr=min_positive_dr.min(dr)}
   if ta==tb{zero_dt+=1}
   if out==HopOutcome::ReachedInterface{interface_events+=1}
   if ia!=ib{transitions[ib][ia]+=1}
   if i==0 && step as usize<=TRACE{println!("trace step={step} before={rb:?} after={ra:?} outcome={out:?} r_before={radb:.12e} r_after={rada:.12e} dt_s={:.12e}",ta-tb);}
   if out==HopOutcome::Released{released=true;break}
  }
  println!("trajectory={i} released={released} steps={used} physical_time_s={:.12e} layer_steps={layer_steps:?} interface_events={interface_events} zero_dt_events={zero_dt} max_radius_m={max_r:.12e} min_positive_displacement_m={min_positive_dr:.12e}",w.time.get::<second>());
  println!("trajectory={i} transitions={transitions:?}");
 }
}