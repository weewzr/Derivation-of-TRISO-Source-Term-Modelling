# Full GitHub Markdown Rendering Audit — all mathematical blocks

Baseline canonical Markdown audited end-to-end from the first through final `math` fence. Audit-only pass; scientific mathematics and canonical LaTeX were not modified.

## Coverage proof

- Total mathematical blocks: **1498**.
- Stable equation IDs: **1481 total / 1481 unique**.
- Intentional auxiliary unnumbered mathematical blocks: **17**.
- First: **MATH-0001 / TRISO-GOV-020**, Section 2, lines 52–54.
- Middle: **MATH-0750 / TRISO-DIS-420**, Section 16.4, lines 9277–9285.
- Final: **MATH-1498 / TRISO-VER-308**, Section 26.2, lines 18600–18606.

### Distributed checkpoints

| Coverage | Block | Stable ID | Section | Source result |
|---:|---|---|---|---|
| 0% | MATH-0001 | TRISO-GOV-020 | 2. Conservation from a control volume | PASS |
| 10% | MATH-0151 | TRISO-ANA-133 | 7.2 Independent global generation/release balance | PASS |
| 20% | MATH-0300 | TRISO-SL-227 | 10.9 Modal coefficient projection | PASS |
| 30% | MATH-0450 | TRISO-ML-439 | 12.5 Transform concentration continuity | PASS |
| 40% | MATH-0600 | TRISO-DIS-108 | 13.3 Define the temporal mesh | PASS |
| 50% | MATH-0750 | TRISO-DIS-420 | 16.4 Surface first-derivative approximation | PASS |
| 60% | MATH-0899 | TRISO-FV-110 | 18.2 Integrate conservation over one spherical cell | PASS |
| 70% | MATH-1049 | TRISO-FV-254 | 18.17 Outermost-cell row | PASS |
| 80% | MATH-1199 | TRISO-ACC-190 | 18A.7.3 Add exact concentration drops | PASS |
| 90% | MATH-1348 | TRISO-FPT-006 | 21.1 Diffusion process and backward generator | PASS |
| 100% | MATH-1498 | TRISO-VER-308 | 26.2 Genuine three-layer multistate verification | PASS |

## Structural results

- Malformed/unclosed fences: **0**.
- Fences containing inner `$$`: **0**.
- Fences containing Markdown `\tag{TRISO-...}`: **0**.
- Nested Markdown fences: **0**.
- Environment mismatch blocks: **0**.
- Orphan/duplicate stable IDs: **0 duplicates**; 17 unnumbered displays are intentionally auxiliary rather than numbered equations.
- Distinct TeX/MathJax commands observed: **82**.
- Cases environments: **1**, source-structure failures **0**.
- Aligned environments: **0**.
- Matrix-like environments: **2**, source-structure failures **0**.
- Forbidden serialization artifacts `\textbackslash`, `\textasciicircum`, HTML entities, replacement characters: **0**.
- Forbidden control characters: source byte audit expected **0** under the existing Stage-A gate.

## Inline-prose and unit audit

The broad suspicious-pattern scan produced 13 candidate lines. Manual classification shows all 13 are **false positives** caused by the detector matching substrings inside already-correct inline math such as `$\mu_n$`, `$\phi_n$`, and `$\phi_i^{(m)}$`. No new malformed inline-math class is established by these findings.

Known fragmented-unit patterns: **0**.

## Screenshot regression equations

Source structure PASS for:
TRISO-GOV-020, GOV-021, GOV-022, GOV-023, GOV-024, GOV-101, GOV-102, ANA-103, ANA-104, ANA-105, ML-460, ML-464, ML-465, ML-493.

Each is present in a `math` fence with its stable ID outside the fence, with no inner `$$`, no inner `\tag`, and no source-level environment mismatch.

## Complexity coverage

Ten longest blocks by body length:
TRISO-ML-468, TRISO-FV-258, TRISO-GOV-118, TRISO-ML-552, TRISO-SPH-021, TRISO-ANA-212, TRISO-FV-111, TRISO-ML-492, TRISO-FV-300, TRISO-ANA-210.

All are included in the full structural inventory.

The machine-readable inventory contains one record for every block, including full mathematical body, line range, section, stable ID where applicable, constructs, commands, and source validation status.

## Parser status

A MathJax 3.2.2 all-block parser has been added to the fast audit path. Its executed PASS/FAIL result must come from the audit workflow; it is not inferred from the source scan. KaTeX remains secondary and must not be described as GitHub render validation.

## Files

- Machine-readable: `ray to outram park/data/full_markdown_math_audit.json`
- Full audit implementation: `verification/full_markdown_audit.py`
- MathJax all-block parser: `verification/mathjax_full_audit.mjs`

## Scientific status

Scientific equations changed: **0**.

Markdown mathematical bodies changed: **0**.

Canonical LaTeX changed: **NO**.

R2-WOS-02: **OPEN**.

R2-B01: **OPEN**.
