pixi install
new-item $PSScriptRoot/../temp -ItemType Directory -ErrorAction SilentlyContinue
copy-item D:\project\toolbox\tools\orc\.pixi\envs\default\Library\bin\raw_r.dll -Destination $PSScriptRoot/../temp
