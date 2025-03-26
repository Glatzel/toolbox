$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..
if ($env:CI) {
    cargo clippy --all-targets --all-features
}
else {
    cargo clippy --fix --all-targets --all-features
}
Set-Location $ROOT
