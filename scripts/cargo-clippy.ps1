Set-Location $PSScriptRoot
Set-Location ..

if ($env:CI) {
    cargo clippy --all-targets --all-features
}
else {
    cargo clippy --fix --all-targets --all-features
}
