import mpmath as mp, json, time
PREC=[50,80,120]
TIMES=[mp.mpf(10)**(mp.mpf(2)+mp.mpf(i)/10) for i in range(81)]
def invert(F,t,method,dps):
    mp.mp.dps=dps
    return mp.invertlaplace(F,t,method=method)
def bench():
    cases=[
      ("exp",lambda s: mp.mpf("2e-3")/(s+mp.mpf("2e-3")),lambda t: 1-mp.e**(-mp.mpf("2e-3")*t)),
      ("mix",lambda s: mp.mpf(".35")*mp.mpf("1e-3")/(s+mp.mpf("1e-3"))+mp.mpf(".65")*mp.mpf("2e-5")/(s+mp.mpf("2e-5")),lambda t: 1-(mp.mpf(".35")*mp.e**(-mp.mpf("1e-3")*t)+mp.mpf(".65")*mp.e**(-mp.mpf("2e-5")*t))),
      ("long",lambda s: mp.mpf("1e-8")/(s+mp.mpf("1e-8")),lambda t: 1-mp.e**(-mp.mpf("1e-8")*t))
    ]
    out=[]
    ts=[mp.mpf("1e2"),mp.mpf("1e4"),mp.mpf("1e6"),mp.mpf("1e8"),mp.mpf("1e10")]
    for name,Phi,exact in cases:
      for method in ("stehfest","dehoog"):
       for dps in PREC:
        errs=[]
        for t in ts:
         val=invert(lambda s: Phi(s)/s,t,method,dps);errs.append(abs(val-exact(t)))
        out.append({"case":name,"method":method,"dps":dps,"max_abs_error":mp.nstr(max(errs),20)})
    # centered-ball diffusion benchmark: Phi=z/sinh(z), CDF residue series
    R=mp.mpf(1);D=mp.mpf(1)
    Phi=lambda s:(R*mp.sqrt(s/D))/mp.sinh(R*mp.sqrt(s/D))
    # survival for center start: S(t)=2 sum_{n>=1} (-1)^(n+1) exp(-n^2*pi^2 D t/R^2)
    exact=lambda t: 1-2*mp.nsum(lambda n:(-1)**(n+1)*mp.e**(-n*n*mp.pi**2*D*t/R**2),[1,mp.inf])
    ts2=[mp.mpf(".01"),mp.mpf(".03"),mp.mpf(".1"),mp.mpf(".3"),mp.mpf("1")]
    for method in ("stehfest","dehoog"):
     for dps in PREC:
      errs=[]
      for t in ts2:
       val=invert(lambda s: Phi(s)/s,t,method,dps);errs.append(abs(val-exact(t)))
      out.append({"case":"centered_ball","method":method,"dps":dps,"max_abs_error":mp.nstr(max(errs),20)})
    return out
if __name__=="__main__":
 print(json.dumps({"mpmath":mp.__version__,"precision_sequence":PREC,"physical_grid":{"start_s":"1e2","end_s":"1e10","points":81},"benchmarks":bench()},indent=2))
