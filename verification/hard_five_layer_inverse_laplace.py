import mpmath as mp, json, time, platform
RSTR=["2.125e-4","3.125e-4","3.525e-4","3.875e-4","4.275e-4"]
DSTR=["1.2502982636347968e-13","1e-8","4.062299125614697e-14","9.227773168241615e-17","4.062299125614697e-14"]
DELSTR="2e-7"; PREC=[50,80,120]
TIMES_STR=[mp.nstr(mp.mpf(10)**(mp.mpf(2)+mp.mpf(i)/10),18) for i in range(81)]
def params():return [mp.mpf(x) for x in RSTR],[mp.mpf(x) for x in DSTR],mp.mpf(DELSTR)
def prob(D,i,j):return D[j]/(D[i]+D[j])
def ratio_complex(x,y):
    # Algebraic scaled form selected by Re(y), valid for complex y with Re(y)>=0.
    if abs(y)<mp.mpf("1e-20"):return x/y
    if mp.re(y)<30:return mp.sinh(x)/mp.sinh(y)
    return mp.exp(x-y)*(1-mp.exp(-2*x))/(1-mp.exp(-2*y))
def ball(r,b,d,s):
    if s==0:return mp.mpf(1)
    z=mp.sqrt(s/d)*b
    if r==0:return z/mp.sinh(z) if mp.re(z)<30 else 2*z*mp.exp(-z)/(1-mp.exp(-2*z))
    return (b/r)*ratio_complex(mp.sqrt(s/d)*r,z)
def shell(a,b,r,d,s):
    if s==0:return a*(b-r)/(r*(b-a)),b*(r-a)/(r*(b-a))
    l=mp.sqrt(s/d);den=l*(b-a)
    return (a/r)*ratio_complex(l*(b-r),den),(b/r)*ratio_complex(l*(r-a),den)
def matrix(s):
    R,D,de=params();K=mp.matrix(8);B=mp.matrix(8,1)
    h=ball(R[0]-de,R[0],D[0],s);K[0,0]=h*(1-prob(D,0,1));K[0,1]=h*prob(D,0,1)
    for st,r in [(1,R[0]+de),(2,R[1]-de)]:
      gm,gp=shell(R[0],R[1],r,D[1],s);K[st,0]=gm*prob(D,1,0);K[st,1]=gm*(1-prob(D,1,0));K[st,2]=gp*(1-prob(D,1,2));K[st,3]=gp*prob(D,1,2)
    for st,r in [(3,R[1]+de),(4,R[2]-de)]:
      gm,gp=shell(R[1],R[2],r,D[2],s);K[st,2]=gm*prob(D,2,1);K[st,3]=gm*(1-prob(D,2,1));K[st,4]=gp*(1-prob(D,2,3));K[st,5]=gp*prob(D,2,3)
    for st,r in [(5,R[2]+de),(6,R[3]-de)]:
      gm,gp=shell(R[2],R[3],r,D[3],s);K[st,4]=gm*prob(D,3,2);K[st,5]=gm*(1-prob(D,3,2));K[st,6]=gp*(1-prob(D,3,4));K[st,7]=gp*prob(D,3,4)
    gm,gp=shell(R[3],R[4],R[3]+de,D[4],s);K[7,6]=gm*prob(D,4,3);K[7,7]=gm*(1-prob(D,4,3));B[7]=gp
    return K,B
def source_factor(s):
    R,D,_=params()
    if s==0:return mp.mpf(1)
    f=lambda r:3*r*r/R[0]**3*ball(r,R[0],D[0],s)
    return mp.quad(f,[0,R[0]])
def phi(s):
    K,B=matrix(s);x=mp.lu_solve(mp.eye(8)-K,B);R,D,_=params()
    q=(1-prob(D,0,1))*x[0]+prob(D,0,1)*x[1]
    return source_factor(s)*q
def complex_gate():
    mp.mp.dps=80
    real=["1e-9","1e-7","1e-5"]
    expected=["0.955021769459339","0.147639152754869","3.22458722262e-7"]
    real_err=[abs(phi(mp.mpf(s))-mp.mpf(e)) for s,e in zip(real,expected)]
    pts=[mp.mpc("1e-8","2e-8"),mp.mpc("1e-6","3e-6"),mp.mpc("1e-4","2e-4")]
    conj=[abs(phi(mp.conj(z))-mp.conj(phi(z))) for z in pts]
    return {"principal_sqrt":True,"real_axis_max_abs_error":mp.nstr(max(real_err),15),"conjugacy_max_abs_error":mp.nstr(max(conj),15)}
def invert_cdf(t,method,dps):
    mp.mp.dps=dps
    return mp.invertlaplace(lambda s:phi(s)/s,mp.mpf(t),method=method)
def run():
    gate=complex_gate()
    # execute full physical grid at primary precision; convergence at representative points at all precisions
    vals={};cost={}
    for method in ("stehfest","dehoog"):
      st=time.time();vals[method]=[invert_cdf(t,method,80) for t in TIMES_STR];cost[method]=time.time()-st
    reps=[0,20,40,60,80];conv={}
    for method in ("stehfest","dehoog"):
      conv[method]={}
      for idx in reps:
       conv[method][TIMES_STR[idx]]={str(d):mp.nstr(invert_cdf(TIMES_STR[idx],method,d),30) for d in PREC}
    # reconstruction using log-time trapezoid: Phi(s)=int exp(-st)dF, approximated from CDF increments
    recon={}
    for method in ("stehfest","dehoog"):
      F=vals[method]; rr={}
      for ss in ["1e-9","1e-8","1e-7","1e-6","1e-5"]:
       sv=mp.mpf(ss);z=mp.mpf(0)
       for i in range(1,len(F)):
        dF=F[i]-F[i-1];tm=mp.sqrt(mp.mpf(TIMES_STR[i])*mp.mpf(TIMES_STR[i-1]));z+=mp.e**(-sv*tm)*dF
       ref=phi(sv);rr[ss]={"reconstructed":mp.nstr(z,25),"reference":mp.nstr(ref,25),"abs_error":mp.nstr(abs(z-ref),12),"rel_error":mp.nstr(abs(z-ref)/max(abs(ref),mp.mpf("1e-80")),12)}
      recon[method]=rr
    out={"versions":{"python":platform.python_version(),"mpmath":mp.__version__},"complex_gate":gate,"time_grid_s":TIMES_STR,"precision_sequence":PREC,"methods":{},"reconstruction":recon}
    for method in vals:
      v=vals[method];out["methods"][method]={"cdf":[mp.nstr(x,30) for x in v],"seconds":cost[method],"min":mp.nstr(min(v),20),"max":mp.nstr(max(v),20),"monotone":all(v[i+1]>=v[i] for i in range(len(v)-1)),"precision_convergence":conv[method]}
    out["comparison"]={"max_abs":mp.nstr(max(abs(a-b) for a,b in zip(vals["stehfest"],vals["dehoog"])),15),"pairs":[{"t":TIMES_STR[i],"stehfest":mp.nstr(vals["stehfest"][i],20),"dehoog":mp.nstr(vals["dehoog"][i],20),"abs":mp.nstr(abs(vals["stehfest"][i]-vals["dehoog"][i]),12)} for i in range(len(vals["stehfest"]))]}
    return out
if __name__=="__main__":print(json.dumps(run(),indent=2))
