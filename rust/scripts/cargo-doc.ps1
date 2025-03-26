$ROOT = git rev-parse --show-toplevel
Set-Location $PSScriptRoot/..

cargo doc --no-deps --all


Remove-Item ./dist/rust-doc.zip -Force -ErrorAction SilentlyContinue
New-Item ./dist -ItemType Directory -ErrorAction SilentlyContinue
Compress-Archive ./target/doc "./dist/rust-doc.zip"
Set-Location $ROOT