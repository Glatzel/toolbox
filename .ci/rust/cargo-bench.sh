# This File is automatically synchronized from https://github.com/Glatzel/template

set -eu
CARGO_BENCH_OPTIONS_DEFAULT="--all-features
--config profile.release.debug=false
--config profile.release.lto="fat"
--config profile.release.codegen-units=1
--config profile.release.strip=true
--config profile.release.opt-level=3
--config build.rustflags=["-C","target-cpu=native"]
"
CARGO_BENCH_OPTIONS="${CARGO_BENCH_OPTIONS:-CARGO_BENCH_OPTIONS_DEFAULT}"

if [ -f ./scripts/setup.sh ]; then
    . ./scripts/setup.sh
fi

if [ "${CI:-}" ]
then
    pixi global install -c https://prefix.dev/glatzel cargo-codspeed \
    cargo codspeed build $CARGO_BENCH_OPTIONS
else
    cargo bench $CARGO_BENCH_OPTIONS
fi
