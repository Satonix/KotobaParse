# Reinjection

KotobaParse reinjection is based on patch spans captured during extraction.

For a simple quoted line:

```txt
.message 10 Haruka "こんにちは"
```

this rule:

```kotoba
rule dialogue
    .message <id:number> <speaker:name> <text:quoted>
    as Dialogue(text, speaker, ctx:id)
    patch text
```

patches only the inner quoted text. The command, ID, speaker and quote characters are preserved.

## Escaping

When a `quoted` capture is patched, translations are escaped before being written back:

- backslash becomes `\\`
- the active quote character becomes escaped
- newlines become `\n`
- tabs become `\t`

Example translation:

```txt
Ele disse "oi"
```

becomes:

```txt
"Ele disse \"oi\""
```

inside the script.

## Fragment modes

For fragmented dialogue, use:

```kotoba
patch fragments text mode keep_fragments
```

or:

```kotoba
patch fragments text mode first_fragment
```

`keep_fragments` tries to preserve timing by splitting the translation across the original text fragments.

`first_fragment` is safer for readability but may alter timing or visual rhythm.
