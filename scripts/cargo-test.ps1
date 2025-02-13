Set-Location $PSScriptRoot
Set-Location ..
& $PSScriptRoot/set-env.ps1

write-output "::group::nextest"
cargo +nightly llvm-cov --all-features --workspace nextest
$code = $LASTEXITCODE
Write-Output "::endgroup::"

write-output "::group::test"
cargo +nightly llvm-cov --all-features --workspace --doc
$code = $code + $LASTEXITCODE
Write-Output "::endgroup::"

write-output "::group::report"
cargo +nightly llvm-cov report
Write-Output "::endgroup::"

write-output "::group::report lcov"
if ( $env:CI ) {
    cargo +nightly llvm-cov report --lcov --output-path lcov.info
}
Write-Output "::endgroup::"

$code = $code + $LASTEXITCODE
Write-Output $code
exit $code
