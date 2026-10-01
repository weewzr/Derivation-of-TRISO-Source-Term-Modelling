#!/usr/bin/env python3
from pathlib import Path
import json,re,sys
SRC=Path(sys.argv[1] if len(sys.argv)>1 else "ray to outram park/TRISO_Source_Term_Derivation.md")
OUT=Path(sys.argv[2] if len(sys.argv)>2 else "ray to outram park/data/markdown_math_expressions.json")
text=SRC.read_text(encoding="utf-8"); lines=text.splitlines()
def heading(n):
 for j in range(n-1,-1,-1):
  if re.match(r"^#{1,6}\\s+",lines[j]): return re.sub(r"^#{1,6}\\s+","",lines[j]).strip()
 return ""
def eid(a,b,raw):
 m=re.search(r"\\\\tag\\{(TRISO-[^}]+)\\}",raw)
 if m:return m.group(1)
 for d in range(12):
  for j in (a-1-d,b-1+d):
   if 0<=j<len(lines):
    q=re.search(r"TRISO-[A-Z0-9]+-\\d+[A-Z]?",lines[j])
    if q:return q.group(0)
 return None
expr=[]; findings=[]; fence=False;i=0
while i<len(lines):
 st=lines[i].strip()
 if st.startswith("~~~") or st.startswith("```"): fence=not fence;i+=1;continue
 if st=="```math":
  start=i+1; body=[];j=i+1
  while j<len(lines) and lines[j].strip()!="```": body.append(lines[j]);j+=1
  if j>=len(lines): findings.append({"class":"unmatched_display_delimiter","line":i+1});break
  raw="\n".join(body); e={"index":len(expr),"type":"display","start_line":start,"end_line":j+1,"section":heading(i),"raw":raw,"equation_id":eid(i+1,j+1,raw)};expr.append(e);i=j+1;continue
 if not fence:
  pos=[m.start() for m in re.finditer(r"(?<!\\\\)\\$",lines[i])]
  if len(pos)%2: findings.append({"class":"odd_inline_delimiter","line":i+1,"text":lines[i]})
  for k in range(0,len(pos)-1,2):
   raw=lines[i][pos[k]+1:pos[k+1]];expr.append({"index":len(expr),"type":"inline","start_line":i+1,"end_line":i+1,"section":heading(i),"raw":raw,"equation_id":eid(i+1,i+1,raw)})
 i+=1
for e in expr:
 for env in ("cases","aligned","matrix","pmatrix","bmatrix","array"):
  a=len(re.findall(r"\\\\begin\\{"+env+r"\\}",e["raw"]));b=len(re.findall(r"\\\\end\\{"+env+r"\\}",e["raw"]))
  if a!=b: findings.append({"class":env+"_environment_mismatch","line":e["start_line"],"equation_id":e["equation_id"],"begin":a,"end":b})
 if re.search(r"\\\\begin\\{[^}]+\\}.*?\n\\s*\n.*?\\\\end\\{[^}]+\\}",e["raw"],re.S): findings.append({"class":"blank_line_inside_environment","line":e["start_line"],"equation_id":e["equation_id"]})
for cls,pat in {"literal_parenthesis_math":r"\\\\\\\\\\(|\\\\\\\\\\)","literal_bracket_math":r"\\\\\\\\\\[|\\\\\\\\\\]","textbackslash":r"\\\\textbackslash","textasciicircum":r"\\\\textasciicircum","html_entity":r"&(?:amp|lt|gt|#x20);","unit_split_regression":r"mol m\\$[\\^\\{]"}.items():
 for n,line in enumerate(lines,1):
  if re.search(pat,line): findings.append({"class":cls,"line":n,"text":line[:500]})
cmd={}
for e in expr:
 for c in re.findall(r"\\\\([A-Za-z]+)",e["raw"]):cmd[c]=cmd.get(c,0)+1
ids=re.findall(r"(?m)^\\*\\*Equation (TRISO-[A-Z0-9]+-\\d+[A-Z]?)\\*\\*$",text)
payload={"source":str(SRC),"line_count":len(lines),"expressions":expr,"structural_findings":findings,"command_inventory":dict(sorted(cmd.items())),"stable_tag_count":len(ids),"stable_unique_tag_count":len(set(ids)),"duplicate_tags":sorted({x for x in ids if ids.count(x)>1})}
OUT.parent.mkdir(parents=True,exist_ok=True);OUT.write_text(json.dumps(payload,indent=2),encoding="utf-8")
print(json.dumps({"expressions":len(expr),"structural_findings":len(findings),"stable_tags":len(ids),"unique_tags":len(set(ids))}))
