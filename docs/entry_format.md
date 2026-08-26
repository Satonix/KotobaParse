# KotobaParse Entry Format v0.1

This document describes the JSON contract intended for SekaiTranslator integration.

Every extracted entry has a stable shape:

```json
{
  "id": "dialogue:12:abcdef123456",
  "type": "Dialogue",
  "text": "Hello",
  "speaker": "Yuki",
  "context": "scene_01",
  "rule": "dialogue",
  "line": 12,
  "fields": {},
  "sources": {},
  "patch": {},
  "speaker_patch": {},
  "patches": {},
  "speaker_patches": {},
  "meta": {}
}
```

## Important fields

- `id`: deterministic ID used as the translation key.
- `type`: entry type, usually `Dialogue`, `Narration` or `Choice`.
- `text`: selected source text. This may change when `source_field` is used.
- `speaker`: selected speaker name, if any.
- `context`: optional context such as label, voice ID, message ID or choice key.
- `fields`: all captured fields and remembered memory values.
- `sources`: multilingual source fields such as `en`, `jp`, `zh_cn`, `zh_tw`.
- `patch`: default text patch used during injection.
- `speaker_patch`: default speaker patch used during character-name injection.
- `patches`: all compatible text patch destinations by field name.
- `speaker_patches`: all compatible speaker patch destinations by field name.

## Source field vs target field

`source_field` controls what the translator sees as the original text.

`target_field` controls what field is modified during reinjection.

For a multilingual `.bs5` script, this allows:

```txt
source_field = jp
target_field = en
```

So the user can translate from Japanese while writing Portuguese into the English slot.
