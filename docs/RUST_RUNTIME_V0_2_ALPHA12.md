# Rust Runtime v0.2.0-alpha.12

This is a focused regression-fix release after alpha.11.

## Fixed

- `protect { literal "@n"; }` is now accepted as a valid one-line program block.
- Inline block statements are split on semicolons while preserving quoted strings.
- Multiline `block` extraction now has a per-line fallback matcher for patterns that span several physical lines.
- Multiline block rebuild now joins block lines with `\n` before patching, instead of concatenating them into a single physical line.

## Why it matters

These fixes unblock the editor-facing language runtime tests added in alpha.11. They are required before wiring KotobaParse diagnostics, summary and preview into SekaiTranslator V2.

## Test on Windows

```powershell
cd "S:\Sekai Visual Novel\Ferramentas\Github Sekai\KotobaParse"
cargo test --workspace
cargo run -p kotoba-cli -- language-spec
cargo run -p kotoba-cli -- editor-probe examples\language_runtime_ready.kotoba tests\fixtures\real\ef_100_01_excerpt.sc --out editor_probe.json
```
