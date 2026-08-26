# Rust Runtime v0.2.0-alpha.10

This release fixes a Rust build regression introduced in alpha.9.

## Fixed

- `find_bytes` incorrectly used `.windows(...).position(|pos| ...)` as if `pos` were an integer index. In Rust, that closure receives the current window slice (`&[u8]`), so the expression attempted `usize + &[u8]` and failed with `E0277`.
- The helper now compares each window directly and maps the returned index back to the requested start offset.

## Regression coverage

A unit test now verifies that offset-aware binary search returns the expected positions and respects the starting offset.

## Validation

Run on Windows:

```powershell
cargo test --workspace
cargo run -p kotoba-cli -- language-spec
```
