from typing import overload


@overload
def remove_empty(item: list) -> list: ...
@overload
def remove_empty(item: dict) -> dict: ...
def remove_empty(item):
    r"""
    Recursively remove empty lists, empty dicts, or None elements from a list or dictionary.

    Parameters
    ----------
    item
        A list or dict to clear.

    Returns
    -------
    list | dict
        A cleaned list or dict.

    References
    ----------
    https://gist.github.com/nlohmann/c899442d8126917946580e7f84bf7ee7

    Examples
    --------
    >>> from glatzel import gitertool
    >>> gitertool.remove_empty([1, 2, [None, 4]])
    [1, 2, [4]]
    >>> gitertool.remove_empty(
    ...     {"a": {"a1": None, "a2": [], "a3": 1}, "b": {"b1": None, "b2": 2}}
    ... )
    {"a": {"a3": 1}, "b": {"b2": 2}}
    """

    def is_empty(x):
        return x is None or x == {} or x == [] or x == ""

    match item:
        case list():
            return [v for v in (remove_empty(v) for v in item) if not is_empty(v)]
        case dict():
            return {k: v for k, v in ((k, remove_empty(v)) for k, v in item.items()) if not is_empty(v)}
        case _:
            return item
