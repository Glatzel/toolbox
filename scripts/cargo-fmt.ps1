Set-Location $PSScriptRoot
Set-Location ..

if ($env:CI) {
    cargo +nightly fmt --all -- --check
}
else {
    cargo +nightly fmt --all
}
