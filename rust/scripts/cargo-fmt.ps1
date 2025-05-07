$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..

$a=cargo +nightly fmt --all
Write-Output $a
Set-Location $ROOT
