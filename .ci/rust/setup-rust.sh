# This File is automatically synchronized from https://github.com/Glatzel/template

set -eu

TOOLCHAIN="${TOOLCHAIN:-stable}"
COMPONENTS="${COMPONENTS:-}"
TARGETS="${TARGETS:-}"

if ! command -v rustup >/dev/null 2>&1; then
    pixi global install \
        -c https://prefix.dev/glatzel \
        -c conda-forge \
        rustup-init

    rustup-init -y \
        --default-toolchain "$TOOLCHAIN" \
        --profile minimal
else
    rustup toolchain install "$TOOLCHAIN" --profile minimal
    rustup default "$TOOLCHAIN"
fi
. "$HOME/.cargo/env"
if [ -n "$COMPONENTS" ]; then
    rustup component add $COMPONENTS
fi

if [ -n "$TARGETS" ]; then
    rustup target add $TARGETS
fi
