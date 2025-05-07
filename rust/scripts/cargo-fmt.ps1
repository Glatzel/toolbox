$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot/..
$result = cargo +nightly fmt --all  -- --check

if ($result) {
    cargo +nightly fmt --all
    throw
}

Set-Location $ROOT
