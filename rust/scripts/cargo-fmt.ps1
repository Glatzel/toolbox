$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..
cargo fmt --all
Set-Location $ROOT
