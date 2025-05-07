$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..
cargo clippy --fix --all
Set-Location $ROOT
