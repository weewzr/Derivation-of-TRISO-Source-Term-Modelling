# Source Rendering Audit

Baseline recovered: `main` at `d518996918df726cfba05e984ffd2e609af7cfa3`.

Scope: presentation/source remediation only. No verified scientific equation, physical model, inverse-Laplace calculation, release CDF, Method 3 model, or supervisor repository content was changed.

## Whole-document artifact audit

| Artifact class | Markdown found/repaired | LaTeX found/repaired | Final known count |
|---|---:|---:|---:|
| GitHub-incompatible inline `\\(...\\)` / residual delimiter tokens | 604 simple pairs plus 83 residual opening and 79 residual closing tokens normalized | 0 | 0 |
| Nested asymptotic inline-math delimiters | 4 | 4 | 0 |
| Missing backslash on Greek commands (`lambda`, `epsilon`) | 5 | 3 | 0 |
| Missing backslash on vector/matrix command `mathbf` | 4 command starts plus associated prose occurrences | 0 after normalization | 0 |
| Plain prose `(dt)`, `(dV)`, `(0/0)` used as mathematical notation | 4 | 0 | 0 |
| `\\textbackslash` | 0 | 0 | 0 |
| `\\textasciicircum` | 0 | 0 | 0 |
| HTML remnants (`&amp;`, `&lt;`, `&gt;`, `&#x20;`) | 0 | 0 | 0 |

The first simple-pair count and later residual-token counts are sequential audit counts, not additive estimates of unique original defects.

## Delimiter and equation-ID audit

- Markdown display `$$` state closes cleanly.
- Markdown inline-dollar audit: 0 lines with an odd number of unescaped inline dollar delimiters outside display blocks.
- Markdown stable displayed equation tags: **1481 total / 1481 unique**.
- LaTeX stable displayed equation tags: **1481 total / 1481 unique**.
- Duplicate stable equation IDs: **0** in each representation.
- Equation loss during this remediation: **0**.
- Scientific equation changes: **0**. All changes in this pass are classified **FORMAT ONLY**.

## Representative source audit A-R

| Block | Markdown human-readable | LaTeX source human-readable | Mathematics synchronized |
|---|---|---|---|
| A. conservation derivation | YES | YES | YES |
| B. spherical-coordinate derivation | YES | YES | YES |
| C. interface conditions | YES | YES | YES |
| D. analytical eigenproblem | YES | YES | YES |
| E. finite-volume derivation | YES | YES | YES |
| F. centre-cell derivation | YES | YES | YES |
| G. WOS derivation | YES | YES | YES |
| H. centred-ball first passage | YES | YES | YES |
| I. spherical-shell first passage | YES | YES | YES |
| J. interface transmission | YES | YES | YES |
| K. accelerated renewal | YES | YES | YES |
| L. Markov-renewal state definitions | YES | YES | YES |
| M. complete K(s) construction | YES | YES | YES |
| N. matrix inverse derivation | YES | YES | YES |
| O. Neumann-series interpretation | YES | YES | YES |
| P. hard five-layer verification | YES | YES | YES |
| Q. high-precision conditioning | YES | YES | YES |
| R. references / traceability material | YES | YES | YES |

## Tables, units, headings and paths

The nomenclature table was a confirmed visible defect: it used literal `\\(...\\)` notation and escaped superscripts. It now uses GitHub inline math such as `$D_i$`, `$\\mathbf J$`, and mathematical units.

Whole-source scans found no remaining `\\textbackslash`, `\\textasciicircum`, or listed HTML conversion remnants. Code/repository paths remain code/path text rather than being converted to mathematics.

## Status

**Markdown source-level human readability: PASS.**

**LaTeX source-level human readability: PASS.**

**Markdown/LaTeX equation parity: PASS (1481 / 1481 unique stable equation IDs).**

This is a source-level result only. It does **not** classify the final manuscript READY. Executed CI render and visual inspection remain required.

R2-WOS-02: **OPEN**.

R2-B01: **OPEN**.


## Root-cause correction — rendered-output failure

The previous source-level PASS is superseded for Markdown by user-observed GitHub-rendered failures. Balanced delimiters and equation parity did not establish rendered correctness.

### Old generation architecture

Repository history shows that the detailed Markdown became the working manuscript and LaTeX was subsequently regenerated from that Markdown (notably commit `7c6032742d571a72a86b1a774bfb1e87524e5a1c`, followed by later LaTeX regeneration passes). Both targets were then edited repeatedly, including broad prose/inline-math normalization passes. This created an unsafe architecture in which Markdown-specific parsing rules, LaTeX syntax, and regex/string cleanup could interact and corrupt nested mathematical structures.

### Root cause

The root cause is **presentation serialization without a syntax-aware, target-specific mathematical boundary**. Arbitrary LaTeX mathematics was treated as text that could be normalized by delimiter/string substitutions. GitHub Markdown then parsed newlines, blank lines, backslashes, table syntax, and math delimiters differently from LaTeX. Consequently a source file could have balanced delimiter counts and matching equation IDs while the GitHub-rendered mathematical object was split into ordinary Markdown blocks.

The user-observed failures — literal `$$`, literal `mol m$^{-3}$`, vertically stacked expressions, split source-term arrays, and `Missing \\end{cases}` — are now explicit regression tests.

### Corrected architecture

The project will use one canonical **scientific content model** and two target-specific serializers. Until a dedicated structured content file is introduced, the scientifically verified LaTeX/equation register and canonical derivation evidence define the mathematics; Markdown is a presentation target, not an intermediate parser for LaTeX generation.

Displayed equations are atomic objects. Markdown serialization must emit one contiguous GitHub math block per equation with no Markdown paragraph breaks inside an environment. LaTeX serialization remains native LaTeX. No circular Markdown -> regex -> LaTeX -> regex -> Markdown conversion is permitted.

### Compatibility gate

Before full-manuscript reconstruction, the repository now contains:

- `ray to outram park/MARKDOWN_MATH_COMPATIBILITY_TEST.md`
- `verification/validate_manuscript_markdown.py`

The fixture covers scalar, fraction, derivative, partial derivative, vector, integral, matrix, aligned derivation, cases, units, roman subscripts, Greek symbols, equation identifier strategy, long equation, and nested braces/parentheses.

The validator is structural only and explicitly does not claim visual correctness.

### Equation-ID strategy under test

The compatibility fixture uses a stable ordinary-Markdown identifier immediately above each atomic display equation, e.g. **Equation TRISO-GOV-020**, rather than requiring `\\tag{TRISO-GOV-020}` inside GitHub math. The LaTeX target retains native labels/tags. This strategy will be adopted for the full Markdown only after rendered GitHub validation.

### Current status

Markdown rendered validation: **FAIL / BLOCKED by demonstrated rendered-output defects in the current full manuscript**.

Compatibility fixture source: **CREATED; rendered GitHub validation required before full regeneration**.

LaTeX: retained; no Markdown-specific rewrite has been applied in this root-cause pass.

Full 1,481-equation Markdown regeneration: **NOT STARTED**, intentionally, pending fixture validation.


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
