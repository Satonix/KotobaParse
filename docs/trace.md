# Inspect and Trace

`inspect` gives a coverage summary:

```bash
python -m kotobaparse.cli inspect script.txt --parser parser.kotoba
```

It reports:

- total lines
- extracted entries
- global skips
- unmatched lines
- entries by type
- hits per rule

`trace` gives per-line decisions:

```bash
python -m kotobaparse.cli trace script.txt --parser parser.kotoba --out trace.json
```

Each event includes:

- source line
- payload
- outcome: `rule`, `global_skip`, or `unmatched`
- matched rule
- captures
- number of entries created

This is meant for future Parser Studio integration in SekaiTranslator.
