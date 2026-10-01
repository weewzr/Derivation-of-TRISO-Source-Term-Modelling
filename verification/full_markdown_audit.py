#!/usr/bin/env python3
from pathlib import Path
import json,re,sys,collections
P=Path("ray to outram park/TRISO_Source_Term_Derivation.md")
t=P.read_text(encoding="utf-8"); lines=t.splitlines()
def heading(line):
 for j in range(line-1,-1,-1):
  if re.match(r"^#{1,6} ",lines[j]):return re.sub(r"^#{1,6} ","",lines[j]).strip()
 return ""
blocks=[];i=0;pending=None
while i<len(lines):
 m=re.match(r"^\*\*Equation (TRISO-[A-Z0-9]+-\d+[A-Z]?)\*\*$",lines[i])
 if m:pending=(m.group(1),i+1);i+=1;continue
 if lines[i].strip()=="```math":
  start=i+1;j=i+1;body=[]
  while j<len(lines) and lines[j].strip()!="```":body.append(lines[j]);j+=1
  closed=j<len(lines); end=j+1 if closed else len(lines)
  eid=None;marker_line=None
  if pending and pending[1]>=start-3:eid,marker_line=pending
  raw="\n".join(body)
  env_stack=[];env_errors=[]
  for x in re.finditer(r"\\(begin|end)\{([^}]+)\}",raw):
   typ,env=x.group(1),x.group(2)
   if typ=="begin":env_stack.append(env)
   elif not env_stack or env_stack[-1]!=env:env_errors.append("unexpected_end:"+env)
   else:env_stack.pop()
  env_errors += ["unclosed:"+x for x in reversed(env_stack)]
  cmds=collections.Counter(re.findall(r"\\([A-Za-z]+)",raw))
  depth=0;maxdepth=0
  for ch in raw:
   if ch=="{":depth+=1;maxdepth=max(maxdepth,depth)
   elif ch=="}":depth-=1
  constructs=sorted(set(re.findall(r"\\begin\{([^}]+)\}",raw)))
  blocks.append({"audit_index":f"MATH-{len(blocks)+1:04d}","start_line":start,"end_line":end,"section":heading(i),"equation_id":eid,"marker_line":marker_line,"body":raw,"body_length":len(raw),"brace_depth":maxdepth,"constructs":constructs,"commands":dict(cmds),"closed":closed,"inner_dollars":"$$" in raw,"inner_tag":bool(re.search(r"\\tag\{TRISO-",raw)),"obsolete_delimiters":bool(re.search(r"(?<!\\)\\(?:\(|\)|\[|\])",raw)),"nested_fence":"```" in raw,"environment_errors":env_errors})
  pending=None;i=j+1;continue
 i+=1
ids=re.findall(r"(?m)^\*\*Equation (TRISO-[A-Z0-9]+-\d+[A-Z]?)\*\*$",t)
# control chars
controls=[]
for off,b in enumerate(P.read_bytes()):
 if b<32 and b not in (10,):controls.append({"offset":off,"hex":f"0x{b:02x}"})
# serialization artifacts whole file
arts={}
for name,pat in {"textbackslash":r"\\textbackslash","textasciicircum":r"\\textasciicircum","literal_backslash_n":r"\\n","html":r"&(?:#x20|amp|lt|gt);","replacement":"�"}.items():
 arts[name]=len(re.findall(pat,t))
# prose only scan
inmath=False;inline=[]
patterns={"lost_mu":r"\$mu\b|\bmu_[A-Za-z0-9]|\(sinmu\)|\(cosmu\)|\(tanmu\)","lost_phi":r"\$phi\b|\bphi_[A-Za-z0-9]","lost_greek":r"\$(?:lambda|epsilon|theta|Phi|rho|alpha|beta|gamma)\b","lost_command":r"\((?:mathbf|boldsymbol|partial|nabla)\s+[^)]+\)","finite_epsilon":r"finite-\(epsilon\)","unit_fragment":r"\b(?:mol\s+)?m\$\^[^$]+\$(?:\s+s\$\^[^$]+\$)?"}
i=0
while i<len(lines):
 if lines[i].strip()=="```math":
  i+=1
  while i<len(lines) and lines[i].strip()!="```":i+=1
  i+=1;continue
 for k,p in patterns.items():
  if re.search(p,lines[i]):inline.append({"line":i+1,"class":k,"text":lines[i]})
 i+=1
# environments/cases/aligned/matrices
cases=[];aligned=[];matrices=[]
for b in blocks:
 if "cases" in b["constructs"]:
  cases.append({"index":b["audit_index"],"id":b["equation_id"],"lines":[b["start_line"],b["end_line"]],"rows":b["body"].count("\\\\"),"errors":b["environment_errors"]})
 if "aligned" in b["constructs"]:
  aligned.append({"index":b["audit_index"],"id":b["equation_id"],"lines":[b["start_line"],b["end_line"]],"rows":b["body"].count("\\\\"),"amps":b["body"].count("&"),"errors":b["environment_errors"]})
 if any(x in b["constructs"] for x in ("matrix","pmatrix","bmatrix","array")):
  matrices.append({"index":b["audit_index"],"id":b["equation_id"],"lines":[b["start_line"],b["end_line"]],"constructs":b["constructs"],"errors":b["environment_errors"]})
cmd=collections.Counter()
for b in blocks:cmd.update(b["commands"])
# checkpoints
check=[]
for pct in range(0,101,10):
 idx=round((len(blocks)-1)*pct/100);b=blocks[idx];check.append({"percent":pct,"index":b["audit_index"],"id":b["equation_id"],"section":b["section"],"status":"PASS" if b["closed"] and not b["inner_dollars"] and not b["inner_tag"] and not b["nested_fence"] and not b["environment_errors"] else "FAIL"})
# screenshot IDs
regids=["TRISO-GOV-020","TRISO-GOV-021","TRISO-GOV-022","TRISO-GOV-023","TRISO-GOV-024","TRISO-GOV-101","TRISO-GOV-102","TRISO-ANA-103","TRISO-ANA-104","TRISO-ANA-105","TRISO-ML-460","TRISO-ML-464","TRISO-ML-465","TRISO-ML-493","TRISO-ML-517","TRISO-ML-518"]
reg=[]
for q in regids:
 b=next((x for x in blocks if x["equation_id"]==q),None);reg.append({"id":q,"found":bool(b),"index":b and b["audit_index"],"lines":b and [b["start_line"],b["end_line"]],"inner_dollars":b and b["inner_dollars"],"inner_tag":b and b["inner_tag"],"environment_errors":b and b["environment_errors"],"source_status":"PASS" if b and not b["inner_dollars"] and not b["inner_tag"] and not b["environment_errors"] else "FAIL"})
out={"baseline_commit":"14afc085b50493879932f95444858aa9f8a0e043","source":str(P),"total_blocks":len(blocks),"stable_ids":len(ids),"unique_stable_ids":len(set(ids)),"duplicate_ids":sorted([x for x,c in collections.Counter(ids).items() if c>1]),"numbered_blocks":sum(bool(x["equation_id"]) for x in blocks),"auxiliary_unnumbered_blocks":sum(not x["equation_id"] for x in blocks),"malformed_fences":sum(not x["closed"] for x in blocks),"inner_dollar_blocks":sum(x["inner_dollars"] for x in blocks),"inner_tag_blocks":sum(x["inner_tag"] for x in blocks),"nested_fence_blocks":sum(x["nested_fence"] for x in blocks),"obsolete_delimiter_blocks":sum(x["obsolete_delimiters"] for x in blocks),"environment_mismatch_blocks":sum(bool(x["environment_errors"]) for x in blocks),"cases":cases,"aligned":aligned,"matrices":matrices,"command_inventory":dict(cmd.most_common()),"distinct_commands":len(cmd),"control_characters":controls,"serialization_artifacts":arts,"inline_findings":inline,"checkpoints":check,"regressions":reg,"longest":[{"index":x["audit_index"],"id":x["equation_id"],"length":x["body_length"],"lines":[x["start_line"],x["end_line"]]} for x in sorted(blocks,key=lambda x:x["body_length"],reverse=True)[:10]],"deepest":[{"index":x["audit_index"],"id":x["equation_id"],"depth":x["brace_depth"],"lines":[x["start_line"],x["end_line"]]} for x in sorted(blocks,key=lambda x:x["brace_depth"],reverse=True)[:10]],"first":blocks[0],"middle":blocks[len(blocks)//2],"final":blocks[-1],"blocks":blocks}
Path("ray to outram park/data/full_markdown_math_audit.json").write_text(json.dumps(out,indent=2),encoding="utf-8")
print(json.dumps({k:out[k] for k in ["total_blocks","stable_ids","unique_stable_ids","numbered_blocks","auxiliary_unnumbered_blocks","malformed_fences","inner_dollar_blocks","inner_tag_blocks","nested_fence_blocks","obsolete_delimiter_blocks","environment_mismatch_blocks","distinct_commands"]},indent=2))
print("cases",len(cases),"aligned",len(aligned),"matrices",len(matrices),"controls",len(controls),"inline",len(inline))
print("first",blocks[0]["audit_index"],blocks[0]["equation_id"],blocks[0]["start_line"],blocks[0]["end_line"])
print("middle",blocks[len(blocks)//2]["audit_index"],blocks[len(blocks)//2]["equation_id"],blocks[len(blocks)//2]["start_line"],blocks[len(blocks)//2]["end_line"])
print("final",blocks[-1]["audit_index"],blocks[-1]["equation_id"],blocks[-1]["start_line"],blocks[-1]["end_line"])
