# KotobaParse Rust Runtime v0.2.0-alpha.13

Regression fix over alpha.12.

## Fixed

- Multiline `block` extraction now prioritizes line-by-line matching for rules whose `match` pattern contains newlines.
- Literal `\\n` inside program syntax is normalized before block matching, so both escaped and already-decoded newline patterns work.
- Block rebuild uses the same line-by-line matcher, keeping extraction and patching consistent.

## PowerShell note

If Windows blocks `tools/package_windows_runtime.ps1`, run it with:

```powershell
powershell -ExecutionPolicy Bypass -File .\tools\package_windows_runtime.ps1
```

This changes execution policy only for that process.
