$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..

cargo machete --with-metadata
Set-Location $ROOT