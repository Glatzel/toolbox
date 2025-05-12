from __future__ import annotations

import datetime
import logging
from logging import INFO, FileHandler
from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from rich.logging import RichHandler


def rich_handler(level: int = INFO) -> RichHandler:
    r"""
    Initialize a RichHandler.

    Parameters
    ----------
    level
        Log level.

    Returns
    -------
    RichHandler
    """
    from rich.console import Console
    from rich.logging import RichHandler
    from rich.theme import Theme

    theme = {
        "logging.level.warning": "yellow",
        "log.time": "cyan",
    }
    handler = RichHandler(
        log_time_format=r"%Y-%m-%d %H:%M:%S",
        rich_tracebacks=True,
        tracebacks_show_locals=True,
        omit_repeated_times=False,
        console=Console(theme=Theme(theme)),
    )
    handler.setLevel(level)
    handler.setFormatter(logging.Formatter("%(message)s"))
    return handler


def file_handler(level: int = INFO, logfile_dir: str | Path = "./logs") -> FileHandler:
    r"""
    Initialize a FileHandler.

    Parameters
    ----------
    level
        Log level.
    logfile_dir
        Directory to save log file.

    Returns
    -------
    FileHandler
    """
    logfile_dir = Path(logfile_dir)
    logfile_dir.mkdir(parents=True, exist_ok=True)
    logfile_name = datetime.datetime.now().strftime(r"%Y-%m-%d %H-%M-%S.log")
    logfile = logfile_dir / logfile_name
    handler = logging.FileHandler(logfile, encoding="utf-8")
    handler.setLevel(level)
    fmt = "%(asctime)s %(levelname)s %(message)s [%(filename)s:%(lineno)d]"
    datefmt = r"%Y-%m-%d %H:%M:%S"
    handler.setFormatter(logging.Formatter(fmt=fmt, datefmt=datefmt))
    return handler
