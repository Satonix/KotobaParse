# Public API v0.1

The SekaiTranslator should use the public API instead of shelling out to the CLI.

```python
from kotobaparse import load_parser, extract_file, template_file, inject_file

parser = load_parser("examples/aokana_bs5.kotoba")
entries = extract_file(parser, "sample.bs5", source_field="jp")
template = template_file(parser, "sample.bs5", source_field="jp")

inject_file(
    parser,
    "sample.bs5",
    "sample_template_jp.json",
    "sample_patched.bs5",
    target_field="en",
    characters="sample_characters_jp.json",
)
```

## Main functions

- `load_parser(path)`
- `load_parser_string(source)`
- `check_parser(parser)`
- `extract_file(parser, script_path, source_field=None)`
- `extract_string(parser, script_source, source_field=None)`
- `template_file(parser, script_path, source_field=None)`
- `characters_file(parser, script_path, source_field=None)`
- `inject_file(parser, script_path, translations, out_path, target_field=None, characters=None, speaker_target_field=None)`
- `inspect_file(parser, script_path)`
- `trace_file(parser, script_path)`

`translations` and `characters` can be either dictionaries or JSON file paths.
