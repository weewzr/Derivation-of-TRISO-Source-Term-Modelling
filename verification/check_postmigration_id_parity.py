#!/usr/bin/env python3
from pathlib import Path
import re,json,sys
md=Path(sys.argv[1] if len(sys.argv)>1 else "ray to outram park/TRISO_Source_Term_Derivation.md").read_text(encoding="utf-8")
tx=Path(sys.argv[2] if len(sys.argv)>2 else "ray to outram park/TRISO_Source_Term_Derivation.tex").read_text(encoding="utf-8")
md_ids=re.findall(r"(?m)^\*\*Equation (TRISO-[A-Z0-9]+-\d+[A-Z]?)\*\*$",md)
tx_ids=re.findall(r"\\tag\{(TRISO-[^}]+)\}",tx)
out={"markdown_total":len(md_ids),"markdown_unique":len(set(md_ids)),"latex_total":len(tx_ids),"latex_unique":len(set(tx_ids)),"missing_in_markdown":sorted(set(tx_ids)-set(md_ids)),"missing_in_latex":sorted(set(md_ids)-set(tx_ids)),"markdown_duplicates":sorted([x for x in set(md_ids) if md_ids.count(x)>1]),"latex_duplicates":sorted([x for x in set(tx_ids) if tx_ids.count(x)>1])}
print(json.dumps(out,indent=2))
ok=out["markdown_total"]==out["markdown_unique"]==out["latex_total"]==out["latex_unique"]==1481 and not out["missing_in_markdown"] and not out["missing_in_latex"] and not out["markdown_duplicates"] and not out["latex_duplicates"]
raise SystemExit(0 if ok else 1)
