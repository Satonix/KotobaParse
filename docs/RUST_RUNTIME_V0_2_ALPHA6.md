# KotobaParse v0.2.0-alpha.6 - Rust rebuild preview

This release adds an editor-oriented rebuild preview layer to the Rust runtime.

## New Rust APIs

- `preview_rebuild(source, spec, patches)`
- `preview_rebuild_bytes(bytes, spec, patches)`

Both APIs return a `KotobaRebuildPreview` object instead of a rebuilt script.

```json
{
  "changed": true,
  "report": {
    "total_patches": 1,
    "applied_patches": 1,
    "skipped_patches": 0,
    "changed_lines": [1],
    "warnings": []
  },
  "changes": [
    {
      "line": 1,
      "before": ".message 10 Haruka \"Hello.\"\n",
      "after": ".message 10 Haruka \"Olá.\"\n",
      "entries": [
        {
          "id": "dialogue_l00001_1",
          "index": 0,
          "kind": "dialogue",
          "rule": "dialogue",
          "speaker": "Haruka",
          "text": "Hello."
        }
      ]
    }
  ]
}
```

## New CLI command

```powershell
cargo run -p kotoba-cli -- preview parser.kotoba input.sc patches.json preview.json
```

The command writes the preview JSON to `preview.json` and prints the rebuild report to stderr.

## Why this matters for the editor

The SekaiTranslator KotobaParse editor can now show a safe preview before applying a rebuild:

- which patches would be applied;
- which patches would be skipped;
- which physical lines would change;
- before/after text for each changed line;
- entries affected by each changed line.
