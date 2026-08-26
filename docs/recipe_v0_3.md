# Recipe canônica — KotobaParse 0.3

## Política de sintaxe

Só existe uma linguagem pública: a Recipe iniciada por `parser Nome:`. Não há fallback, aliases ou modo legado. Um comando desconhecido interrompe `check`, `diagnose`, `extract`, `preview`, `rebuild` e `roundtrip` com a linha responsável.

São rejeitados, entre outros:

- `name:`, `extensions:` e o cabeçalho `parser Nome` sem `:`;
- `kotoba Nome { ... }`;
- `message:`, `text block:`, `indexed lines:` e `binary strings:`;
- `target`, `take`, `text is`, `remember as`, `skip line` e variações de plural;
- blocos sem efeito como `inside strings` e `ignore strings`;
- qualquer comando não reconhecido dentro de um bloco.

## Estrutura mínima

```kotoba
parser Nome:
    file ".ext"
    encoding utf8

    text:
        capture text as rest
        save as Narration
        patch text
```

## Metadados e tipos

```kotoba
file ".sc"
encoding cp932
type VoiceId = matches "^[A-Za-z0-9_-]+$"
type SceneId = like "scene_0000"
```

`encoding` define o encoding do arquivo reconstruído, não o encoding usado
para interpretar o script de entrada. A entrada é lida conforme seus próprios
bytes (BOM, UTF válido ou encoding legado detectado). Se `encoding` for
omitido, a reinjeção preserva o encoding detectado no original.

Para ler um script japonês original e exportar a tradução em ANSI ocidental:

```kotoba
encoding windows-1252
```

`ansi` é aceito como alias, mas `windows-1252` é o nome recomendado por não
depender da página de código configurada no Windows.

`matches` recebe regex. `like` usa o padrão amigável da Recipe.

## Filtros

```kotoba
ignore:
    empty
    starts with any ";", "#"
    contains "[jump"
    ends with ".png"
    equals any "black", "white"
    matches "^bgm[0-9]+$"
    like any "se#", "ev#*"
```

## Proteção

```kotoba
protect:
    between "[" and "]"
    literal "@n"
    matches "\\\\[A-Za-z]+"
```

## Diálogo com speaker delimitado

```kotoba
dialogue:
    when starts with "<"
    capture speaker between "<" and ">"
    capture text after ">"
    save as Dialogue
    patch speaker
    patch text
```

Esse bloco extrai `<Kyoji>Texto` diretamente como diálogo. `patch speaker`
habilita a reinjeção opcional do nome, enquanto `patch text` altera apenas o
texto posterior a `>`. Não é necessário usar `rule Dialogue:` para esse caso.

## Blocos semânticos nomeados

```kotoba
dialogue PrintText:
    when starts with "[PrintText]="
    capture speaker as field 2 separated by tab
    capture text as field 3 separated by tab
    patch text

text SceneTitle:
    when matches "^str 155[ \t]+(?P<text>.+)$"
    patch text

ignore StringCommand:
    when starts with "str "
```

O nome depois de `dialogue`, `text`, `choice` ou `ignore` é opcional. Ele
serve para documentar e distinguir várias formas do mesmo tipo semântico.
`dialogue` usa `Dialogue` como tipo padrão, `text` usa `Narration`, `choice`
usa `Choice` e `ignore` descarta o record; `save as` e `skip` só são
necessários para sobrescrever esses padrões.

Condições aceitas incluem `when matches`, `when format`, `when starts with`,
`when exists`, `when not exists`, `when previous is voice`, `when previous
is speaker`, `when next is quoted` e `when text is quoted`. Capturas por
regex, formato, delimitadores e colunas podem ser usadas diretamente nesses
blocos.

`rule Nome:` permanece apenas para compatibilidade de leitura com Recipes da
`0.3.0-alpha.2`. Não é necessário em parsers novos.

## Scripts binários

```kotoba
read:
    records as binary
    magic "10 00 00 08"
    length u32le
    encoding utf8
    min length 4
```

`read: records as binary` é a única declaração de records binários. O antigo bloco `binary strings:` não existe.

## JSON estruturado

```kotoba
json Line:
    entries "$.scenes[*].lines[*]"
    id "id"
    speaker "speaker"
    context "voice"
    text "text"
```

`entries` e `text` são obrigatórios.

## Migração

A versão 0.3 não converte parsers automaticamente. Cada parser antigo deve ser reescrito e validado com:

```powershell
kotoba check parser.kotoba
kotoba roundtrip parser.kotoba script-original.ext
```

Depois da migração, faça também extração e reinjeção com casos reais da engine; passar no `check` prova a sintaxe, não a cobertura do formato.
