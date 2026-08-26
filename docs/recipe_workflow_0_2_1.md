# KotobaParse 0.2.1 Recipe Workflow

A Recipe parser should read like a small checklist:

```kotoba
parser NomeDoParser:
    file ".ext"
    encoding utf8

    read:
        # 1. Como o arquivo organiza cada entrada?
        records as lines

    numbered lines:
        # 2. A linha tem id/cabeçalho antes do texto?
        id between "<" and ">"
        content after ">"

    protect:
        # 3. O que aparece dentro do texto, mas não deve ser traduzido?
        slash commands
        hash numbers
        between "[" and "]"

    ignore:
        # 4. O que NÃO é texto?
        empty content
        content like any "bgm#", "se#", "map#"
        content equals any "black", "white"

    voice:
        # 5. Existe voice-id?
        content like any "{speaker}_000000", "{speaker}000000"
        remember as voice
        skip line

    speaker:
        # 6. Existe speaker separado?
        content when previous is voice and next is quoted
        remember as speaker
        skip line

    dialogue:
        # 7. O que é diálogo?
        text is quoted
        speaker fallback remembered speaker
        speaker fallback voice.speaker
        save as Dialogue
        patch content

    text:
        # 8. O que sobrou é narração?
        text is content
        save as Narration
        patch content
```

For structured block scripts, use the same recipe order, but change the reader:

```kotoba
read:
    records as blocks
    block starts with "text = {"
    block ends when braces close
```
