pixi install
new-item $PSScriptRoot/../temp -ItemType Directory -ErrorAction SilentlyContinue
copy-item PSScriptRoot/../.pixi/envs/default/Library/bin/raw_r.dll -Destination $PSScriptRoot/../temp
copy-item PSScriptRoot/../.pixi/envs/default/Library/bin/lcms2.dll -Destination $PSScriptRoot/../temp
copy-item PSScriptRoot/../.pixi/envs/default/Library/bin/jpeg8.dll -Destination $PSScriptRoot/../temp
