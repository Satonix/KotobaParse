param(
  [string]$Version = "0.3.0-alpha.2",
  [string]$Channel = "alpha",
  [string]$Platform = "windows-x64",
  [switch]$GenerateSql
)

$ErrorActionPreference = "Stop"
$Root = Resolve-Path (Join-Path $PSScriptRoot "..")
Set-Location $Root

cargo build -p kotoba-cli --release

$OutRoot = Join-Path $Root "dist"
$PackageName = "kotobaparse-v$Version-$Platform"
$PackageDir = Join-Path $OutRoot $PackageName
$ZipPath = Join-Path $OutRoot "$PackageName.zip"

if (Test-Path $PackageDir) { Remove-Item -Recurse -Force $PackageDir }
if (Test-Path $ZipPath) { Remove-Item -Force $ZipPath }
New-Item -ItemType Directory -Force $PackageDir | Out-Null
New-Item -ItemType Directory -Force (Join-Path $PackageDir "examples") | Out-Null

Copy-Item (Join-Path $Root "target\release\kotoba.exe") (Join-Path $PackageDir "kotoba.exe") -Force
Copy-Item (Join-Path $Root "README.md") (Join-Path $PackageDir "README.md") -Force -ErrorAction SilentlyContinue
Copy-Item (Join-Path $Root "examples\*.kotoba") (Join-Path $PackageDir "examples") -Force -ErrorAction SilentlyContinue

Compress-Archive -Path (Join-Path $PackageDir "*") -DestinationPath $ZipPath -Force
$Hash = (Get-FileHash $ZipPath -Algorithm SHA256).Hash.ToLowerInvariant()
$Size = (Get-Item $ZipPath).Length
$FileName = Split-Path $ZipPath -Leaf
$AssetPath = "kotobaparse/releases/$FileName"

$Manifest = [ordered]@{
  version = $Version
  channel = $Channel
  platform = $Platform
  filename = $FileName
  asset_path = $AssetPath
  sha256 = $Hash
  size_bytes = $Size
  notes = "KotobaParse runtime $Version"
}
$ManifestPath = Join-Path $OutRoot "$PackageName.release.json"
$Manifest | ConvertTo-Json -Depth 5 | Set-Content -Encoding UTF8 $ManifestPath

$LatestPath = Join-Path $OutRoot "latest-$Platform.release.json"
$Manifest | ConvertTo-Json -Depth 5 | Set-Content -Encoding UTF8 $LatestPath

Write-Host "Runtime criado:" $ZipPath
Write-Host "SHA256:" $Hash
Write-Host "Release JSON:" $ManifestPath
Write-Host "Latest JSON:" $LatestPath
Write-Host "Suba o ZIP e o .release.json para hub/kotobaparse/releases/. Não precisa mexer no banco."

if ($GenerateSql) {
  $SqlPath = Join-Path $OutRoot "$PackageName.release.sql"
@"
INSERT INTO hub_kotobaparse_runtime_releases
(version, channel, platform, filename, asset_path, sha256, size_bytes, notes, is_active)
VALUES
('$Version', '$Channel', '$Platform', '$FileName', '$AssetPath', '$Hash', $Size, 'KotobaParse runtime $Version', 1)
ON DUPLICATE KEY UPDATE
filename = VALUES(filename), asset_path = VALUES(asset_path), sha256 = VALUES(sha256), size_bytes = VALUES(size_bytes), notes = VALUES(notes), is_active = 1;
"@ | Set-Content -Encoding UTF8 $SqlPath
  Write-Host "SQL opcional:" $SqlPath
}
