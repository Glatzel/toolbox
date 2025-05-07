$ErrorActionPreference = "Stop"
Set-Location $PSScriptRoot/..
# $result = cargo +nightly fmt --all  -- --check

# if ($result) {
    cargo +nightly fmt --all
    # exit 1
# }

Set-Location $ROOT
