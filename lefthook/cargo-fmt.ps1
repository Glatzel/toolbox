# This File is automatically synchronized from https://github.com/Glatzel/template

if ($env:CI) {
    rustup toolchain install nightly --profile=minimal
    rustup component add rustfmt --toolchain nightly
}

$ROOT = git rev-parse --show-toplevel
Set-Location $ROOT
foreach ($f in Get-ChildItem "Cargo.lock" -Recurse) {
    # skip target and package folder
    if ($f -contains "target") { continue }
    if ($f -contains "crate") { continue }

    Set-Location $f.Directory.ToString()
    Write-Output "Cargo fmt in: $pwd"
    cargo +nightly fmt --all
}
Set-Location $ROOT
