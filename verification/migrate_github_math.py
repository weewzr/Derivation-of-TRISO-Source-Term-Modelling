#!/usr/bin/env python3
from pathlib import Path
import json,re,sys
SRC=Path(sys.argv[1] if len(sys.argv)>1 else "ray to outram park/TRISO_Source_Term_Derivation.md")
OUT=Path(sys.argv[2] if len(sys.argv)>2 else "verification/tmp_candidate/TRISO_Source_Term_Derivation.github-math.candidate.md")
DIFF=Path(sys.argv[3] if len(sys.argv)>3 else "verification/tmp_candidate/github_math_migration_diff.json")
text=SRC.read_text(encoding="utf-8")
pat=re.compile(r"^\$\$\n(.*?)\n\$\$$",re.M|re.S)
old={}
def convert(m):
 body=m.group(1)
 q=re.search(r"(?m)^\\tag\{(TRISO-[^}]+)\}\s*$",body)
 if not q:
  raise SystemExit("display block without stable TRISO tag near: "+body[:120])
 eid=q.group(1)
 if eid in old: raise SystemExit("duplicate ID "+eid)
 math=re.sub(r"(?m)^\\tag\{TRISO-[^}]+\}\s*\n?","",body).rstrip()
 old[eid]=math
 return f"**Equation {eid}**\n\n```math\n{math}\n```"
candidate=pat.sub(convert,text)
# Contextual introductory inline normalization explicitly required by the GitHub migration.
candidate=candidate.replace("Let r be distance from the particle centre, t be time, c_i(r,t) be concentration in layer i, and D_i be the layer diffusivity.",
 "Let $r$ be distance from the particle centre, $t$ be time, $c_i(r,t)$ be concentration in layer $i$, and $D_i$ be the layer diffusivity.")
new={}
for m in re.finditer(r"(?m)^\*\*Equation (TRISO-[^*]+)\*\*\n\n```math\n(.*?)\n```",candidate,re.S):
 new[m.group(1)]=m.group(2)
ids_old=set(old);ids_new=set(new)
changed=[q for q in sorted(ids_old&ids_new) if old[q]!=new[q]]
report={"equations_compared":len(ids_old&ids_new),"equations_unchanged":len(ids_old&ids_new)-len(changed),"unexpected_mathematical_changes":changed,"ids_lost":sorted(ids_old-ids_new),"ids_added":sorted(ids_new-ids_old),"old_ids":len(ids_old),"new_ids":len(ids_new),"old_display_dollars_remaining":candidate.count("$$"),"markdown_tags_remaining":len(re.findall(r"\\tag\{TRISO-",candidate)),"math_fence_count":candidate.count("```math")}
OUT.parent.mkdir(parents=True,exist_ok=True);OUT.write_text(candidate,encoding="utf-8");DIFF.write_text(json.dumps(report,indent=2),encoding="utf-8");print(json.dumps(report))
if report["old_ids"]!=1481 or report["new_ids"]!=1481 or changed or report["ids_lost"] or report["ids_added"] or report["old_display_dollars_remaining"] or report["markdown_tags_remaining"] or report["math_fence_count"]!=1481: raise SystemExit(1)
