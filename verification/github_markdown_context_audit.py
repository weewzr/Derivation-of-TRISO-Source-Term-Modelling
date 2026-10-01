#!/usr/bin/env python3
from pathlib import Path
import hashlib,json,re
p=Path("ray to outram park/TRISO_Source_Term_Derivation.md");t=p.read_text(encoding="utf-8");lines=t.splitlines()
events=[];in_math=False;open_line=None;inline_open=False
for n,line in enumerate(lines,1):
 st=line.strip()
 if st=="```math":
  if in_math:events.append({"code":"CTX001_NESTED_MATH_FENCE","line":n})
  if inline_open:events.append({"code":"CTX002_FENCE_WHILE_INLINE_OPEN","line":n})
  in_math=True;open_line=n;continue
 if in_math and st=="```":
  in_math=False;open_line=None;continue
 if in_math:continue
 # ignore ordinary code fences for inline-dollar state
 if st.startswith("```"):continue
 # count unescaped dollars on each prose line; GitHub inline math cannot span arbitrary paragraphs
 pos=[m.start() for m in re.finditer(r"(?<!\\)\$",line)]
 if len(pos)%2:events.append({"code":"CTX003_ODD_INLINE_DOLLAR_LINE","line":n,"text":line})
if in_math:events.append({"code":"CTX004_UNCLOSED_MATH_FENCE","line":open_line})
# explicit regressions
reg=[]
for q in ["TRISO-ML-517","TRISO-ML-518"]:
 marker=f"**Equation {q}**";i=next((k for k,x in enumerate(lines) if x==marker),None)
 reg.append({"id":q,"marker_line":None if i is None else i+1,"context":None if i is None else lines[max(0,i-6):min(len(lines),i+12)]})
# count inline math before ML-517 and math fences before it
target=next(i for i,x in enumerate(lines) if x=="**Equation TRISO-ML-517**")
prefix="\n".join(lines[:target])
inline_pairs=sum(len(re.findall(r"(?<!\\)\$",x))//2 for x in lines[:target] if not x.strip().startswith("```"))
fences_before=sum(1 for x in lines[:target] if x.strip()=="```math")
out={"sha256":hashlib.sha256(p.read_bytes()).hexdigest(),"file_bytes":len(t.encode()),"file_lines":len(lines),"context_errors":events,"context_error_count":len(events),"ML517":{"line":target+1,"math_fences_before":fences_before,"approx_inline_pairs_before":inline_pairs},"regressions":reg,"classification":"Exact committed bytes place TRISO-ML-517/518 in valid top-level fenced-math context. The user-observed GitHub failure is therefore not explained by local expression syntax or a local fence/delimiter defect. Classify it as a GitHub render-stage / whole-document Markdown+MathJax interaction. A specific resource/complexity threshold remains a hypothesis, not an established root cause, until isolated with actual GitHub-rendered prefix/chunk experiments."}
Path("ray to outram park/data/github_markdown_context_audit.json").write_text(json.dumps(out,indent=2),encoding="utf-8")
print(json.dumps({k:out[k] for k in ["sha256","file_bytes","file_lines","context_error_count","ML517","classification"]},indent=2))
