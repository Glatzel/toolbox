$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot

& "$PSScriptRoot/../scripts/pytest.ps1"

Set-Location $PSScriptRoot
pixi run rattler-build build
Set-Location $ROOT
