# KotobaParse Recipe DSL — alpha.16 draft

Recipe DSL is an easier, Python-shaped but non-Python syntax for visual novel parsers.
It is intentionally written like short instructions instead of code.

```kotoba
parser SimpleKiriKiri:
    file ".ks"
    encoding cp932

    protect:
        brackets "[" and "]"

    ignore:
        empty lines
        lines starting with ";"
        lines starting with "*"
        lines starting with "["

    text:
        text is rest
        save as Narration
        patch text
```

Supported in alpha.16:

- `parser Name:` recipe header.
- `file`, `target`, `encoding`, `mode binary`.
- `quotes:` using `"open" to "close"`.
- `protect:` with `slash commands`, `hash numbers`, `between`, `brackets`, `html tags`, `ruby tags`, `angle tags`, `newlines`.
- `ignore:` with `empty lines`, `lines starting with`, `comments`, `labels`, `command lines`, `tags`.
- `voice:` with `format "AAA-{speaker}-0000"`.
- `choice:` for simple command choices.
- `message:` for command-style VN dialogue lines.
- `remember:` / `clear:` for simple tag-based speaker state.
- `binary strings:` metadata compiled to the existing binary string block model.

Compatibility: old KotobaParse rules still work. Recipe DSL compiles into the existing runtime spec.

Known alpha.16 limitation: `text block:` is accepted but still compiles conservatively to a line rule while the multi-line recipe collector is finalized.

Additional alpha.17 prototype features:

- `numbered lines:` / `indexed lines:` with `id between "<" and ">"`.
- `ignore:` with `empty content`, `content equals`, `content matching`, `content like`, `content like any`.
- Short ignore literals like `";"`, equivalent to `lines starting with ";"`.
- Friendly `like` patterns:
  - `#` = one or more digits.
  - `0` = one exact digit, repeated for exact length.
  - `A` = one exact ASCII letter, repeated for exact length.
  - `*` = zero or more id characters `[A-Za-z0-9_]`.
  - `?` = one id character `[A-Za-z0-9_]`.
  - `{name}` = named id segment.

Example:

```kotoba
ignore:
    content like any "bgm#", "se#", "map#", "sp#*", "ev#*", "#-#_*"
```
