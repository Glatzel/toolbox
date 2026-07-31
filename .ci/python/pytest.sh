# This File is automatically synchronized from https://github.com/Glatzel/template

set -eu

PIXI_ENVIRONMENT=${PIXI_ENVIRONMENT:-default}

DEFAULT_PYTEST_ARGS="
./tests
--color=yes
--cov
--cov-report term
--cov-report=xml:tests_report/coverage.xml
--durations=10
--junitxml=junit.xml
--maxfail 0
--verbose
"

if [ -z "${CI:-}" ]; then
    DEFAULT_PYTEST_ARGS="$DEFAULT_PYTEST_ARG --cov-report=html:tests_report/htmlcov"
fi
PYTEST_ARGS=${PYTEST_ARGS:-"$DEFAULT_PYTEST_ARGS"}

# shellcheck disable=SC2086
pixi run -e "$PIXI_ENVIRONMENT" pytest $PYTEST_ARGS
