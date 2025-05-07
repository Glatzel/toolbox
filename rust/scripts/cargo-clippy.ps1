$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..
cargo +stable clippy --fix --all
Set-Location $ROOT
