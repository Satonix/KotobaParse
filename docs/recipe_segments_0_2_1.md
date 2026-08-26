# KotobaParse 0.2.1 Recipe Segments

Comandos gerais para arquivos com uma linha dividida em segmentos/campos paralelos.

```kotoba
read:
    records as segmented lines
    segments separated by "␂"
    choice segments separated by "␅"
    fields "en", "ja", "zh_cn", "zh_tw"
    source field "en"
    patch field "en"
```

Uso com diálogo:

```kotoba
dialogue:
    segment "en"
    speaker between "【" and "】"
    text after "】："
    save as Dialogue
    patch segment
```

Uso com narração:

```kotoba
text:
    segment "en"
    save as Narration
    patch segment
```

Uso com escolhas em célula `0:"..." 1:"..." OUTLINE:"..."`:

```kotoba
choice:
    segment "en"
    command "select"
    split quoted options
    save as Choice
    patch segment
```

Esses comandos não são específicos de `.bs5`; servem para qualquer formato por linha com campos paralelos separados por delimitadores fixos.
