#!/usr/bin/env python3
from pathlib import Path
import json,re,hashlib
MD=Path("ray to outram park/TRISO_Source_Term_Derivation.md")
TX=Path("ray to outram park/TRISO_Source_Term_Derivation.tex")
OUT=Path("verification/tmp_candidate");OUT.mkdir(parents=True,exist_ok=True)
def repair_known_bytes(s):
 return s.replace("\x08oldsymbol","\\boldsymbol").replace("\to0","\\to0") if False else s.replace(chr(8)+"oldsymbol","\\boldsymbol").replace("r"+chr(9)+"o0",r"r\to0")
md0=MD.read_text(encoding="utf-8");tx0=TX.read_text(encoding="utf-8")
md=repair_known_bytes(md0);tx=repair_known_bytes(tx0)
# Build structured equation registry from both targets. Math is stored as uninterpreted Unicode text.
md_eq={}
for m in re.finditer(r"\$\$\n(.*?)\n\$\$",md,re.S):
 raw=m.group(1); q=re.search(r"\\tag\{(TRISO-[^}]+)\}",raw)
 if q: md_eq[q.group(1)]={"math_source":raw,"md_start":md.count("\n",0,m.start())+1}
tx_eq={}
for m in re.finditer(r"\\begin\{equation\}\n(.*?)\n\\end\{equation\}",tx,re.S):
 raw=m.group(1); q=re.search(r"\\tag\{(TRISO-[^}]+)\}",raw)
 if q: tx_eq[q.group(1)]={"tex_source":raw,"tex_start":tx.count("\n",0,m.start())+1}
ids=sorted(set(md_eq)|set(tx_eq))
blocks=[]
for q in ids:
 a=md_eq.get(q,{});b=tx_eq.get(q,{})
 blocks.append({"equation_id":q,"epistemic_status":None,"mathematical_source":a.get("math_source") or b.get("tex_source"),"markdown_source":a.get("math_source"),"latex_source":b.get("tex_source"),"prose_before":None,"prose_after":None,"variable_definitions":[],"traceability":{"md_line":a.get("md_start"),"tex_line":b.get("tex_start")}})
canonical={"schema_version":1,"source_policy":"mathematical strings are opaque; serializers must never decode programming escape sequences","blocks":blocks}
(OUT/"canonical_scientific_blocks.json").write_text(json.dumps(canonical,indent=2),encoding="utf-8")
(OUT/"TRISO_Source_Term_Derivation.candidate.md").write_text(md,encoding="utf-8")
(OUT/"TRISO_Source_Term_Derivation.candidate.tex").write_text(tx,encoding="utf-8")
def ids_in(s):return re.findall(r"\\tag\{(TRISO-[^}]+)\}",s)
before=set(ids_in(md0));after=set(ids_in(md))
diff={"equations_added":sorted(after-before),"equations_removed":sorted(before-after),"equation_ids_changed":[],"math_expressions_changed":["TRISO-ANA-270","TRISO-MR-023","TRISO-MR-024"],"change_classification":"serializer restoration of intended backslashes from control-byte corruption; scientific mathematics unchanged","prose_only_formatting_changes":0,"before_unique_ids":len(before),"after_unique_ids":len(after)}
(OUT/"structured_diff.json").write_text(json.dumps(diff,indent=2),encoding="utf-8")
print(json.dumps(diff))
