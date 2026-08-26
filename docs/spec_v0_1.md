# KotobaParse v0.1 Specification Draft

KotobaParse v0.1 is a small language for describing visual novel script parsers.

A `.kotoba` parser converts raw script lines into structured translatable entries and records safe reinjection spans.

## Core declarations

```kotoba
parser Name
target ".ext"
encoding utf8
```

`parser` names the parser. `target` declares the intended script extension. `encoding` declares how script files should be read/written.

Supported encoding aliases in the runtime:

- `utf8`
- `utf-8`
- `cp932`
- `shift_jis`
- `sjis`

## Indexed lines

Some scripts prefix every line with a source index:

```txt
<22>A quiet afternoon.
```

Declare this with:

```kotoba
line indexed "<" <line:number> ">"
```

Rules then match only the payload after `<number>`.

## Types

Custom types are regex-backed:

```kotoba
type voice_id
    pattern "^[a-zA-Z]+_[0-9]{6}(_[a-z])?$"
```

Built-in capture types:

- `line`
- `rest`
- `quoted`
- `word`
- `name`
- `number`

## Skip rules

```kotoba
skip empty
skip prefix ";"
skip unless ".message"
```

Global skip rules run before ordered parser rules.

## Rules

Rules are evaluated top-to-bottom. The first matching rule wins.

```kotoba
rule dialogue
    .message <id:number> <speaker:name> <text:quoted>
    as Dialogue(text, speaker, ctx:id)
    patch text
```

A rule can:

- match one or more pattern lines
- create an entry using `as`
- mark a reinjection field using `patch`
- store context with `remember`
- skip technical lines using `skip`

Supported entry types in v0.1:

- `Dialogue`
- `Narration`
- `Choice`

## Captures

Capture syntax:

```kotoba
<field:type>
```

Examples:

```kotoba
<text:line>
<text:quoted>
<speaker:name>
<voice:voice_id>
```

Quoted captures extract the inner text and preserve enough patch information to safely re-escape quotes during reinjection.

## Collect

`collect quoted` captures fragmented quoted dialogue across multiple lines.

```kotoba
rule dialogue
    <voice:voice_id>
    <speaker:name>
    collect quoted -> text
        allow asset_id
        allow voice_id
    end
    as Dialogue(text, speaker, ctx:voice)
    patch fragments text mode keep_fragments
```

Allowed lines are preserved but excluded from the extracted text.

## Patch

Simple patch:

```kotoba
patch text
```

Fragment patch:

```kotoba
patch fragments text mode keep_fragments
patch fragments text mode first_fragment
```

`keep_fragments` distributes translated text across original fragments using a length-based heuristic.

`first_fragment` writes the full translation into the first fragment and clears the rest.

## Protect

Protection rules identify tags or engine codes that should be protected before machine/AI translation.

```kotoba
protect
    [r]
    [p]
    pattern "\\[[^\\]]+\\]"
```

Protection is exposed through the Python API and is not automatically applied during extraction.
