# This File is automatically synchronized from https://github.com/Glatzel/template

$ErrorActionPreference = "Stop"
$PSNativeCommandUseErrorActionPreference = $true
$ROOT = git rev-parse --show-toplevel
Set-Location $ROOT
foreach ($f in Get-ChildItem "Cargo.lock" -Recurse) {
    # skip target and package folder
    if ($f -contains "target") { continue }
    if ($f -contains "crate") { continue }

    Set-Location $f.Directory.ToString()
    Write-Output "Cargo fmt in: $pwd"
    if (Test-Path ./scripts/setup.ps1) {
        &./scripts/setup.ps1
        Set-Location $f.Directory.ToString()
    }
    cargo +stable clippy --fix --all-features
    cargo +stable clippy --all-features -- -Dwarnings
    Set-Location $ROOT
}
