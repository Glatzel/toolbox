from toolbox import rayon


def foo(x):
    return x + 1


def test_execute_processes():
    result = rayon.execute_processes(fn=foo, params=([1], [2], [3]))
    assert result == [2, 3, 4]


def test_execute_threads():
    result = list(rayon.execute_threads(fn=foo, params=([1], [2], [3])))
    assert result == [2, 3, 4]


class TestrayonExecutor:
    def test_process(self):
        executor = rayon.ParallelExecutor(fn=foo)
        executor.append(1)
        executor.append(2)
        executor.append(3)
        result = executor.run()
        assert result == [2, 3, 4]

    def test_thread(self):
        executor = rayon.ParallelExecutor(fn=foo, mode="thread")
        executor.append(1)
        executor.append(2)
        executor.append(3)
        result = list(executor.run())
        assert result == [2, 3, 4]
