import logging
import time

__log = logging.getLogger(__name__)


def timer(func):
    """
    Time counter decorator.

    Parameters
    ----------
    func
        Function.

    Returns
    -------
    Callable
    """

    def func_wrapper(*args, **kwargs):
        time_start = time.perf_counter()
        result = func(*args, **kwargs)
        time_end = time.perf_counter()
        time_spend = time_end - time_start
        __log.info(f"{func.__name__} cost time: {time_spend:.3f} s")
        return result

    return func_wrapper
