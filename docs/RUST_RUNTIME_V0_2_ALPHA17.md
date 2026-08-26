# KotobaParse v0.2.0-alpha.17

Alpha.17 extends the Recipe DSL with friendlier pattern matching for users who need regex power without writing full regex.

## Friendly `like` patterns

`like` compiles to regex internally, but keeps parser files readable:

```kotoba
ignore:
    content like any "bgm#", "se#", "map#", "sp#*", "ev#*", "#-#_*"
```

Shorthand meanings:

- `#` = one or more digits, for example `003`, `503`, `23`.
- `0` = one exact digit; repeated zeroes mean exact length, for example `000000` = six digits.
- `A` = one exact ASCII letter; repeated `A` means exact length, for example `AAA` = three letters.
- `*` = zero or more id characters: letters, numbers, underscore.
- `?` = one id character: letter, number, underscore.
- `{name}` = named id segment, useful in voice ids.

Examples:

```kotoba
content like "bgm#"       # ^bgm[0-9]+$
content like "sp#*"       # ^sp[0-9]+[A-Za-z0-9_]*$
content like "#-#_*"      # scene ids like 1-1_0712_Dream
content like "{speaker}_000000"    # yuki_000001
content like "{speaker}_000000_*"  # yuki_000002_a
```

Full regex remains available:

```kotoba
ignore:
    content matching "^(?:bgm|se|map|sp|ev)[0-9]+[A-Za-z0-9_]*$"
```

## Numbered line helper

Recipe now accepts indexed/numbered line declarations:

```kotoba
numbered lines:
    id between "<" and ">"
    content after ">"
```

Matching, skip rules, and patch spans operate on the content after the index, while rebuild preserves the original `<id>` prefix.

## Short ignore aliases

Recipe `ignore:` now accepts:

```kotoba
ignore:
    empty content
    content equals "black"
    content matching "^debug_"
    content like "bgm#"
    content like any "bgm#", "se#"
    ";"      # shorthand for lines starting with ";"
```
