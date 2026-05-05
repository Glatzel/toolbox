if ($IsLinux ) {
    pixi install
    $env:PKG_CONFIG_DIR= Resolve-Path "./.pixi/envs/default/lib"
}
