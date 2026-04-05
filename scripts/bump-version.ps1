Set-Location $PSScriptRoot/..
$version = Get-Date -Format 'yyyy.M.d'

# python
$cargoTomlPath = "./python/pyproject.toml"
(Get-Content -Path $cargoTomlPath) -replace '^version = .*', "version = `"$version`"" | Set-Content -Path $cargoTomlPath
Write-Host "Updated python version to $version"

# rust
$cargoTomlPath = "./rust/Cargo.toml"
(Get-Content -Path $cargoTomlPath) -replace '^version = .*', "version = `"$version`"" | Set-Content -Path $cargoTomlPath
Write-Host "Updated rust version to $version"
Set-Location rust
cargo update
Set-Location $PSScriptRoot/..

$cargoTomlPath = "./tools/orc/Cargo.toml"
(Get-Content -Path $cargoTomlPath) -replace '^version = .*', "version = `"$version`"" | Set-Content -Path $cargoTomlPath
Write-Host "Updated orc version to $version"
Set-Location tools/orc
cargo update
Set-Location $PSScriptRoot/..

$cargoTomlPath = "./tools/shook/Cargo.toml"
(Get-Content -Path $cargoTomlPath) -replace '^version = .*', "version = `"$version`"" | Set-Content -Path $cargoTomlPath
Write-Host "Updated shook version to $version"
Set-Location tools/shook
cargo update
Set-Location $PSScriptRoot/..


$cargoTomlPath = "./tools/vinaya/Cargo.toml"
(Get-Content -Path $cargoTomlPath) -replace '^version = .*', "version = `"$version`"" | Set-Content -Path $cargoTomlPath
Write-Host "Updated vinaya version to $version"
Set-Location tools/vinaya
cargo update
Set-Location $PSScriptRoot/..
