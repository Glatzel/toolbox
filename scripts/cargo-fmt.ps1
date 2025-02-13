Set-Location $PSScriptRoot
Set-Location ..

if ($env:CI) {
    cargo +nightly fmt --all -- --check
}
else {
    & $PSScriptRoot/set-env.ps1
    cargo +nightly fmt --all
}
