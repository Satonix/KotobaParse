# Kotoba Language Spec v0.2 Draft

Esta especificação formaliza o objetivo do KotobaParse: uma linguagem própria para programar parsers de engines de visual novel, não apenas um arquivo de regex.

A linguagem deve cobrir três classes de script:

1. **Textual linear**: `.ks`, `.sc`, `.txt`, `.bs5` textual, arquivos com comandos por linha.
2. **Estruturado**: JSON, tabelas, arrays de strings, blocos com chave/valor.
3. **Binário/chunked**: scripts compilados com strings, offsets, payloads com tamanho e metadados.

O princípio central é:

```txt
parser .kotoba + script original -> entries traduzíveis -> preview/rebuild -> script reconstruído
```

O runtime deve ser previsível, determinístico e seguro para editor. O parser não pode depender da UI do SekaiTranslator.

## Programa

A forma canônica é program-style:

```kotoba
kotoba NomeDoParser {
    target ".sc";
    encoding cp932;

    rule dialogue {
        match ".message <id:number> @<speaker:name> <voice:word> <text:quoted>";
        emit Dialogue(text, speaker, ctx:voice);
        patch text;
        patch speaker speaker;
    }
}
```

O formato antigo continua válido como compatibilidade, mas novos recursos devem nascer na sintaxe program-style.

## Metadados obrigatórios

```kotoba
target ".ks";
extension ".ks";
extensions ".ks", ".tjs";
encoding utf-8;
encoding cp932;
encoding utf-16le;
```

O runtime deve sempre processar bytes usando o encoding declarado. Quando possível, deve preservar BOM, quebras de linha e bytes sem alteração em round-trip sem patch.

## Modelo de execução

O KotobaParse processa o script em estágios:

1. Decodificação de bytes.
2. Normalização mínima controlada pelo parser.
3. Aplicação de skips globais.
4. Execução de regras em ordem.
5. Atualização de estado/contexto.
6. Emissão de entries.
7. Preview/rebuild usando campos capturados.

A linguagem deve evitar efeitos colaterais arbitrários. Ela é uma DSL de parsing/rebuild, não uma linguagem geral como Python.

## Tipos e captures

Tipos declaram padrões reutilizáveis:

```kotoba
type speaker {
    pattern "[A-Za-zÀ-ÿ_][^\\s\"“”]{0,40}";
    trim;
}
```

Captures são nomeadas:

```kotoba
<speaker:name>
<text:quoted>
<id:number>
<text:rest>
<choice:cell>
```

O campo capturado deve ser preservado em `entry.fields`, permitindo rebuild por campo.

## Regras

```kotoba
rule dialogue {
    match "<speaker:name>: <text:rest>";
    emit Dialogue(text, speaker);
    patch text;
}
```

Comandos de regra:

| Comando | Status | Uso |
|---|---:|---|
| `match` | implementado | padrão principal da regra |
| `emit` | implementado | cria Dialogue/Narration/ChoiceGroup |
| `patch` | implementado | define campo reinjetável |
| `patch speaker` | implementado | permite renomear personagem |
| `remember` | implementado inicial | guarda capture em contexto |
| `forget` | implementado inicial | remove contexto |
| `skip` | implementado | regra auxiliar sem entry |
| `when` | planejado | condição local |
| `if` | planejado | controle simples e determinístico |
| `set/state` | planejado | estado nomeado formal |

## Estado/contexto

Recursos atuais:

```kotoba
remember speaker;
forget speaker;
```

Forma planejada:

```kotoba
state speaker = null;
set speaker = capture(name);
when speaker != null;
```

Uso previsto: engines onde o nome do personagem aparece em uma linha e o texto em outra.

## Skips

```kotoba
skip empty;
skip prefix "#";
skip unless ".message";
skip asset;
```

Skips devem ser seguros: não podem apagar dados do script, apenas impedir emissão de entry.

## Proteção de tags

```kotoba
protect {
    pattern "@[^\\s]+";
    pattern "\\[[^\\]]+\\]";
    literal "@n";
}
```

A especificação final deve exigir validação de tags preservadas quando o parser declarar:

```kotoba
require tags preserved;
```

## Transformações

Planejado:

```kotoba
transform text {
    decode "@n" as newline;
    encode newline as "@n";
    protect tags;
}
```

Isso é essencial para engines que usam `@n`, `\n`, `[r]`, `<br>`, comandos inline, rubi, voice tags etc.

## Choices

```kotoba
rule choices {
    match "<choices:cell>";
    emit ChoiceGroup(choices);
    patch text;
}
```

Cada escolha deve ser uma entry independente, mas o rebuild deve alterar apenas a opção correspondente dentro da célula/lista.

## Blocos multilinha

Planejado:

```kotoba
block message {
    start "BEGIN_MESSAGE";
    end "END_MESSAGE";

    rule body {
        match "<speaker:name>\n<text:rest>";
        emit Dialogue(text, speaker);
    }
}
```

Necessário para engines que não usam uma fala por linha.

## JSON estruturado

Planejado:

```kotoba
json scenario {
    path "$.scenario[*]";
    text "text";
    speaker "speaker";
    id "id";
    emit Dialogue(text, speaker, ctx:id);
    patch text;
}
```

O parser atual por regex de JSON array é temporário e serve para casos simples.

## Tabelas

Planejado:

```kotoba
table csv {
    delimiter ",";
    text column "Text";
    speaker column "Name";
    id column "Id";
    emit Dialogue(text, speaker, ctx:id);
}
```

## Binário

Experimental:

```kotoba
binary string_block {
    magic "10 00 00 08";
    length u32le;
    encoding utf-8;
    min_len 4;
    profile gls_nut;
}
```

Planejado para rebuild:

```kotoba
binary patch same_length pad 00;
binary patch resize length u32le;
binary patch external table;
```

Para binários, rebuild só deve ser liberado quando a estratégia for explícita.

## Estratégias de rebuild

| Estratégia | Status | Uso |
|---|---:|---|
| `field` | implementado | substitui campo capturado preservando linha |
| `choice` | implementado inicial | troca opção dentro de ChoiceGroup |
| `same_length` | planejado | binário com tamanho fixo |
| `resize_chunk` | planejado | atualiza tamanho do payload/chunk |
| `external_patch` | planejado | gera patch externo sem alterar original |
| `json_path` | planejado | atualiza nó JSON específico |

## Diagnóstico e editor

O runtime deve expor:

```powershell
kotoba check parser.kotoba
kotoba check parser.kotoba --json
kotoba diagnose parser.kotoba
kotoba summary parser.kotoba
kotoba language-spec
```

Esses comandos alimentam o futuro editor do KotobaParse dentro do SekaiTranslator.

## Critério para dizer que suporta uma engine

Uma engine só deve ser marcada como suportada quando tiver:

1. Parser `.kotoba` oficial ou fixture.
2. Extração com entries corretas.
3. Preview de rebuild correto.
4. Round-trip sem patch preservando o arquivo.
5. Rebuild com patches pequenos validado.
6. Teste de regressão.
7. Documentação de limites.

## Regra de ouro

O KotobaParse deve ser poderoso o suficiente para parser de VN, mas restrito o suficiente para ser previsível, testável, seguro para UI e portável para Rust/Tauri/WASM.
