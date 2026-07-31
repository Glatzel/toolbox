# This File is automatically synchronized from https://github.com/Glatzel/template

set -eu

CARGO_BUILD_OPTIONS="${CARGO_BUILD_OPTIONS:---all-features}"

if [ -f ./scripts/setup.sh ]; then
    . ./scripts/setup.sh
fi

cargo build $CARGO_BUILD_OPTIONS
