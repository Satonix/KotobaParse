# Rust runtime v0.2.0-alpha.2

This patch release fixes the first regressions reported after running `cargo test --workspace` on Windows.

## Fixed regressions

- `rust_runtime_extracts_json_array_strings` returned `0` entries because the JSON fixture lines include outer JSON quotes and trailing commas. The parser fixture now matches lines such as:

```json
"Keisuke: “...”",
```

- `rust_runtime_extracts_real_aokana_excerpt` expected 3 entries, but the fixture contains 4 BS5 dialogue rows. The test now asserts the real fixture count and verifies the final dialogue entry.

## Test command

```powershell
cargo test --workspace
```

If only the runtime regression test is needed:

```powershell
cargo test -p kotoba-core --test runtime
```
