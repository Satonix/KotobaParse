# KotobaParse Rust Runtime v0.2.0-alpha.3

This release fixes the remaining JSON array regression from alpha.2.

## Problem

The JSON fixture contains indented string entries, for example:

```json
      "Keisuke: “..."",
```

The Rust line matcher previously matched only the physical line exactly. Since the DSL pattern intentionally starts at the JSON string quote, indentation caused all JSON entries to be skipped.

## Fix

For each rule, the runtime now tries:

1. the original physical line;
2. `trim()` as a fallback only when the original line does not match.

The entry still keeps the original physical line number, so rebuild/patch behavior remains line-stable.

## Validation

Run locally:

```powershell
cargo test --workspace
```

Or only the regression test:

```powershell
cargo test -p kotoba-core --test runtime rust_runtime_extracts_json_array_strings
```
