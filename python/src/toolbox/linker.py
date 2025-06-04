import importlib
import logging
import re
import sys

_log = logging.getLogger(__name__)


def reload_modules(pattern: str) -> None:
    """
    Reload modules by name pattern.

    Parameters
    ----------
    pattern
        Name Pattern.

    Examples
    --------
    >>> import glatzel
    >>> from glatzel import importtool
    >>> importtool.reload("glatzel")
    """
    for module_name in list(sys.modules.keys()):
        if re.match(pattern=pattern, string=module_name):
            _log.info(f"Reload module: {pattern}")
            importlib.reload(sys.modules[module_name])
