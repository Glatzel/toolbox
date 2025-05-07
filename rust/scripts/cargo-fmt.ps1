$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot/..

cargo +nightly fmt --all

Set-Location $ROOT
