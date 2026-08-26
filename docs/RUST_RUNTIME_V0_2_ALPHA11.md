# Rust Runtime v0.2.0-alpha.11

Alpha 11 turns the Rust runtime from a rule-file executor into the minimum viable language runtime needed by the future SekaiTranslator KotobaParse editor.

Implemented/activated in this round:

- `state` declarations with optional initial values.
- Stateful extraction through `remember`, `forget`, `set` and `let` inside rules.
- Rule conditions through `when`, including `exists`, `missing`, `==`, `!=` and `contains`.
- Program blocks through `block { start; end; rule; }` for multiline script fragments.
- Initial structured JSON extraction/rebuild through `json { entries; text; speaker; context; id; }`.
- Text transforms through `transform FIELD { replace FROM TO on extract/rebuild; }`.
- Protected-token checking during rebuild using `protect literal` and `protect pattern`.
- Rebuild strategy declaration through `rebuild strategy ...` for editor/diagnostic visibility.
- CLI `editor-probe`, which emits diagnostics + summary + extraction report + sample entries for the SekaiTranslator editor.

Important limitations:

- Block rebuild is implemented for simple block-local field replacement, but complex block reshaping still needs engine-specific tests.
- JSON rebuild currently serializes with pretty formatting; it preserves data, not original whitespace.
- Binary reinjection remains deferred.
- Python remains the compatibility reference until Rust/Python parity tests are expanded.

Suggested validation on Windows:

```powershell
cd "S:\Sekai Visual Novel\Ferramentas\Github Sekai\KotobaParse"
cargo test --workspace
cargo run -p kotoba-cli -- language-spec
cargo run -p kotoba-cli -- check examples\language_runtime_ready.kotoba
cargo run -p kotoba-cli -- summary examples\language_runtime_ready.kotoba
```

Editor probe example:

```powershell
cargo run -p kotoba-cli -- editor-probe examples\ef_command_message.kotoba tests\fixtures\real\ef_100_01_excerpt.sc --out editor_probe.json
```
