#!/usr/bin/env python3
from pathlib import Path
import json,sys

def validate(paths):
 errors=[]
 for p in paths:
  b=p.read_bytes()
  for i,x in enumerate(b):
   if x<32 and x not in (10,):
    errors.append({"code":"SER004_CONTROL_CHARACTER","file":str(p),"offset":i,"hex":f"0x{x:02x}"})
  t=b.decode("utf-8")
  if p.suffix==".md":
   if t.count("$$")%2:
    errors.append({"code":"MD002_UNMATCHED_DISPLAY_DELIMITER","file":str(p)})
   for env,code in [("cases","ENV001_UNMATCHED_CASES"),("aligned","ENV002_UNMATCHED_ALIGNED"),("matrix","ENV003_UNMATCHED_MATRIX"),("bmatrix","ENV003_UNMATCHED_MATRIX"),("pmatrix","ENV003_UNMATCHED_MATRIX")]:
    if t.count("\\begin{"+env+"}")!=t.count("\\end{"+env+"}"):
     errors.append({"code":code,"file":str(p)})
   for token,code in [("\\textbackslash","SER001_TEXTBACKSLASH_ARTIFACT"),("\\textasciicircum","SER002_TEXTASCIICIRCUM_ARTIFACT")]:
    if token in t:
     errors.append({"code":code,"file":str(p),"token":token})
   # Literal LaTeX math delimiters are forbidden in the GitHub Markdown target.
   # Use literal scanning, but ignore a token when its backslash is itself
   # immediately escaped (e.g. TeX line break \\\\ followed by parentheses).
   for token in ("\\(","\\)","\\[","\\]"):
    start=0
    while True:
     i=t.find(token,start)
     if i<0: break
     escaped=i>0 and t[i-1]=="\\"
     if not escaped:
      errors.append({"code":"SER005_LITERAL_LATEX_DELIMITER","file":str(p),"token":token,"offset":i})
     start=i+len(token)
 return errors

def main():
 paths=[Path(p) for p in sys.argv[1:]] or [Path("verification/tmp_fixture/fixture.md"),Path("verification/tmp_fixture/fixture.tex")]
 try:
  errors=validate(paths)
 except Exception as exc:
  print(json.dumps({"status":"VALIDATOR_INTERNAL_ERROR","error":repr(exc)},indent=2))
  return 2
 print(json.dumps({"status":"VALIDATION_FAILURE" if errors else "PASS","files":[str(p) for p in paths],"errors":errors},indent=2))
 return 1 if errors else 0

if __name__=="__main__":
 raise SystemExit(main())
