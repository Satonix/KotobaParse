$ErrorActionPreference = "Stop"
Set-Location -LiteralPath (Split-Path -Parent $PSScriptRoot)
cargo test --workspace
cargo run -p kotoba-cli -- check examples\ef_message.kotoba
cargo run -p kotoba-cli -- extract examples\ef_message.kotoba tests\fixtures\ef\sample.sc "$env:TEMP\kotoba_entries.json"
Write-Host "KotobaParse checks passed."
