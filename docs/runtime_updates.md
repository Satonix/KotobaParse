# KotobaParse Runtime Updates

KotobaParse can be distributed as an updateable runtime for SekaiTranslator.
This lets the parser engine evolve without requiring a full SekaiTranslator
update.

## Build the runtime ZIP

From the KotobaParse repository root:

```bash
python tools/build_runtime.py
```

This generates:

```text
dist/kotobaparse-runtime-v<version>.zip
dist/kotobaparse-runtime-v<version>.manifest.json
```

The ZIP layout is:

```text
manifest.json
kotobaparse/
examples/
bundled_parsers/
std/
README.md
LICENSE
```

## Manual test in SekaiTranslator

Extract the runtime ZIP into:

```text
%LOCALAPPDATA%\SekaiTranslatorV\runtimes\kotobaparse\versions\<version>\
```

Create:

```text
%LOCALAPPDATA%\SekaiTranslatorV\runtimes\kotobaparse\current.json
```

Example:

```json
{
  "version": "0.1.0.dev11",
  "path": "versions/0.1.0.dev11"
}
```

SekaiTranslator should then load this runtime before falling back to the
KotobaParse installed in the Python environment.

## Release flow

1. Update KotobaParse.
2. Run tests.
3. Run `python tools/build_runtime.py`.
4. Upload the generated ZIP and manifest to a GitHub Release.
5. SekaiTranslator can later download the ZIP, validate `sha256`, extract it
   into `versions/<version>`, and update `current.json`.

## Compatibility contract

The runtime should keep these public API exports stable:

- `load_parser`
- `load_parser_string`
- `check_parser`
- `extract_file`
- `extract_string`
- `template_file`
- `inject_file`
- `inspect_file`
- `trace_file`
- `characters_file`

Breaking these APIs should require a new `api_version`.

## Pacote para updater do SekaiTranslator

Para gerar um zip do runtime Windows usado pelo Hub:

```powershell
cd "S:\Sekai Visual Novel\Ferramentas\Github Sekai\KotobaParse"
.\tools\package_windows_runtime.ps1
```

O script cria em `dist/`:

- `kotobaparse-v0.2.0-alpha.12-windows-x64.zip`
- `kotobaparse-v0.2.0-alpha.12-windows-x64.release.json`
- `kotobaparse-v0.2.0-alpha.12-windows-x64.release.sql`

Suba o `.zip` para `hub/kotobaparse/releases/` na VPS e aplique o `.sql` no banco `sekaivn`.
