$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..

cargo +nightly fmt --all --check
cargo +nightly fmt --all
Set-Location $ROOT
