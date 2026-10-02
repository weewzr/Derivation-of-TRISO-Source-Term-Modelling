import mpmath as mp, json, time, platform
RSTR=["2.125e-4","3.125e-4","3.525e-4","3.875e-4","4.275e-4"]
DSTR=["1.2502982636347968e-13","1e-8","4.062299125614697e-14","9.227773168241615e-17","4.062299125614697e-14"]
DELSTR="2e-7"; TIMES=["1e2","1e4","1e6","1e8","1e10"]; REQUESTED_DPS=50
def params():return [mp.mpf(x) for x in RSTR],[mp.mpf(x) for x in DSTR],mp.mpf(DELSTR)
def p(D,i,j):return D[j]/(D[i]+D[j])
def ratio(x,y):
 if abs(y)<mp.mpf("1e-20"):return x/y
 if mp.re(y)<30:return mp.sinh(x)/mp.sinh(y)
 return mp.exp(x-y)*(1-mp.exp(-2*x))/(1-mp.exp(-2*y))
def ball(r,b,d,s):
 if s==0:return mp.mpf(1)
 l=mp.sqrt(s/d);z=l*b
 if r==0:return z/mp.sinh(z) if mp.re(z)<30 else 2*z*mp.exp(-z)/(1-mp.exp(-2*z))
 return (b/r)*ratio(l*r,z)
def shell(a,b,r,d,s):
 if s==0:return a*(b-r)/(r*(b-a)),b*(r-a)/(r*(b-a))
 l=mp.sqrt(s/d);den=l*(b-a)
 return (a/r)*ratio(l*(b-r),den),(b/r)*ratio(l*(r-a),den)
def source_exact(s):
 R,D,_=params()
 if s==0:return mp.mpf(1)
 z=R[0]*mp.sqrt(s/D[0])
 if abs(z)<mp.mpf("1e-8"):return 1-z*z/15+2*z**4/315
 if mp.re(z)>30:
  em=mp.exp(-2*z);coth=(1+em)/(1-em)
 else:coth=mp.coth(z)
 return 3*(z*coth-1)/(z*z)
def source_quad(s):
 R,D,_=params()
 if s==0:return mp.mpf(1)
 return mp.quad(lambda r:3*r*r/R[0]**3*ball(r,R[0],D[0],s),[0,R[0]])
def matrix(s):
 R,D,de=params();K=mp.matrix(8);B=mp.matrix(8,1)
 h=ball(R[0]-de,R[0],D[0],s);K[0,0]=h*(1-p(D,0,1));K[0,1]=h*p(D,0,1)
 for st,r in [(1,R[0]+de),(2,R[1]-de)]:
  gm,gp=shell(R[0],R[1],r,D[1],s);K[st,0]=gm*p(D,1,0);K[st,1]=gm*(1-p(D,1,0));K[st,2]=gp*(1-p(D,1,2));K[st,3]=gp*p(D,1,2)
 for st,r in [(3,R[1]+de),(4,R[2]-de)]:
  gm,gp=shell(R[1],R[2],r,D[2],s);K[st,2]=gm*p(D,2,1);K[st,3]=gm*(1-p(D,2,1));K[st,4]=gp*(1-p(D,2,3));K[st,5]=gp*p(D,2,3)
 for st,r in [(5,R[2]+de),(6,R[3]-de)]:
  gm,gp=shell(R[2],R[3],r,D[3],s);K[st,4]=gm*p(D,3,2);K[st,5]=gm*(1-p(D,3,2));K[st,6]=gp*(1-p(D,3,4));K[st,7]=gp*p(D,3,4)
 gm,gp=shell(R[3],R[4],R[3]+de,D[4],s);K[7,6]=gm*p(D,4,3);K[7,7]=gm*(1-p(D,4,3));B[7]=gp
 return K,B
def phi_raw(s):
 K,B=matrix(s);x=mp.lu_solve(mp.eye(8)-K,B);R,D,_=params();q=(1-p(D,0,1))*x[0]+p(D,0,1)*x[1]
 return source_exact(s)*q
class Cache:
 def __init__(self):self.d={};self.calls=0;self.misses=0
 def key(self,s):return (mp.mp.dps,mp.nstr(mp.re(s),mp.mp.dps+8),mp.nstr(mp.im(s),mp.mp.dps+8))
 def phi(self,s):
  self.calls+=1;k=self.key(s)
  if k not in self.d:self.misses+=1;self.d[k]=phi_raw(s)
  return self.d[k]
def main():
 mp.mp.dps=80
 source_checks=[]
 for s in [mp.mpf("1e-8"),mp.mpf("1e-5"),mp.mpc("1e-6","2e-6")]:
  a=source_exact(s);q=source_quad(s);source_checks.append({"s":str(s),"exact":mp.nstr(a,30),"quad":mp.nstr(q,30),"abs_error":mp.nstr(abs(a-q),12)})
 expected=[("1e-9","0.955021769459339"),("1e-7","0.147639152754869"),("1e-5","3.22458722262e-7")]
 real_axis=[{"s":s,"abs_error":mp.nstr(abs(phi_raw(mp.mpf(s))-mp.mpf(v)),12)} for s,v in expected]
 cpts=[mp.mpc("1e-8","2e-8"),mp.mpc("1e-6","3e-6"),mp.mpc("1e-4","2e-4")]
 conj=[{"s":str(z),"abs_error":mp.nstr(abs(phi_raw(mp.conj(z))-mp.conj(phi_raw(z))),12)} for z in cpts]
 mp.mp.dps=REQUESTED_DPS;c=Cache();rows=[];t0=time.time()
 for ts in TIMES:
  t=mp.mpf(ts)
  a=time.time();F=mp.invertlaplace(lambda s:c.phi(s)/s,t,method="stehfest");tc=time.time()-a
  a=time.time();S=mp.invertlaplace(lambda s:(1-c.phi(s))/s,t,method="stehfest");tsur=time.time()-a
  rows.append({"t_s":ts,"cdf":mp.nstr(F,30),"survival":mp.nstr(S,30),"F_plus_S_minus_1":mp.nstr(F+S-1,12),"cdf_seconds":tc,"survival_seconds":tsur})
 vals=[mp.mpf(x["cdf"]) for x in rows]
 out={"classification":"IL-1_SMOKE_ONLY","versions":{"python":platform.python_version(),"mpmath":mp.__version__},"requested_dps":REQUESTED_DPS,"times_s":TIMES,"source_factor_crosscheck":source_checks,"complex_gate":{"principal_sqrt":True,"real_axis":real_axis,"conjugacy":conj},"stehfest":{"rows":rows,"cdf_bounds":all(0<=x<=1 for x in vals),"cdf_monotone":all(vals[i+1]>=vals[i] for i in range(len(vals)-1)),"transform_calls":c.calls,"unique_transform_evaluations":c.misses,"cache_hits":c.calls-c.misses,"runtime_seconds":time.time()-t0},"physics":{"R_m":RSTR,"D_m2_s":DSTR,"epsilon_m":"1e-7","alpha":2,"K":1,"source":"uniform kernel volume","outer":"absorbing OPyC","process":"C"}}
 print(json.dumps(out,indent=2))
if __name__=="__main__":main()
