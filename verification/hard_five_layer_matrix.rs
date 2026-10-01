use std::f64::consts::PI;
const R:[f64;5]=[2.125e-4,3.125e-4,3.525e-4,3.875e-4,4.275e-4];
const D:[f64;5]=[1.2502982636347968e-13,1.0e-8,4.062299125614697e-14,9.227773168241615e-17,4.062299125614697e-14];
const DEL:f64=200e-9; const SS:[f64;7]=[0.0,1e-9,1e-8,1e-7,1e-6,1e-5,1e-4];

fn p(i:usize,j:usize)->f64{D[j]/(D[i]+D[j])}
fn ratio(x:f64,y:f64)->f64{if y.abs()<1e-6{x/y}else if y<40.0{x.sinh()/y.sinh()}else{(x-y).exp()*(1.0-(-2.0*x).exp())/(1.0-(-2.0*y).exp())}}
fn ball(r:f64,b:f64,d:f64,s:f64)->f64{if s==0.0{return 1.0}let l=(s/d).sqrt();let z=l*b;if r==0.0{return if z<40.0{z/z.sinh()}else{2.0*z*(-z).exp()/(1.0-(-2.0*z).exp())}}(b/r)*ratio(l*r,z)}
fn shell(a:f64,b:f64,r:f64,d:f64,s:f64)->(f64,f64){if s==0.0{return(a*(b-r)/(r*(b-a)),b*(r-a)/(r*(b-a)))}let l=(s/d).sqrt();let den=l*(b-a);((a/r)*ratio(l*(b-r),den),(b/r)*ratio(l*(r-a),den))}
fn mat(s:f64)->([[f64;8];8],[f64;8]){let mut k=[[0.0;8];8];let mut b=[0.0;8];
 let h=ball(R[0]-DEL,R[0],D[0],s);k[0][0]=h*(1.0-p(0,1));k[0][1]=h*p(0,1);
 for(st,r)in[(1,R[0]+DEL),(2,R[1]-DEL)]{let(gm,gp)=shell(R[0],R[1],r,D[1],s);k[st][0]=gm*p(1,0);k[st][1]=gm*(1.0-p(1,0));k[st][2]=gp*(1.0-p(1,2));k[st][3]=gp*p(1,2);}
 for(st,r)in[(3,R[1]+DEL),(4,R[2]-DEL)]{let(gm,gp)=shell(R[1],R[2],r,D[2],s);k[st][2]=gm*p(2,1);k[st][3]=gm*(1.0-p(2,1));k[st][4]=gp*(1.0-p(2,3));k[st][5]=gp*p(2,3);}
 for(st,r)in[(5,R[2]+DEL),(6,R[3]-DEL)]{let(gm,gp)=shell(R[2],R[3],r,D[3],s);k[st][4]=gm*p(3,2);k[st][5]=gm*(1.0-p(3,2));k[st][6]=gp*(1.0-p(3,4));k[st][7]=gp*p(3,4);}
 let(gm,gp)=shell(R[3],R[4],R[3]+DEL,D[4],s);k[7][6]=gm*p(4,3);k[7][7]=gm*(1.0-p(4,3));b[7]=gp;(k,b)}
fn solve(mut a:[[f64;8];8],mut b:[f64;8])->[f64;8]{for i in 0..8{let mut m=i;for j in i+1..8{if a[j][i].abs()>a[m][i].abs(){m=j}}a.swap(i,m);b.swap(i,m);let q=a[i][i];for j in i..8{a[i][j]/=q}b[i]/=q;for r in 0..8{if r!=i{let f=a[r][i];for j in i..8{a[r][j]-=f*a[i][j]}b[r]-=f*b[i]}}}b}
fn inv(a:[[f64;8];8])->[[f64;8];8]{let mut z=[[0.0;8];8];for j in 0..8{let mut e=[0.0;8];e[j]=1.0;let c=solve(a,e);for i in 0..8{z[i][j]=c[i]}}z}
fn jacobi(mut a:[[f64;8];8])->[f64;8]{for _ in 0..200{let(mut p,mut q,mut mx)=(0,1,0.0);for i in 0..8{for j in i+1..8{if a[i][j].abs()>mx{mx=a[i][j].abs();p=i;q=j}}}if mx<1e-18{break}let th=0.5*(2.0*a[p][q]).atan2(a[q][q]-a[p][p]);let(c,s)=(th.cos(),th.sin());for k in 0..8{if k!=p&&k!=q{let(ap,aq)=(a[k][p],a[k][q]);a[k][p]=c*ap-s*aq;a[p][k]=a[k][p];a[k][q]=s*ap+c*aq;a[q][k]=a[k][q]}}let(app,aqq,apq)=(a[p][p],a[q][q],a[p][q]);a[p][p]=c*c*app-2.0*s*c*apq+s*s*aqq;a[q][q]=s*s*app+2.0*s*c*apq+c*c*aqq;a[p][q]=0.0;a[q][p]=0.0}let mut e=[0.0;8];for i in 0..8{e[i]=a[i][i]}e}
fn metrics(k:[[f64;8];8],b:[f64;8])->([f64;8],f64,f64,f64){let mut a=[[0.0;8];8];for i in 0..8{for j in 0..8{a[i][j]=(if i==j{1.0}else{0.0})-k[i][j]}}let x=solve(a,b);let mut rr=0.0;for i in 0..8{let mut z=-b[i];for j in 0..8{z+=a[i][j]*x[j]}rr+=z*z}let ai=inv(a);let nf=a.iter().flatten().map(|v|v*v).sum::<f64>().sqrt();let ni=ai.iter().flatten().map(|v|v*v).sum::<f64>().sqrt();let mut ata=[[0.0;8];8];for i in 0..8{for j in 0..8{for m in 0..8{ata[i][j]+=a[m][i]*a[m][j]}}}let ev=jacobi(ata);let smin=ev.iter().copied().filter(|v|*v>=0.0).fold(f64::INFINITY,f64::min).sqrt();(x,rr.sqrt(),nf*ni,smin)}
fn rho(k:[[f64;8];8])->f64{let mut v=[1.0/8.0_f64.sqrt();8];let mut lam:f64=0.0;for _ in 0..200000{let mut w=[0.0;8];for i in 0..8{for j in 0..8{w[i]+=k[i][j]*v[j]}}let n=w.iter().map(|x|x*x).sum::<f64>().sqrt();if n==0.0{return 0.0}for i in 0..8{w[i]/=n}let mut kv=[0.0;8];for i in 0..8{for j in 0..8{kv[i]+=k[i][j]*w[j]}}let nl:f64=w.iter().zip(kv).map(|(a,b)|a*b).sum::<f64>();if(nl-lam).abs()<1e-15{lam=nl;break}lam=nl;v=w}lam}
fn init(s:f64,x:[f64;8],n:usize)->f64{let q=(1.0-p(0,1))*x[0]+p(0,1)*x[1];if s==0.0{return q}let dr=R[0]/n as f64;let mut z=0.0;for i in 0..n{let r=(i as f64+0.5)*dr;z+=3.0*r*r/R[0].powi(3)*ball(r,R[0],D[0],s)}z*dr*q}
fn main(){println!("HARD8 R={R:?} D={D:?} delta={DEL:.12e} s={SS:?}");
 for i in 0..4{println!("IFACE i={i} p_out={:.16e} p_in={:.16e} refl_out={:.16e} refl_in={:.16e}",p(i,i+1),p(i+1,i),1.0-p(i,i+1),1.0-p(i+1,i));}
 for &(x,y) in &[(1e-8,2e-8),(2.0,3.0),(1000.0,1200.0)]{println!("RATIO_TEST x={x:.3e} y={y:.3e} value={:.16e}",ratio(x,y));}
 let mut prev=[f64::INFINITY;8];let mut previ=f64::INFINITY;
 for &s in &SS{let(k,b)=mat(s);let(x,res,cond,smin)=metrics(k,b);let r=if s==0.0{rho(k)}else{f64::NAN};let i1=init(s,x,1000);let i2=init(s,x,10000);let i3=init(s,x,100000);let mut rows=[0.0;8];for i in 0..8{rows[i]=k[i].iter().sum::<f64>()+b[i]}let bound=x.iter().all(|v|v.is_finite()&&*v>=-1e-10&&*v<=1.0+1e-10)&&i3.is_finite()&&i3>=-1e-10&&i3<=1.0+1e-10;let mono=x.iter().zip(prev).all(|(a,b)|*a<=b+1e-10)&&i3<=previ+1e-10;println!("S s={s:.1e} rho={r:.16e} gap={:.16e} phi={x:?} init1k={i1:.16e} init10k={i2:.16e} init100k={i3:.16e} qdiff1={:.3e} qdiff2={:.3e} residual={res:.3e} kappaF={cond:.6e} sigma_min={smin:.16e} bounds={bound} monotone={mono} rows={rows:?}",if s==0.0{1.0-r}else{f64::NAN},(i2-i1).abs(),(i3-i2).abs());prev=x;previ=i3;}
}