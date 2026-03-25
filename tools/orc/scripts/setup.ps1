Remove-Item $PSScriptRoot/../.pixi -Recurse -Force -ErrorAction SilentlyContinue
pixi install
Remove-Item $PSScriptRoot/../.pixi/envs/default/Library/bin/jpeg8.dll -ErrorAction SilentlyContinue
Remove-Item $PSScriptRoot/../.pixi/envs/default/Library/bin/api-ms-win*.dll -ErrorAction SilentlyContinue
