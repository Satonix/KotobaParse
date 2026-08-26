# Publicação automática do runtime sem inserir no banco

O Hub agora consegue detectar releases do KotobaParse diretamente pelos arquivos em:

```text
hub/kotobaparse/releases/
```

## Como gerar

No Windows:

```powershell
cd "S:\Sekai Visual Novel\Ferramentas\Github Sekai\KotobaParse"
.\tools\package_windows_runtime.ps1
```

O script gera em `dist`:

```text
kotobaparse-v0.2.0-alpha.12-windows-x64.zip
kotobaparse-v0.2.0-alpha.12-windows-x64.release.json
latest-windows-x64.release.json
```

## Como publicar

Suba para a VPS:

```text
hub/kotobaparse/releases/kotobaparse-v0.2.0-alpha.12-windows-x64.zip
hub/kotobaparse/releases/kotobaparse-v0.2.0-alpha.12-windows-x64.release.json
```

Não é necessário abrir phpMyAdmin nem aplicar SQL para cada release.

## SQL opcional

Se quiser continuar gerando SQL manual, rode:

```powershell
.\tools\package_windows_runtime.ps1 -GenerateSql
```
