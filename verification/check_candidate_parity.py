#!/usr/bin/env python3
from pathlib import Path
import re,json,sys
md=Path(sys.argv[1]).read_text();tx=Path(sys.argv[2]).read_text()
def blocks_md(s):
 d={}
 for m in re.finditer(r"\$\$\n(.*?)\n\$\$",s,re.S):
  q=re.search(r"\\tag\{(TRISO-[^}]+)\}",m.group(1))
  if q:d[q.group(1)]=m.group(1)
 return d
def blocks_tx(s):
 d={}
 for m in re.finditer(r"\\begin\{equation\}\n(.*?)\n\\end\{equation\}",s,re.S):
  q=re.search(r"\\tag\{(TRISO-[^}]+)\}",m.group(1))
  if q:d[q.group(1)]=m.group(1)
 return d
a,b=blocks_md(md),blocks_tx(tx); missing_md=sorted(set(b)-set(a));missing_tx=sorted(set(a)-set(b))
out={"md_ids":len(a),"tex_ids":len(b),"missing_in_md":missing_md,"missing_in_tex":missing_tx}
print(json.dumps(out));raise SystemExit(1 if missing_md or missing_tx or len(a)!=1481 else 0)
