import logging

from rich.pretty import pretty_repr
from toolbox import cycler

log = logging.getLogger()


def test_remove_empty():
    d = {
        "a": "",
        "b": [],
        "c": 123,
        "d": "456",
        "e": {},
        "f": [
            None,
            123456,
            "",
            "abc",
            [],
            {
                "f.a": "",
                "f.b": [],
                "f.c": 123,
                "f.d": "456",
                "f.e": [123, None, []],
            },
        ],
    }
    test_d = cycler.remove_empty(item=d)
    expected_d = {
        "c": 123,
        "d": "456",
        "f": [
            123456,
            "abc",
            {
                "f.c": 123,
                "f.d": "456",
                "f.e": [123],
            },
        ],
    }
    log.debug(pretty_repr(locals()))
    assert test_d == expected_d
