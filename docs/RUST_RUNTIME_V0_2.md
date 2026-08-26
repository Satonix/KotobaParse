# KotobaParse v0.2 Rust Runtime

This is the first Rust implementation of KotobaParse. It is designed as a standalone crate so any tool can use the parser DSL without depending on SekaiTranslator.

## Separation from SekaiTranslator

KotobaParse owns:

- `.kotoba` grammar parsing;
- rule matching;
- extraction;
- rebuild/patch application;
- round-trip and diagnostics contracts.

SekaiTranslator owns:

- projects;
- UI;
- Hub/login/sync/locks;
- translation workflow;
- local cache and export UX.

## Local path usage from SekaiTranslatorV2

Development layout:

```txt
Parent/
  KotobaParse/
  SekaiTranslatorV2/
```

SekaiTranslatorV2 consumes:

```toml
kotoba-core = { path = "../KotobaParse/crates/kotoba-core" }
```

## Regression rule

Any DSL behavior that affects extraction or rebuild must have a fixture in this repository.
