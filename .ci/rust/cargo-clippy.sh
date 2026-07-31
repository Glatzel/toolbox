# This File is automatically synchronized from https://github.com/Glatzel/template

set -eu

CARGO_CLIPPY_OPTIONS="${CARGO_CLIPPY_OPTIONS:---workspace --all-features}"
CARGO_CLIPPY_ARGS="${CARGO_CLIPPY_ARGS:--D warnings}"

if [ -f ./scripts/setup.sh ]; then
    . ./scripts/setup.sh
fi

cargo clippy $CARGO_CLIPPY_OPTIONS -- $CARGO_CLIPPY_ARGS
