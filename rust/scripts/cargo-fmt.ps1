$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..
cargo +nightly fmt --all -- --config-path $ROOT
Set-Location $ROOT
