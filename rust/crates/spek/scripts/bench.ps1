Set-Location $PSScriptRoot/..
$env:FFT_SIGNAL_SIZE=5
$env:STFT_SIGNAL_SIZE=7
$env:FFT_SIZE=2048
cargo bench --all-features -p spek @args
