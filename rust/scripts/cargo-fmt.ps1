$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot/..

if ($env:CI) {
    cargo +nightly fmt --all -- --check
}
else {
    cargo +nightly fmt --all
}

Set-Location $ROOT
