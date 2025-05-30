$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..
if (-not $env:CI) {
    cargo +stable clippy --fix
}
cargo +stable clippy -- -Dwarnings
Set-Location $ROOT
