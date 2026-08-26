# KotobaParse Feature Matrix v0.2 Draft

| Área | Recurso | Status alpha.9 | Prioridade |
|---|---|---:|---:|
| Linguagem | `kotoba Nome {}` | Implementado | Alta |
| Linguagem | formato legado | Implementado | Alta |
| Linguagem | `use` módulos | Reservado | Média |
| Linguagem | `state`, `set`, `let` | Planejado | Alta |
| Linguagem | `when`, `if` | Planejado | Alta |
| Extração | regras linha a linha | Implementado | Alta |
| Extração | captures tipadas | Implementado | Alta |
| Extração | `emit Dialogue/Narration` | Implementado | Alta |
| Extração | `emit ChoiceGroup` | Implementado inicial | Média |
| Extração | blocos multilinha | Planejado | Alta |
| Extração | JSON por path | Planejado | Alta |
| Extração | CSV/tabela | Planejado | Média |
| Encoding | UTF-8/BOM | Implementado | Alta |
| Encoding | UTF-16 LE/BE | Implementado | Alta |
| Encoding | CP932/Shift-JIS | Implementado | Alta |
| Tags | `protect` | Implementado declarativo | Alta |
| Tags | validação origem/tradução | Planejado | Alta |
| Rebuild | patch por campo | Implementado | Alta |
| Rebuild | preview | Implementado | Alta |
| Rebuild | report | Implementado | Alta |
| Rebuild | JSON path | Planejado | Alta |
| Rebuild | binário same-length | Planejado | Média |
| Rebuild | binário resize chunk | Planejado | Média |
| Editor | diagnose JSON | Implementado | Alta |
| Editor | summary/outline | Implementado | Alta |
| Editor | language-spec JSON | Implementado | Alta |
| CLI | check/diagnose/summary | Implementado | Alta |
| CLI | extract/rebuild/preview | Implementado | Alta |
| CLI | inspect/trace Rust | Planejado | Média |

## Alpha 11 implementation note

The Rust runtime now implements the editor-critical subset of the formal language:

| Feature | Alpha 11 status |
|---|---|
| `state` | implemented for line/block extraction context |
| `remember` / `forget` | implemented |
| `set` / `let` | implemented for captures, state values and literals |
| `when` | implemented for `exists`, `missing`, `==`, `!=`, `contains` |
| `block` | experimental extraction and simple field rebuild |
| `json` | experimental structured extraction/rebuild by simple JSON paths |
| `transform` | experimental extract/rebuild replacement transforms |
| `protect` | implemented as rebuild warning checks |
| `rebuild strategy` | parsed and surfaced to editor summaries |
| `editor-probe` | implemented in CLI |

This is enough to start building the SekaiTranslator KotobaParse editor around diagnostics, outline, extraction preview and rebuild preview. Complex binary rebuild and full Python parity are still outside this alpha.
