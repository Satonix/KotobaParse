# KotobaParse v0.2.0-alpha.4 - Rust DSL diagnostics and summary

This alpha prepares the Rust runtime for the future SekaiTranslator parser editor.

## Added

- `diagnose_source(source)` in `kotoba-core`.
- `summarize_source(source)` and `summarize_spec(spec)` in `kotoba-core`.
- Structured Rust-side diagnostic output with `severity`, `message`, `line`, `column`, and `rule`.
- Editor-friendly parser summary containing parser metadata, types, rules, captures, patch fields, and symbols.
- CLI commands:
  - `kotoba diagnose parser.kotoba`
  - `kotoba check parser.kotoba --json`
  - `kotoba summary parser.kotoba`

## Why this matters

The SekaiTranslator KotobaParse editor should not call extraction just to know whether a parser is valid. It needs a safe language-service layer that keeps returning JSON even when the user is editing an incomplete or broken parser.

## Recommended local validation

```powershell
cargo test --workspace
cargo run -p kotoba-cli -- diagnose examples\ef_command_message.kotoba
cargo run -p kotoba-cli -- summary examples\aokana_bs5.kotoba
cargo run -p kotoba-cli -- check examples\json_array_strings.kotoba --json
```
