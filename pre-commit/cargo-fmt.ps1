# This File is automatically synchronized from https://github.com/Glatzel/template

rustup toolchain install nightly --profile=minimal
rustup component add rustfmt --toolchain nightly
$ROOT = git rev-parse --show-toplevel
Set-Location $ROOT
foreach ($f in Get-ChildItem "Cargo.lock" -Recurse) {
    Set-Location $f.Directory.ToString()
    Write-Output "Cargo fmt in: $pwd"
    cargo +nightly fmt --all --all-features
}
Set-Location $ROOT
