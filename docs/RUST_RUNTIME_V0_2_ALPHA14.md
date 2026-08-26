# KotobaParse v0.2.0-alpha.14

Correção focada no rebuild de blocos multilinha.

## Corrigido

- `rebuild_with_report` agora reconstrói o texto do bloco a partir do corpo lógico das linhas antes de aplicar `match` multilinha.
- Evita inserir linhas em branco artificiais quando a origem vem de `split_preserving_newline()`.
- Preserva a quebra final do bloco após aplicar patch.

## Teste de regressão alvo

- `language_runtime_extracts_multiline_blocks` deve passar com `cargo test --workspace`.
