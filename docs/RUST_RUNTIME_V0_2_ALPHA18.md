# KotobaParse Rust Runtime v0.2.0-alpha.18

This alpha extends the Recipe DSL for numbered visual novel scripts where voice ids,
speaker names and dialogue text live on separate `<id>content` lines.

## New Recipe blocks

```kotoba
voice:
    content like any "{speaker}_000000", "{speaker}_000000_*"
    remember as voice
    skip line

speaker:
    content when previous is voice and next is quoted
    remember as speaker
    skip line

dialogue:
    text is quoted
    speaker fallback remembered speaker
    speaker fallback voice.speaker
    save as Dialogue
    patch content
```

`voice:` stores the current voice id and clears a stale remembered speaker.
`speaker:` stores a plain speaker line only when the previous context has a voice and
the next non-skipped line is quoted dialogue.
`dialogue:` emits a Dialogue entry using the remembered speaker first, then falling
back to the derived speaker name from `voice.speaker`.

## Target format

```txt
<108>yuki_000003
<109>Yuki
<110>"Mamiya Takuji."
```

This extracts as:

```txt
kind: dialogue
speaker: Yuki
text: Mamiya Takuji.
```
