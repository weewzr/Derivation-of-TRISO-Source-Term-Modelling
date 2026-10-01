#!/usr/bin/env python3
from pathlib import Path
import json,re,sys
paths=[Path(p) for p in sys.argv[1:]] or [Path("verification/tmp_fixture/fixture.md"),Path("verification/tmp_fixture/fixture.tex")]
errors=[]
for p in paths:
 b=p.read_bytes()
 for i,x in enumerate(b):
  if x<32 and x not in (10,):
   errors.append({"code":"SER004_CONTROL_CHARACTER","file":str(p),"offset":i,"hex":f"0x{x:02x}"})
 t=b.decode("utf-8")
 if "\\x08" in repr(t) or "\\x09" in repr(t): pass
 if p.suffix==".md":
  if t.count("$$")%2: errors.append({"code":"MD002_UNMATCHED_DISPLAY_DELIMITER","file":str(p)})
  for env,code in [("cases","ENV001_UNMATCHED_CASES"),("aligned","ENV002_UNMATCHED_ALIGNED"),("matrix","ENV003_UNMATCHED_MATRIX"),("bmatrix","ENV003_UNMATCHED_MATRIX"),("pmatrix","ENV003_UNMATCHED_MATRIX")]:
   if t.count("\\begin{"+env+"}")!=t.count("\\end{"+env+"}"):errors.append({"code":code,"file":str(p)})
  for token,code in [("\\textbackslash","SER001_TEXTBACKSLASH_ARTIFACT"),("\\textasciicircum","SER002_TEXTASCIICIRCUM_ARTIFACT"),("\\(","SER005_LITERAL_LATEX_DELIMITER"),("\\)","SER005_LITERAL_LATEX_DELIMITER")]:
   if token in t:errors.append({"code":code,"file":str(p),"token":token})
print(json.dumps({"files":[str(p) for p in paths],"errors":errors},indent=2))
raise SystemExit(1 if errors else 0)
