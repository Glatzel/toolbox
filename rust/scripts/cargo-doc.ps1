# This File is automatically synchronized from https://github.com/Glatzel/template

$config = if ($args.Count) { $args } else { @('--no-deps', '--workspace', '--all-features') }
Set-Location $PSScriptRoot/..
cargo doc @config
