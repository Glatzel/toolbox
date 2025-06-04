$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot
Set-Location $PSScriptRoot
pixi run rattler-build build
Set-Location $ROOT
