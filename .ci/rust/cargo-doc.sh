# This File is automatically synchronized from https://github.com/Glatzel/template

set -eu

CARGO_DOC_OPTIONS="${CARGO_DOC_OPTIONS:---all-features}"
CARGO_DOC_ARGS="${CARGO_DOC_ARGS:-}"

if [ -f ./scripts/setup.sh ]; then
    . ./scripts/setup.sh
fi

RUSTDOCFLAGS="-D warnings" cargo doc $CARGO_DOC_OPTIONS -- $CARGO_DOC_ARGS
