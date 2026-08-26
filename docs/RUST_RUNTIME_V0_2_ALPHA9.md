# Rust Runtime v0.2.0-alpha.9

Alpha.9 formalizes KotobaParse as a parser programming language.

This release intentionally does not prioritize `.nut` rebuild. It defines the capabilities the language must expose to support arbitrary visual novel script formats over time.

## New API

```rust
language_spec() -> KotobaLanguageSpec
```

The returned JSON is meant for:

- the future KotobaParse editor inside SekaiTranslator;
- CLI documentation;
- feature gating;
- capability display;
- test planning.

## New CLI command

```powershell
cargo run -p kotoba-cli -- language-spec
```

It prints the formal language feature matrix as JSON.

## Scope

Alpha.9 formalizes these areas:

- program syntax;
- metadata and encoding;
- state/control-flow direction;
- line, block, JSON, table and binary source models;
- extraction model;
- rebuild strategies;
- tag integrity;
- diagnostics and editor-facing APIs.

## Important distinction

The language spec contains three statuses:

- `implemented`: available in the current Rust runtime;
- `experimental`: usable, but not final API;
- `planned` or `reserved`: formal direction, not production behavior yet.

A feature being present in `language-spec` does not mean it is safe to use in production unless its status is `implemented`.
