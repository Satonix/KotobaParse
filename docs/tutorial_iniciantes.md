# Tutorial para iniciantes

Este tutorial mostra o caminho mínimo para criar um parser `.kotoba`, extrair textos e reinjetar traduções.

## 1. Estrutura básica

```kotoba
parser SimpleVN
target ".txt"
encoding utf8

skip empty

rule dialogue
    [name=<speaker:quoted>] <text:rest>
    as Dialogue(text, speaker)
    patch text

rule narration
    <text:line>
    as Narration(text)
    patch text
```

A ideia é simples:

- `parser` define o nome do parser.
- `target` define a extensão de arquivo esperada.
- `encoding` define a codificação do script.
- `rule` define um padrão de captura.
- `as Dialogue(...)` ou `as Narration(...)` define o tipo da entrada extraída.
- `patch text` informa onde a tradução será reinjetada.

## 2. Validar o parser

```bash
python -m kotobaparse.cli check examples/indexed_vn.kotoba
```

## 3. Extrair entradas técnicas

```bash
python -m kotobaparse.cli extract script.txt --parser meu_parser.kotoba --out entries.json
```

## 4. Gerar template de tradução

```bash
python -m kotobaparse.cli template script.txt --parser meu_parser.kotoba --out template.json
```

O template gerado possui `original` e `translation`. A tradução deve ser escrita no campo `translation`.

## 5. Reinjetar traduções

```bash
python -m kotobaparse.cli inject script.txt --parser meu_parser.kotoba --translations template.json --out script_patched.txt
```

## 6. Scripts multilíngues

Em formatos com vários idiomas no mesmo arquivo, use `--source-field` para escolher o texto usado como origem:

```bash
python -m kotobaparse.cli template script.bs5 --parser examples/multilingual_bs5.kotoba --source-field jp --out template_jp.json
```

E use `--target-field` para escolher onde a tradução será escrita:

```bash
python -m kotobaparse.cli inject script.bs5 --parser examples/multilingual_bs5.kotoba --translations template_jp.json --target-field en --out script_patched.bs5
```

Isso permite traduzir olhando o japonês, mas reinjetar no campo inglês.

## 7. Diagnóstico

Use `inspect` para ver cobertura:

```bash
python -m kotobaparse.cli inspect script.txt --parser meu_parser.kotoba
```

Use `trace` para gerar dados linha por linha:

```bash
python -m kotobaparse.cli trace script.txt --parser meu_parser.kotoba --out trace.json
```

Esses dados são pensados para integração futura com o Parser Studio do SekaiTranslator.
