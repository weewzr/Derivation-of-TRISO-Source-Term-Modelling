#!/usr/bin/env python3
from pathlib import Path
import json,re,sys
p=Path(sys.argv[1] if len(sys.argv)>1 else "verification/tmp_candidate/TRISO_Source_Term_Derivation.github-math.candidate.md")
t=p.read_text(encoding="utf-8");lines=t.splitlines();errors=[]
for i,b in enumerate(p.read_bytes()):
 if b<32 and b!=10:errors.append({"code":"SER004_CONTROL_CHARACTER","offset":i,"hex":f"0x{b:02x}"})
fences=list(re.finditer(r"(?m)^```math\n(.*?)\n```$",t,re.S))
ids=re.findall(r"(?m)^\*\*Equation (TRISO-[A-Z0-9]+-\d+[A-Z]?)\*\*$",t)
if len(fences)!=1481:errors.append({"code":"MDM001_MATH_FENCE_COUNT","count":len(fences)})
if len(ids)!=1481 or len(set(ids))!=1481:errors.append({"code":"MDM002_STABLE_ID_COUNT","count":len(ids),"unique":len(set(ids))})
if "$$" in t:errors.append({"code":"REG001_LITERAL_DOLLAR_ARCHITECTURE"})
if re.search(r"\\tag\{TRISO-",t):errors.append({"code":"REG007_LITERAL_TAG_ARCHITECTURE"})
for n,m in enumerate(fences):
 body=m.group(1)
 for env in ("cases","aligned","matrix","pmatrix","bmatrix","array"):
  if body.count("\\begin{"+env+"}")!=body.count("\\end{"+env+"}"):errors.append({"code":"ENV_MISMATCH","fence":n,"env":env})
# Known source-level presentation regressions.
for token,code in [("mol m$^{-3}$","REG005_UNIT_FRAGMENTATION"),("(sinmu)","REG008_PLAIN_TEXT_INLINE_VARIABLES"),("$mu","REG008_PLAIN_TEXT_INLINE_VARIABLES"),("$phi","REG008_PLAIN_TEXT_INLINE_VARIABLES"),("(mathbf B)","REG008_PLAIN_TEXT_INLINE_VARIABLES"),("finite-(epsilon)","REG008_PLAIN_TEXT_INLINE_VARIABLES")]:
 if token in t:errors.append({"code":code,"token":token})
print(json.dumps({"math_fences":len(fences),"stable_ids":len(ids),"unique_ids":len(set(ids)),"errors":errors},indent=2));raise SystemExit(1 if errors else 0)
