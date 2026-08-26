# KotobaParse v0.2.0-alpha.8 - GLS/NUT binary string blocks

This alpha adds an experimental binary block program construct for compiled GLS `.nut` scripts.

The first supported binary layout is the one observed in `ma00_000.nut`:

```text
10 00 00 08 + u32le byte length + UTF-8 payload
```

Example parser:

```kotoba
kotoba GLSNutScript {
    target ".nut";

    binary string_block {
        magic "10 00 00 08";
        length u32le;
        encoding utf-8;
        min_len 4;
        profile gls_nut;
    }
}
```

Run extraction:

```powershell
cargo run -p kotoba-cli -- extract examples\gls_nut_program.kotoba ma00_000.nut out_nut.json
```

The `gls_nut` profile tries to normalize payloads for translation:

- preserves raw payload in `fields.raw`;
- emits `payload_offset`, `prefix_offset`, and original byte `length`;
- extracts speaker from `//【speaker】` when present;
- extracts voice path from `src='voice/...'` when present;
- removes voice tags and Japanese source comment lines from the primary `text` field.

This is extraction-only for now. Binary rebuild is intentionally not enabled yet, because arbitrary length changes may require updating additional binary/chunk metadata.
