import logging

import path_macro

import python.clerk.clerk as clerk


def test_rich_handler():
    rich_handle = logtool.rich_handler()
    logging.basicConfig(handlers=[rich_handle])
    log = logging.getLogger()
    log.debug("This is a debug-level message")
    log.info("This is an info-level message")
    log.warning("This is a warning-level message")
    log.error("This is an error-level message")
    log.critical("This is a critical-level message")


def test_file_handler():
    rich_handle = logtool.file_handler(logfile_dir=path_macro.TEMP_DIR / "tests/logtool")
    logging.basicConfig(handlers=[rich_handle])
    log = logging.getLogger()
    log.debug("This is a debug-level message")
    log.info("This is an info-level message")
    log.warning("This is a warning-level message")
    log.error("This is an error-level message")
    log.critical("This is a critical-level message")
