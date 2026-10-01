# GitHub Mathematics Rendering Standard

Checked: 2026-10-02.

## Authoritative platform evidence

GitHub's official "Writing mathematical expressions" documentation states that GitHub supports LaTeX-formatted mathematics in Markdown and renders it with **MathJax**. Inline mathematics may use `$...$` (or GitHub's backtick-assisted inline form). Display mathematics may use `$$...$$`, or the dedicated fenced ```math` block. GitHub explicitly states that the fenced-math form does not require `$$` delimiters.

GitHub's markup repository documents that Markdown files are processed through GitHub's markup pipeline and then additional GitHub.com filters/sanitization. Therefore generic LaTeX or KaTeX validity is not sufficient evidence of GitHub rendering behavior.

Primary references:
- GitHub Docs, "Writing mathematical expressions": https://docs.github.com/en/get-started/writing-on-github/working-with-advanced-formatting/writing-mathematical-expressions
- GitHub Blog, "Math support in Markdown" (2022; updated 2024): https://github.blog/news-insights/product-news/math-support-in-markdown/
- github/markup: https://github.com/github/markup
- MathJax documentation / source: https://github.com/mathjax/MathJax-docs

## Selected manuscript standard

Inline mathematics: `$...$`.

Displayed mathematics:

```math
<atomic mathematical body>
```

Stable equation identifiers are ordinary Markdown immediately before the math fence:

**Equation TRISO-GOV-020**

The Markdown mathematical body does **not** contain `\tag{TRISO-...}`.

## Rationale

The former manuscript architecture used multiline `$$` blocks containing `\tag{TRISO-...}`. Although `$$` is officially supported, the user observed actual GitHub rendering failures in this manuscript: visible delimiters/tags, split multiline expressions, cases failures, and vertical fragmentation. The dedicated `math` fence is also officially supported and gives Markdown an explicit atomic block boundary, while moving traceability metadata out of the MathJax expression.

The LaTeX/PDF target remains native LaTeX with its existing `\tag` / `\label` strategy.

KaTeX remains a secondary syntax checker only. It is not evidence of GitHub-render compatibility.

## Compatibility assumptions

- Math bodies are preserved verbatim except removal of the Markdown-only `\tag{TRISO-ID}` presentation line.
- Every old stable ID becomes exactly one adjacent ordinary-Markdown equation marker.
- Fenced math is not placed inside ordinary Markdown table cells.
- Mathematical strings remain opaque to programming-language escape interpretation.
- Actual GitHub-rendered stratified inspection remains mandatory after promotion.
