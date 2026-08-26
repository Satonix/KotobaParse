# KotobaParse v0.2.0-alpha.16

Alpha.16 adds the first **Recipe DSL** layer: a simpler, intuitive syntax that compiles to the existing rule runtime.

Main additions:

- `parser Name:` recipe syntax.
- `file`, `encoding`, `quotes`, `protect`, `ignore` recipe blocks.
- `voice format "AAA-{speaker}-0000"` with `voice.speaker` fallback support.
- `choice:` and `message:` high-level recipe blocks that generate normal Kotoba rules internally.
- `remember:` and `clear:` helpers for simple speaker state from tags.
- `binary strings:` recipe metadata mapped to existing binary block specs.
- New protect aliases: `hash numbers`, `html tags`, `ruby tags`, `angle tags`, `newlines`.

Compatibility:

- Existing KotobaParse parsers remain valid.
- Program-style `{ ... }` parsers remain valid.
- Recipe syntax is detected only when the first non-empty line is `parser Name:`.

Known limitation:

- `text block:` is accepted but currently compiles conservatively to a single-line text rule while the multi-line recipe collector is finalized.
