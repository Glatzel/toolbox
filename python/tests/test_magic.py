import time

from toolbox import magic


def test_timer():
    @magic.timer
    def foo():
        time.sleep(0.1)

    foo()
