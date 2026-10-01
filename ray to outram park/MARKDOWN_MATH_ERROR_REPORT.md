# Markdown / Manuscript Serialization Error Inventory

Inventory baseline: GitHub Actions run **36885479913**, head SHA `c6da4b8a3f8aba39818d8c8d58d64d159ebd2ab6`.

This is an inventory-only document. No manuscript equation or scientific content was changed in producing it.

## Executed validation evidence

- Markdown extraction: **1498 mathematical expressions**.
- Strict web-math parser: **KaTeX 0.16.22**, `throwOnError=true`, `strict="error"`.
- KaTeX PASS: **1498**.
- KaTeX FAIL: **0**.
- Existing structural extractor findings: **0**, but this is not sufficient for rendered validity.
- Stable equation IDs in the live Markdown: **1481 unique**; **1481 tagged display equations**. The run's reported `stable_tags=0` was a validator extraction bug and is superseded by the direct inventory.
- LuaLaTeX: **FAIL** on forbidden control character.

## Control-character inventory

| Code | File | Byte offset | Line | Nearest equation ID | Context / interpretation |
|---|---|---:|---:|---|---|
| SER003_PROGRAMMING_ESCAPE_CORRUPTION / SER004_CONTROL_CHARACTER | `TRISO_Source_Term_Derivation.md` | 39170 | 2439 | TRISO-ANA-270 | Intended `r\to0`; `\t` was serialized as ASCII TAB (0x09), yielding `r<TAB>o0`. |
| SER003_PROGRAMMING_ESCAPE_CORRUPTION / SER004_CONTROL_CHARACTER | `TRISO_Source_Term_Derivation.md` | 236389 | 16886 | TRISO-MR-023 | Intended `\boldsymbol`; `\b` became BACKSPACE (0x08), yielding `^^Holdsymbol`. |
| SER003_PROGRAMMING_ESCAPE_CORRUPTION / SER004_CONTROL_CHARACTER | `TRISO_Source_Term_Derivation.md` | 236531 | 16897 | TRISO-MR-024 | Second intended `\boldsymbol` corrupted identically. |
| SER003_PROGRAMMING_ESCAPE_CORRUPTION / SER004_CONTROL_CHARACTER | `TRISO_Source_Term_Derivation.tex` | 52415 | 2681 | TRISO-ANA-270 | Same 0x09 `\to` corruption propagated into LaTeX. |
| SER003_PROGRAMMING_ESCAPE_CORRUPTION / SER004_CONTROL_CHARACTER | `TRISO_Source_Term_Derivation.tex` | 316562 | 18363 | TRISO-MR-023 | Same 0x08 `\boldsymbol` corruption propagated into LaTeX; this is the fatal run-36885479913 error. |
| SER003_PROGRAMMING_ESCAPE_CORRUPTION / SER004_CONTROL_CHARACTER | `TRISO_Source_Term_Derivation.tex` | 316754 | 18375 | TRISO-MR-024 | Second propagated 0x08 corruption. |

No NUL (0x00), vertical tab (0x0B), form feed (0x0C), or carriage return (0x0D) was found in the audited manuscript targets. No forbidden control character was found in `references.bib`, `validate_markdown_math.py`, or `katex_strict_lint.mjs`.

## Exact escape-sequence mechanism

The corruption is diagnostic of ordinary programming-language string escape interpretation before serialization:

- source text intended as `\boldsymbol` was represented in a non-raw string, so the leading `\b` became byte 0x08 BACKSPACE;
- source text intended as `\to` was represented in a non-raw string, so `\t` became byte 0x09 TAB.

The same bytes occur in both Markdown and LaTeX, proving that LuaLaTeX did not create them. Repository history shows Markdown was used as the detailed working manuscript and LaTeX was repeatedly regenerated from it (e.g. commit `7c6032742d571a72a86b1a774bfb1e87524e5a1c`). The responsible **generation class** is therefore a string-based manuscript generation/normalization step using interpreted string literals. The exact historical ephemeral script source is not retained as a current repository file, so attribution to a specific filename is not supported by the repository.

## Three validation layers

### Layer 1 — Markdown structure

Current automated structural extractor at run 36885479913 reported 0 findings. This is **not a rendered PASS** because it did not detect the three control-character corruptions and its stable-tag counter was defective.

Known Layer-1 serialization/control failures affecting Markdown: **3 occurrences / 3 nearby equation contexts**.

### Layer 2 — web-math parser

KaTeX 0.16.22 scanned **1498 expressions**.

- PASS: **1498**
- FAIL: **0**
- TEX001_WEB_MATH_PARSE_ERROR: **0**

This demonstrates why Layer 2 cannot override Layers 1 or 3.

### Layer 3 — rendered regression

The user's observed GitHub-rendered failures remain active acceptance failures:

- REG001_LITERAL_DOLLAR_VISIBLE — observed.
- REG002_RAW_TEX_VISIBLE — observed.
- REG003_CASES_RENDER_FAILURE — observed (`Missing \end{cases}`).
- REG004_VERTICAL_FRAGMENTATION — observed.
- REG005_UNIT_FRAGMENTATION — observed (`mol m$^{-3}$`).

Rendered regression status: **FAIL (5 failure classes)**. A per-equation count cannot be inferred from screenshots alone and is therefore not fabricated.

## Environment inventory

The strict KaTeX pass reported no parser failures for extracted `cases`, `aligned`, matrix-family, or other expressions.

Automated environment mismatch counts from run 36885479913:

- ENV001_UNMATCHED_CASES: **0 detected by source extractor**
- ENV002_UNMATCHED_ALIGNED: **0 detected**
- ENV003_UNMATCHED_MATRIX: **0 detected**

Nevertheless REG003 remains FAIL because the rendered GitHub evidence demonstrates that source-level environment pairing is insufficient when Markdown block boundaries split rendering.

## Undefined equation references from executed LaTeX run

| Code | Reference | Audit result |
|---|---|---|
| REF001_UNDEFINED_EQUATION_REFERENCE | `eq:triso-fpt-002` | Equation and exact label exist at LaTeX line 16963. Warning occurred on first pass before cross-reference state existed; not a missing equation. |
| REF001_UNDEFINED_EQUATION_REFERENCE | `eq:triso-fpt-004` | Equation and exact label exist at line 16983; first-pass unresolved reference. |
| REF001_UNDEFINED_EQUATION_REFERENCE | `eq:triso-fpt-005` | Equation and exact label exist at line 16991; first-pass unresolved reference. |
| REF001_UNDEFINED_EQUATION_REFERENCE | `eq:triso-wos-005` | Equation and exact label exist at line 17095; first-pass unresolved reference. |
| REF001_UNDEFINED_EQUATION_REFERENCE | `eq:triso-fpt-022` | Equation and exact label exist at line 17309; first-pass unresolved reference. |
| REF001_UNDEFINED_EQUATION_REFERENCE | `eq:triso-fpt-040` | Equation and exact label exist at line 17473; first-pass unresolved reference. |

Classification for all six: **A — equation exists and label is present**. The fatal compile stopped before later LaTeX passes could resolve the references. No equation is missing and no equation should be invented.

## Undefined citation

REF002_UNDEFINED_CITATION: `NIST_DLMF`.

The manuscript genuinely cites NIST DLMF for Sturm–Liouville provenance. A real `@misc{NIST_DLMF,...}` entry already exists in `references.bib`, identifying the National Institute of Standards and Technology, *Digital Library of Mathematical Functions*, `https://dlmf.nist.gov/`, with sections 1.13 and 3.7 noted.

Classification: **bibliography entry exists; first-pass citation unresolved because BibTeX/later LaTeX passes were never reached after the fatal pass-1 control-character error**. No bibliography fabrication is needed.

## Root-cause clusters

### RC1 — programming-language escape corruption

Occurrences: **6 physical file occurrences**, representing **3 source defects propagated to two presentation targets**.

Affected equation contexts: **TRISO-ANA-270, TRISO-MR-023, TRISO-MR-024**.

Error codes: SER003, SER004.

### RC2 — Markdown rendered-block serialization incompatibility

Executed strict parser failures: **0**.

Rendered regression classes: **5**.

Affected equation count: **not determinable from available rendered screenshots without fabricating data**. The full rendered diagnostic must enumerate this after a renderer-aware Layer-3 harness is added.

Error codes: REG001–REG005.

### RC3 — first-pass LaTeX reference/citation state interrupted by fatal compile

Equation-reference warnings: **6**, all labels confirmed present.

Citation warnings: **1**, bibliography entry confirmed present.

Affected scientific equations missing: **0**.

Error codes: REF001, REF002.

### RC4 — validator coverage defect

The run printed `stable_tags=0` although direct source audit confirms **1481 `\tag{TRISO-...}` blocks / 1481 unique stable equation IDs**. The existing extractor therefore cannot be treated as the authoritative equation-ID counter until corrected.

## Systematic repair plan — NOT EXECUTED IN THIS PASS

1. Make manuscript generation byte-safe: raw strings / escaped backslashes or a structured serializer; never allow interpreted programming-language escapes in mathematical source.
2. Add a forbidden-control-character scan before Markdown parsing and before LuaLaTeX; fail on C0 controls except LF, with TAB allowed only where explicitly whitelisted outside math.
3. Correct stable-ID extraction independently of `\tag` presentation strategy.
4. Extend Layer 1 to model Markdown block boundaries around every display/environment.
5. Keep strict KaTeX as Layer 2.
6. Add Layer-3 rendered regression checks for literal delimiters, raw TeX, cases failure, vertical fragmentation, and unit fragmentation.
7. Re-run the complete inventory after generator repair; only then group and repair Markdown rendering classes.
8. Re-run LaTeX through all bibliography/reference passes and require REF001=0 and REF002=0 at the final pass.
9. Preserve all 1481 stable equation IDs and change no scientific mathematics.

R2-WOS-02: **OPEN**.

R2-B01: **OPEN**.


## Validated-candidate promotion — run 36888699218

Canonical validation run **36888699218** at source head `0d63a6215e45a0bde5d737f2b90beeeba96beb41` completed successfully under the permanent two-stage architecture.

Executed evidence:

- Stage A `fast-manuscript-validation`: **PASS**.
- Validator self-test and serializer fixture: **PASS**.
- Stable equation IDs: **1481 Markdown / 1481 LaTeX**, unique and synchronized.
- Structured diff: **0 equations added, 0 removed, 0 equation IDs changed**.
- Strict KaTeX 0.16.22: **1498 PASS / 0 FAIL**.
- Authorized serialization restorations only: **TRISO-ANA-270, TRISO-MR-023, TRISO-MR-024**.
- Scientific mathematics changed: **NO**.
- Stage B `full-manuscript-render`: **PASS**.
- LuaLaTeX/BibTeX repeated build and final reference/citation audit: **PASS**.
- PDF artifact: **143 pages**.

The validated candidate from this run was promoted to the canonical Markdown and LaTeX deliverables without an additional normalization or regex-cleanup pass. The historical failure evidence above is retained.

The programming-escape/control-character defect class demonstrated by the prior `0x09` / `0x08` corruptions is closed for the promoted candidate: the canonical promoted targets contain no forbidden control characters.

The permanent build architecture remains Stage A fast validation followed by dependency-gated Stage B full render.

R2-WOS-02 remains **OPEN**.

R2-B01 remains **OPEN**.


## Final presentation closure pass — 2026-10-02

Presentation-only remediation after validated-candidate promotion:

- normalized **7** fragmented nomenclature unit cells in both canonical targets;
- no displayed scientific equation changed;
- stable IDs remain **1481 / 1481 unique** in Markdown;
- forbidden control characters: **0**;
- literal `\\(`, `\\)`, `\\[`, `\\]`: **0** in canonical Markdown;
- `\\textbackslash` / `\\textasciicircum`: **0**;
- listed HTML conversion remnants: **0**;
- fragmented-unit regression scan: **0 remaining**.

Closure status at source level:

- RC1 programming-language escape corruption: **CLOSED**.
- RC2 Markdown block serialization incompatibility: **SOURCE-LEVEL CLOSED; rendered sample confirmation required**.
- RC3 interrupted first-pass references/citations: **CLOSED by successful run 36888699218**.
- RC4 validator coverage defect: **CLOSED for current 1481-ID baseline**.
- REG001 literal dollar visible: **source regression scan CLOSED; visual confirmation pending**.
- REG002 raw TeX visible: **source regression scan CLOSED; visual confirmation pending**.
- REG003 cases render failure: **structural/KaTeX gate CLOSED; visual confirmation pending**.
- REG004 vertical fragmentation: **structural/KaTeX gate CLOSED; visual confirmation pending**.
- REG005 unit fragmentation: **CLOSED at source level (7 repaired, 0 remaining); visual confirmation pending**.

A representative visual sample is recorded separately in `PRESENTATION_VISUAL_SAMPLE.md`.
