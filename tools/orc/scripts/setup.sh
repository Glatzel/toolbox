#!/bin/sh

set -e

rm -rf /.pixi

pixi install

case "$(uname -s)" in
    MINGW*|MSYS*|CYGWIN*)
        rm -f ".pixi/envs/default/Library/bin/jpeg8.dll"
        rm -f ".pixi/envs/default/Library/bin/api-ms-win*.dll"
        rm -f ".pixi/envs/default/api-ms-win-crt-runtime*.dll"
        ;;
    Linux)
        rm -f ".pixi/envs/default/lib/libjpeg.so.8.*"
        ;;
esac
