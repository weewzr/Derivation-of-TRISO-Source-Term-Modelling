import mpmath as mp, json, time, platform
from inverse_laplace_il1_smoke import Cache, phi_raw
TIMES=[mp.nstr(mp.mpf(10)**(mp.mpf(2)+mp.mpf(i)/10),20) for i in range(81)]
METHODS=("stehfest","dehoog"); PRIMARY=80; VERIFY_DPS=(50,120); VERIFY_TIMES=("1e2","1e6","1e10")
def inv(cache,t,method,dps,kind):
 mp.mp.dps=dps
 f=(lambda s:cache.phi(s)/s) if kind=="cdf" else (lambda s:(1-cache.phi(s))/s)
 a=time.time();v=mp.invertlaplace(f,mp.mpf(t),method=method);return v,time.time()-a
def production(method):
 mp.mp.dps=PRIMARY;c=Cache();rows=[];start=time.time()
 for ts in TIMES:
  F,tf=inv(c,ts,method,PRIMARY,"cdf");S,tsur=inv(c,ts,method,PRIMARY,"survival")
  rows.append({"t_s":ts,"cdf":mp.nstr(F,45),"survival":mp.nstr(S,45),"F_plus_S_minus_1":mp.nstr(F+S-1,20),"cdf_seconds":tf,"survival_seconds":tsur})
 vals=[mp.mpf(x["cdf"]) for x in rows]
 return rows,{"runtime_seconds":time.time()-start,"transform_calls":c.calls,"unique_transform_evaluations":c.misses,"cache_hits":c.calls-c.misses,"raw_bounds":all(0<=x<=1 for x in vals),"raw_monotone":all(vals[i+1]>=vals[i] for i in range(len(vals)-1)),"max_abs_F_plus_S_minus_1":mp.nstr(max(abs(mp.mpf(x["F_plus_S_minus_1"])) for x in rows),20)}
def verify(method,primary_rows):
 by={x["t_s"]:mp.mpf(x["cdf"]) for x in primary_rows};out={}
 # TIMES renders exact powers as e.g. 100.0; locate by numerical equality.
 for target in VERIFY_TIMES:
  key=next(k for k in by if mp.mpf(k)==mp.mpf(target));v80=by[key];vals={"80":mp.nstr(v80,45)}
  for dps in VERIFY_DPS:
   c=Cache();v,_=inv(c,target,method,dps,"cdf");vals[str(dps)]=mp.nstr(v,45)
  f50,f80,f120=[mp.mpf(vals[str(d)]) for d in (50,80,120)]
  out[target]={**vals,"abs_50_80":mp.nstr(abs(f50-f80),20),"abs_80_120":mp.nstr(abs(f80-f120),20)}
 return out
def reconstruct(rows):
 F=[mp.mpf(x["cdf"]) for x in rows];tt=[mp.mpf(x["t_s"]) for x in rows];out={}
 mp.mp.dps=80
 for ss in ("1e-9","1e-8","1e-7","1e-6","1e-5"):
  s=mp.mpf(ss);z=mp.mpf(0)
  for i in range(1,len(F)):
   z+=mp.e**(-s*mp.sqrt(tt[i]*tt[i-1]))*(F[i]-F[i-1])
  ref=phi_raw(s);out[ss]={"reconstructed":mp.nstr(z,35),"direct":mp.nstr(ref,35),"absolute_error":mp.nstr(abs(z-ref),18),"relative_error":mp.nstr(abs(z-ref)/max(abs(ref),mp.mpf("1e-100")),18)}
 return out
def main():
 out={"classification":"IL-3_CONTROLLED_PRODUCTION_EVIDENCE_CANDIDATE","versions":{"python":platform.python_version(),"mpmath":mp.__version__},"grid":{"points":81,"start_s":"1e2","end_s":"1e10","log10_step":"0.1","times_s":TIMES},"primary_dps":PRIMARY,"targeted_precision_times_s":VERIFY_TIMES,"methods":{}}
 for method in METHODS:
  rows,metrics=production(method);out["methods"][method]={"rows":rows,"metrics":metrics,"precision_convergence":verify(method,rows),"forward_reconstruction":reconstruct(rows)}
 A=out["methods"]["stehfest"]["rows"];B=out["methods"]["dehoog"]["rows"];cmp=[]
 for a,b in zip(A,B):
  d=mp.mpf(a["cdf"])-mp.mpf(b["cdf"]);cmp.append({"t_s":a["t_s"],"signed_difference":mp.nstr(d,25),"absolute_difference":mp.nstr(abs(d),25)})
 out["method_comparison"]=cmp
 im=max(range(len(cmp)),key=lambda i:abs(mp.mpf(cmp[i]["absolute_difference"])))
 out["method_comparison_summary"]={"max_absolute_difference":cmp[im]["absolute_difference"],"time_of_max_difference_s":cmp[im]["t_s"]}
 print(json.dumps(out,indent=2))
if __name__=="__main__":main()
