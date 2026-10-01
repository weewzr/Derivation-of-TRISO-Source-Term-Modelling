#!/usr/bin/env python3
"""Structural validator for TRISO GitHub Markdown mathematics.

This does not prove visual correctness. It detects serialization defects that
must fail before rendered visual inspection.
"""
from pathlib import Path
import re, sys

path = Path(sys.argv[1] if len(sys.argv) > 1 else "ray to outram park/TRISO_Source_Term_Derivation.md")
text = path.read_text(encoding="utf-8")
lines = text.splitlines()
errors=[]

# Display delimiter state and inline-dollar parity outside displays.
in_display=False
for no,line in enumerate(lines,1):
    if line.strip()=="$$":
        in_display=not in_display
        continue
    if not in_display:
        dollars=len(re.findall(r"(?<!\\)\$",line))
        if dollars%2:
            errors.append(f"{no}: odd inline-dollar count")
if in_display:
    errors.append("unclosed display-math delimiter")

# Environment balance, including cases/aligned/matrices.
beg=re.findall(r"\\begin\{([^}]+)\}",text)
end=re.findall(r"\\end\{([^}]+)\}",text)
for env in sorted(set(beg+end)):
    if beg.count(env)!=end.count(env):
        errors.append(f"environment {env}: begin={beg.count(env)} end={end.count(env)}")

# Stable IDs: production manuscript must preserve uniqueness.
ids=re.findall(r"TRISO-[A-Z0-9]+-\d+[A-Z]?",text)
tags=re.findall(r"\\tag\{(TRISO-[^}]+)\}",text)
if path.name=="TRISO_Source_Term_Derivation.md":
    if len(tags)!=len(set(tags)):
        errors.append("duplicate tagged stable equation IDs")
    if len(set(tags))!=1481:
        errors.append(f"expected 1481 unique tagged equation IDs in current baseline, found {len(set(tags))}")

for token in (r"\textbackslash",r"\textasciicircum","&#x20;","&amp;","&lt;","&gt;"):
    if token in text:
        errors.append(f"suspicious serialization token: {token}")

# Known screenshot regressions in source form.
if "mol m$^{-3}$" in text:
    errors.append("known unit-rendering regression mol m$^{-3}$")
if re.search(r"\$\$\s*\n\s*\$\$",text):
    errors.append("empty display block")

if errors:
    print("FAIL")
    for e in errors: print(e)
    raise SystemExit(1)
print("PASS: structural checks only; rendered visual validation still required")
