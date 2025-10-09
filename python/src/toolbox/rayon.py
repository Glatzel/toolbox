import multiprocessing
from collections.abc import Callable, Iterable
from concurrent.futures import ThreadPoolExecutor
from typing import Any, Literal


def execute_processes(fn: Callable, params: Iterable[Iterable[Any]]) -> Iterable:
    r"""
    Execute tasks concurrently wrapping multiprocessing.

    Parameters
    ----------
    fn
        A callable that will take as many arguments as there are passed iterables.
    params
        Iterable parameters of `fn`.

    Returns
    -------
    list[Unknown]
        An iterator equivalent to: starmap but the calls may be evaluated out-of-order.

    Examples
    --------
    >>> from glatzel import parallel
    >>> def long_time_task(name):
    ...     start = time.time()
    ...     time.sleep(random.random() * 3)
    ...     end = time.time()
    ...     print("Task %s runs %0.2f seconds." % (name, (end - start)))
    >>> parallel.execute_processes(long_time_task, zip(range(5)))
    Task 0 runs 0.11 seconds.
    Task 4 runs 0.34 seconds.
    Task 1 runs 1.76 seconds.
    Task 3 runs 2.30 seconds.
    Task 2 runs 2.84 seconds.
    """
    result: list
    with multiprocessing.Pool(processes=multiprocessing.cpu_count()) as pool:
        result = pool.starmap(fn, params)
    return result


def execute_threads(fn: Callable, params: Iterable[Iterable[Any]], max_workers: int = 32) -> Iterable:
    r"""
    Execute tasks concurrently wrapping thread.

    Parameters
    ----------
    fn
        A callable that will take as many arguments as there are passed iterables.
    params
        Iterable parameters of `fn`.
    max_workers
        The maximum number of threads that can be used to execute the given calls.

    Returns
    -------
    list[Unknown]
        An iterator equivalent to: starmap but the calls may be evaluated out-of-order.

    Examples
    --------
    >>> from glatzel import parallel
    >>> def long_time_task(name):
    ...     start = time.time()
    ...     time.sleep(random.random() * 3)
    ...     end = time.time()
    ...     print("Task %s runs %0.2f seconds." % (name, (end - start)))
    >>> parallel.execute_threads(long_time_task, zip(range(5)))
    Task 0 runs 0.11 seconds.
    Task 4 runs 0.34 seconds.
    Task 1 runs 1.76 seconds.
    Task 3 runs 2.30 seconds.
    Task 2 runs 2.84 seconds.
    """
    with ThreadPoolExecutor(max_workers) as executor:
        return executor.map(lambda t: fn(*t), params)


class ParallelExecutor:
    r"""
    A concurrent executor inspired concurrent.futures.

    Parameters
    ----------
    fn
        A callable that will take as many arguments as there are passed iterables.
    mode
        Choose multiprocessing or thread mode.

    Examples
    --------
    >>> from glatzel import parallel
    >>> def foo(a, b, c):
    ...     print(a, b, c)
    >>> executor = parallel.ParallelExecutor(foo, "thread")
    >>> executor.append(1, 2, 3)
    >>> executor.append(4, 5, 6)
    >>> executor.run()
    1, 2, 3
    4, 5, 6
    """

    def __init__(self, fn, mode: Literal["process", "thread"] = "process") -> None:
        self.fn = fn
        self.params = []
        self.method = mode

    def append(self, *params) -> None:
        r"""
        Append parameter set.

        Parameters
        ----------
        *params
            Parameters.
        """
        self.params.append(params)

    def run(self) -> Iterable:
        r"""
        Execute fn.

        Returns
        -------
        list[Any]
            An iterator equivalent to: starmap but the calls may be evaluated out-of-order.
        """
        result: Iterable = []
        match self.method:
            case "process":
                result = execute_processes(self.fn, self.params)
            case "thread":
                result = execute_threads(self.fn, self.params)
        return result
