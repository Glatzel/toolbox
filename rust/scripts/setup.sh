git submodule update --init

if [ "$(uname -s)" = "Linux" ]; then
    pixi install
    export PKG_CONFIG_PATH="$(realpath ./.pixi/envs/default/lib/pkgconfig)"
fi
