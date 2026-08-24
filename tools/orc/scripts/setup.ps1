rename-item $PSScriptRoot/../_pixi.toml pixi.toml
rename-item $PSScriptRoot/../_pixi.lock pixi.lock
pixi reinstall --frozen
rename-item $PSScriptRoot/../pixi.toml _pixi.toml
rename-item $PSScriptRoot/../pixi.lock _pixi.lock
if ($IsWindows) {
    Remove-Item $PSScriptRoot/../.pixi/envs/default/Library/bin/jpeg8.dll -ErrorAction SilentlyContinue
    Remove-Item $PSScriptRoot/../.pixi/envs/default/Library/bin/api-ms-win*.dll -ErrorAction SilentlyContinue
    Remove-Item $PSScriptRoot/../.pixi/envs/default/api-ms-win-crt-runtime*.dll -ErrorAction SilentlyContinue
}
if ($IsLinux) {
    Remove-Item $PSScriptRoot/../.pixi/envs/default/lib/libjpeg.so.8.* -ErrorAction SilentlyContinue
}
