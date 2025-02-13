Set-Location $PSScriptRoot
Set-Location ..
& $PSScriptRoot/set-env.ps1

cargo +nightly llvm-cov --no-report --all-features --workspace nextest
$code = $LASTEXITCODE
cargo +nightly llvm-cov --no-report --all-features --workspace --doc
$code = $code + $LASTEXITCODE
cargo +nightly llvm-cov report

if ( $env:CI ) {
    cargo +nightly llvm-cov report --lcov --output-path lcov.info
}

$code = $code + $LASTEXITCODE
Write-Output $code
exit $code
