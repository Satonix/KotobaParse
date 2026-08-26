# KotobaParse language service

`kotobaparse.language_service` is the editor-facing layer for `.kotoba` files.
It is intentionally UI-agnostic: it only returns structured diagnostics and parser summaries.
SekaiTranslator can call this layer from the future KotobaParse editor without depending on parser internals.

## Public API

```python
from kotobaparse import diagnose_source, diagnose_file, summarize_source, summarize_file
```

### `diagnose_source(source)` / `diagnose_file(path)`

Parses and validates a parser without throwing syntax errors to the editor.
The return value is a `ParserDiagnosticReport`:

```json
{
  "ok": true,
  "diagnostics": [],
  "summary": {
    "name": "EFMessage",
    "target": ".sc",
    "encoding": "cp932",
    "rules": []
  }
}
```

When a parser is incomplete or invalid, `ok` becomes `false` and diagnostics include line/column data when available.

### `summarize_source(source)` / `summarize_file(path)`

Returns a `ParserSummary` with stable metadata for editor sidebars, outlines and future syntax helpers:

- parser name, target and encoding;
- presets;
- custom types;
- rules;
- captures;
- actions;
- patch fields;
- quote pairs;
- protect rules;
- outline symbols.

## CLI

```powershell
kotoba diagnose parser.kotoba
kotoba check parser.kotoba --json
kotoba spec parser.kotoba
```

`diagnose` and `check --json` are meant for live editor validation.
`spec` is meant for parser outline/preview panels.
