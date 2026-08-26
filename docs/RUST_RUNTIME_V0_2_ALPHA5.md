# KotobaParse v0.2.0-alpha.5 - Rust rebuild reports and field-aware patching

This alpha hardens the Rust rebuild path before the SekaiTranslator KotobaParse editor is added.

## Added

- `KotobaRebuildReport` with:
  - `total_patches`
  - `applied_patches`
  - `skipped_patches`
  - `changed_lines`
  - `warnings`
- `rebuild_with_report(source, spec, patches)` in `kotoba-core`.
- `rebuild_bytes_with_report(bytes, spec, patches)` in `kotoba-core`.
- CLI `rebuild` now prints a rebuild report as JSON to stderr.
- CLI `roundtrip --out ...` now prints a no-op rebuild report.

## Changed

- Rebuild no longer depends only on a broad `line.find(entry.text)` replacement.
- Patches are applied to the captured field span from the matched rule, which is safer for repeated text on the same file.
- Quoted captures preserve their original quote wrapper when replacing text.
- Speaker patches use the speaker capture field when available.
- Choice-group patches can replace a single selected choice inside a choice cell instead of replacing the entire cell.
- Byte rebuild with no patches, or with zero applied patches, returns the original bytes unchanged.
- UTF-8 BOM and UTF-16 BOM/no-BOM style are preserved when bytes are rebuilt after applied patches.

## Safety behavior

A patch is skipped when:

- both translation fields are empty;
- the target entry cannot be found;
- the source text in the patch does not match the current extracted source text;
- the target line or capture field cannot be found.

This is intentionally stricter than the old fallback behavior because reinjection must not silently patch the wrong text.

## Recommended local validation

```powershell
cargo test --workspace
cargo run -p kotoba-cli -- roundtrip examples\ef_command_message.kotoba tests\fixtures\real\ef_100_01_excerpt.sc
cargo run -p kotoba-cli -- roundtrip examples\majikoi_plaintext.kotoba tests\fixtures\real\majikoi_act_b_excerpt.txt
```
