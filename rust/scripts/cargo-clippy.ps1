$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..
cargo clippy --fix --all-targets --all-features
Set-Location $ROOT
