#!/usr/bin/env python3
from pathlib import Path
import json,sys
def forbidden(s):
 return [(i,ord(c)) for i,c in enumerate(s) if ord(c)<32 and c not in "\n"]
def md(blocks):
 out=["# Serializer compatibility fixture",""]
 for b in blocks:
  out += [b["status"]+" "+b["prose_before"],"",f'**Equation {b["id"]}**',"","$$",b["math"],"$$",""]
 return "\n".join(out)
def tex(blocks):
 out=[r"\\documentclass{article}",r"\\usepackage{amsmath,amssymb,bm}",r"\\begin{document}"]
 for b in blocks:
  out += [b["status"]+" "+b["prose_before"],r"\\begin{equation}",b["math"],r"\\tag{"+b["id"]+"}",r"\\end{equation}"]
 out += [r"\\end{document}"]
 return "\n".join(out)+"\n"
src=Path(sys.argv[1] if len(sys.argv)>1 else "verification/serializer_fixture.json")
out=Path(sys.argv[2] if len(sys.argv)>2 else "verification/tmp_fixture")
d=json.loads(src.read_text()); blocks=d["blocks"]
for b in blocks:
 if forbidden(b["math"]): raise SystemExit("forbidden control in canonical math "+b["id"])
out.mkdir(parents=True,exist_ok=True)
(out/"fixture.md").write_text(md(blocks),encoding="utf-8")
(out/"fixture.tex").write_text(tex(blocks),encoding="utf-8")
print(json.dumps({"blocks":len(blocks),"control_characters":0}))
