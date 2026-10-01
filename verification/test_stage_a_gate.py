#!/usr/bin/env python3
from pathlib import Path
import tempfile
from stage_a_gate import validate

def run(text):
 with tempfile.TemporaryDirectory() as d:
  p=Path(d)/"case.md";p.write_text(text,encoding="utf-8")
  return validate([p])

for token in ("\\(","\\)","\\[","\\]"):
 e=run("prose "+token+" x")
 assert any(x["code"]=="SER005_LITERAL_LATEX_DELIMITER" and x["token"]==token for x in e),(token,e)

assert run("valid inline $D_i$")==[], run("valid inline $D_i$")
assert run("$$\nD_i\n$$")==[], run("$$\nD_i\n$$")
# A TeX line break followed by parentheses is not a literal inline delimiter.
assert run("$$\n\\begin{aligned}\nA&=B,\\\\\n(I-K)\\Phi&=B.\n\\end{aligned}\n$$")==[]
print("PASS: Stage-A validator delimiter self-tests")
