import mpmath as mp
import numpy as np, platform, mpmath
RSTR=["2.125e-4","3.125e-4","3.525e-4","3.875e-4","4.275e-4"]
DSTR=["1.2502982636347968e-13","1e-8","4.062299125614697e-14","9.227773168241615e-17","4.062299125614697e-14"]
SSTR=["0","1e-9","1e-8","1e-7","1e-6","1e-5","1e-4"]
F64_INIT=[0.9999998878466840,0.9550171330454440,0.6756517543904377,0.14763924355510766,0.0040562104165042193,3.2249018605821876e-7,1.067566716213942e-19]
F64_RES=[1.286e-13,1.435e-13,1.536e-13,1.420e-13,5.184e-13,1.138e-10,4.337e-19]
PREC=[50,80,120]

def params():
    return [mp.mpf(x) for x in RSTR],[mp.mpf(x) for x in DSTR],mp.mpf("2e-7")
def p(D,i,j): return D[j]/(D[i]+D[j])
def ratio(x,y):
    if abs(y)<mp.mpf("1e-20"): return x/y
    if y<40: return mp.sinh(x)/mp.sinh(y)
    return mp.exp(x-y)*(1-mp.exp(-2*x))/(1-mp.exp(-2*y))
def ball(r,b,d,s):
    if s==0:return mp.mpf(1)
    l=mp.sqrt(s/d); z=l*b
    if r==0:return z/mp.sinh(z) if z<40 else 2*z*mp.exp(-z)/(1-mp.exp(-2*z))
    return (b/r)*ratio(l*r,z)
def shell(a,b,r,d,s):
    if s==0:return a*(b-r)/(r*(b-a)),b*(r-a)/(r*(b-a))
    l=mp.sqrt(s/d); den=l*(b-a)
    return (a/r)*ratio(l*(b-r),den),(b/r)*ratio(l*(r-a),den)
def matrix(s):
    R,D,de=params(); K=mp.matrix(8); B=mp.matrix(8,1)
    h=ball(R[0]-de,R[0],D[0],s); K[0,0]=h*(1-p(D,0,1));K[0,1]=h*p(D,0,1)
    for st,r in [(1,R[0]+de),(2,R[1]-de)]:
        gm,gp=shell(R[0],R[1],r,D[1],s);K[st,0]=gm*p(D,1,0);K[st,1]=gm*(1-p(D,1,0));K[st,2]=gp*(1-p(D,1,2));K[st,3]=gp*p(D,1,2)
    for st,r in [(3,R[1]+de),(4,R[2]-de)]:
        gm,gp=shell(R[1],R[2],r,D[2],s);K[st,2]=gm*p(D,2,1);K[st,3]=gm*(1-p(D,2,1));K[st,4]=gp*(1-p(D,2,3));K[st,5]=gp*p(D,2,3)
    for st,r in [(5,R[2]+de),(6,R[3]-de)]:
        gm,gp=shell(R[2],R[3],r,D[3],s);K[st,4]=gm*p(D,3,2);K[st,5]=gm*(1-p(D,3,2));K[st,6]=gp*(1-p(D,3,4));K[st,7]=gp*p(D,3,4)
    gm,gp=shell(R[3],R[4],R[3]+de,D[4],s);K[7,6]=gm*p(D,4,3);K[7,7]=gm*(1-p(D,4,3));B[7]=gp
    return K,B
def init_phi(s,x):
    R,D,_=params(); q=(1-p(D,0,1))*x[0]+p(D,0,1)*x[1]
    if s==0:return q
    # high precision adaptive quadrature of the frozen uniform-volume source integral
    f=lambda r: 3*r*r/R[0]**3*ball(r,R[0],D[0],s)
    return mp.quad(f,[0,R[0]])*q
def evaluate(dps):
    mp.mp.dps=dps; out=[]
    for ss in SSTR:
        s=mp.mpf(ss);K,B=matrix(s);A=mp.eye(8)-K;x=mp.lu_solve(A,B)
        res=mp.norm(A*x-B,2)/max(mp.norm(B,2),mp.mpf(1))
        U,sv,V=mp.svd(A); vals=[sv[i] for i in range(len(sv))]; smax=max(vals);smin=min(vals);cond=smax/smin
        init=init_phi(s,x)
        rho=None
        if s==0:
            eig,_=mp.eig(K);rho=max(abs(z) for z in eig)
        out.append((s,x,init,res,cond,smin,smax,rho,A))
    return out
print("VERSIONS python="+platform.python_version()+" numpy="+np.__version__+" mpmath="+mpmath.__version__)
allres={}
for dps in PREC:
    print("PRECISION dps="+str(dps));allres[dps]=evaluate(dps)
    for idx,(s,x,ini,res,cond,smin,smax,rho,A) in enumerate(allres[dps]):
        rr="NOT_EVALUATED" if rho is None else mp.nstr(rho,35)
        gap="NOT_EVALUATED" if rho is None else mp.nstr(1-rho,35)
        print("HP dps={} s={} init={} residual={} kappa2={} sigma_min={} sigma_max={} rho={} gap={}".format(dps,mp.nstr(s,8),mp.nstr(ini,35),mp.nstr(res,8),mp.nstr(cond,12),mp.nstr(smin,12),mp.nstr(smax,12),rr,gap))
# comparison uses 80 digit reference
for i,ss in enumerate(SSTR):
    s,x,ini,res,cond,smin,smax,rho,A=allres[80][i]
    a=np.array([[float(A[r,c]) for c in range(8)] for r in range(8)])
    sv=np.linalg.svd(a,compute_uv=False); k2=sv[0]/sv[-1]
    absd=abs(mp.mpf(str(F64_INIT[i]))-ini); reld=absd/max(abs(ini),mp.mpf("1e-100"))
    conv50=abs(allres[50][i][2]-ini);conv120=abs(allres[120][i][2]-ini)
    print("COMPARE s={} f64_init={:.17e} hp80_init={} absdiff={} reldiff={} f64_res={:.6e} hp80_res={} hp50_80={} hp80_120={} numpy_kappa2={:.16e} numpy_sigma_min={:.16e}".format(ss,F64_INIT[i],mp.nstr(ini,35),mp.nstr(absd,12),mp.nstr(reld,12),F64_RES[i],mp.nstr(res,8),mp.nstr(conv50,8),mp.nstr(conv120,8),k2,sv[-1]))
if max(abs(allres[80][0][1][i]-1) for i in range(8))>=mp.mpf("1e-9"): raise SystemExit("normalization state criterion failed")
if abs(allres[80][0][2]-1)>=mp.mpf("1e-9"): raise SystemExit("normalization init criterion failed")
if max(r[3] for r in allres[80])>=mp.mpf("1e-10"): raise SystemExit("residual criterion failed")
if not all(mp.isfinite(r[2]) and 0<=r[2]<=1+mp.mpf("1e-10") for r in allres[80]): raise SystemExit("bounds failed")
print("ORIGINAL_GATES high_precision=PASS")
