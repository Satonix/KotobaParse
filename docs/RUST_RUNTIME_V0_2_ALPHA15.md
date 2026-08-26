# KotobaParse v0.2.0-alpha.15

This release reduces the amount of regex needed for common visual novel parser tasks while keeping all previous `type ... pattern` and `protect pattern` syntax compatible.

## New simplified skip syntax

Use these for ordinary line filtering instead of writing custom regex types plus skip rules.

```kotoba
skip empty
skip startswith ";"
skip startswith "*"
skip startswith "["
skip startswith any ";", "*", "["
skip contains "[jump"
skip endswith ".png"
skip equals "[cm]"
skip unless startswith ".message"
skip unless contains ".message"
skip unless endswith "[cr]"
skip unless equals "TEXT"
```

Legacy syntax still works:

```kotoba
skip prefix ";"
skip unless ".message"
```

`skip unless "x"` keeps its old behavior: skip the line unless it contains `x`. Use `skip unless startswith "x"` when the beginning of the line matters.

## New simplified protect syntax

```kotoba
protect bracket "[" "]"
protect tag "[" "]"
protect slash_commands
protect quote "「" "」"
```

These are shortcuts for common regex protection patterns. The old syntax remains valid:

```kotoba
protect
    pattern "\\[[^\\]]+\\]"
    pattern "\\\\[A-Za-z]+"
```

## Example: simple KiriKiri/KAG parser

```kotoba
parser YandereKiriKiriKS

target ".ks"
encoding utf-8

skip empty
skip startswith ";"
skip startswith "*"
skip startswith "["

protect bracket "[" "]"

rule narration
    <text:line>
    as Narration(text, ctx:text)
    patch text
```

This ignores comments, labels and full-line commands, while extracting normal text lines and preserving inline tags such as `[cr]`.

## Example: `.message` scripts

```kotoba
parser EfMusicaSC

target ".sc"
encoding cp932

skip empty
skip unless startswith ".message"
protect slash_commands

rule narration
    .message <id:number> <text:rest>
    as Narration(text, ctx:id)
    patch text
```

For complete `.message` support with speakers and voice IDs, continue using typed captures and ordered rules.
