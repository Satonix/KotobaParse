# KotobaParse Rust Runtime v0.2.0-alpha.1

Primeira rodada prática do runtime Rust usando amostras reais fornecidas pelo projeto.

## Compatibilidade adicionada

- Leitura de bytes com `encoding` do parser `.kotoba`.
- `utf-8`, `utf-8-sig`, `utf-16`, `utf-16le`, `utf-16be`, `cp932`/`shift-jis` e `windows-1252`.
- `extract_bytes(...)` para extrair scripts sem depender de `String` UTF-8 já pronta.
- `rebuild_bytes(...)` para round-trip respeitando encoding configurado.
- `extract_binary_ascii_strings(...)` e `extract_binary_text_entries(...)` como base experimental para `.nut`/scripts binários.

## Fixtures derivadas das amostras reais

Os testes Rust agora cobrem pequenos recortes sanitizados em:

```txt
tests/fixtures/real/aokana_ep01_excerpt.bs5
tests/fixtures/real/ef_100_01_excerpt.sc
tests/fixtures/real/array_strings_excerpt.json
tests/fixtures/real/majikoi_act_b_excerpt.txt
tests/fixtures/real/nut_binary_excerpt.bin
```

Os arquivos completos enviados pelo usuário não foram incorporados como fixtures públicas. Isso evita colocar scripts inteiros de jogos no repositório e mantém os testes pequenos.

## Parsers novos

```txt
examples/json_array_strings.kotoba
examples/majikoi_plaintext.kotoba
tests/fixtures/parsers/json_array_strings.kotoba
tests/fixtures/parsers/majikoi_txt.kotoba
```

## Limitações atuais

- O runtime Rust ainda é uma implementação inicial.
- O parser `.nut` binário ainda não faz reinjeção por offset. Ele apenas lista strings textuais prováveis.
- O modo `collect quoted` da DSL ainda precisa ser portado para Rust.
- Não rodei `cargo test` neste ambiente porque `cargo`/`rustc` não estão instalados no sandbox.
