$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..
$check = cargo +nightly fmt --all --check
if ($check){
    cargo +nightly fmt --all
    exit 1
}
Set-Location $ROOT
