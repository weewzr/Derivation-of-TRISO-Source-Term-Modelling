import mpmath as mp, json, time, platform
from inverse_laplace_il1_smoke import phi_raw, Cache
TIMES=["1e2","1e4","1e6","1e8","1e10"]; PREC=[50,80,120]; PRIMARY=80
def inv(cache,t,method,dps,kind):
 mp.mp.dps=dps
 f=(lambda s:cache.phi(s)/s) if kind=="cdf" else (lambda s:(1-cache.phi(s))/s)
 a=time.time();v=mp.invertlaplace(f,mp.mpf(t),method=method);return v,time.time()-a
def main():
 out={"classification":"IL-2_METHOD_PRECISION_PILOT_ONLY","versions":{"python":platform.python_version(),"mpmath":mp.__version__},"times_s":TIMES,"precision_sequence":PREC,"primary_dps":PRIMARY,"methods":{}}
 for method in ("stehfest","dehoog"):
  rows=[];c=Cache()
  for ts in TIMES:
   F,tf=inv(c,ts,method,PRIMARY,"cdf");S,tsur=inv(c,ts,method,PRIMARY,"survival")
   rows.append({"t_s":ts,"cdf":mp.nstr(F,40),"survival":mp.nstr(S,40),"F_plus_S_minus_1":mp.nstr(F+S-1,18),"cdf_seconds":tf,"survival_seconds":tsur})
  conv={}
  for ts in TIMES:
   vals={}
   for dps in PREC:
    cc=Cache();F,_=inv(cc,ts,method,dps,"cdf");vals[str(dps)]=mp.nstr(F,45)
   f50,f80,f120=[mp.mpf(vals[str(d)]) for d in PREC]
   conv[ts]={"F50":vals["50"],"F80":vals["80"],"F120":vals["120"],"abs_50_80":mp.nstr(abs(f50-f80),18),"abs_80_120":mp.nstr(abs(f80-f120),18)}
  vv=[mp.mpf(x["cdf"]) for x in rows]
  out["methods"][method]={"rows":rows,"precision_convergence":conv,"raw_bounds":all(0<=x<=1 for x in vv),"raw_monotone":all(vv[i+1]>=vv[i] for i in range(len(vv)-1)),"transform_calls":c.calls,"unique_transform_evaluations":c.misses,"cache_hits":c.calls-c.misses}
 A=out["methods"]["stehfest"]["rows"];B=out["methods"]["dehoog"]["rows"]
 out["comparison"]=[{"t_s":a["t_s"],"stehfest":a["cdf"],"dehoog":b["cdf"],"signed_difference":mp.nstr(mp.mpf(a["cdf"])-mp.mpf(b["cdf"]),20),"absolute_difference":mp.nstr(abs(mp.mpf(a["cdf"])-mp.mpf(b["cdf"])),20)} for a,b in zip(A,B)]
 print(json.dumps(out,indent=2))
if __name__=="__main__":main()
