if ($IsLinux ) {
    pixi install
    $env:PKG_CONFIG_PATH= Resolve-Path "./.pixi/envs/default/lib/pkgconfig"
}
