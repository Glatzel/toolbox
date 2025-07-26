Set-Location $PSScriptRoot/..
cargo +nightly miri test -p envoy
