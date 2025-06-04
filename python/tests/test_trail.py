import logging

import path_macro
import pytest
from pyfakefs.fake_filesystem import FakeFilesystem
from rich.pretty import pretty_repr
from toolbox import trail

log = logging.getLogger()


@pytest.fixture
def fake_files(fs: FakeFilesystem):
    for i in range(12):
        fs.create_file(f"a/{i}.txt")
    for i in range(11):
        fs.create_file(f"a/img/{i}.png")
    for i in range(2):
        fs.create_file(f"a/img/{i}.jpg")
    return fs


@pytest.mark.parametrize(
    ("pattern", "mode", "count"),
    [
        ("*.txt", "glob", 12),
        (r"\d\.txt", "re", 10),
    ],
)
def test_glob(fake_files, pattern, mode, count):
    txt = trail.glob("a", pattern, mode=mode)
    assert len(list(txt)) == count


@pytest.mark.parametrize(
    ("pattern", "mode", "count"),
    [
        ("*.png", "glob", 11),
        (r"\d\.\w+", "re", 22),
    ],
)
def test_rglob(fake_files, pattern, mode, count):
    txt = trail.rglob("a", pattern, mode=mode)
    assert len(list(txt)) == count


@pytest.mark.parametrize(
    ("size", "expected_v"),
    [
        pytest.param(22, "22.0 B", id="B"),
        pytest.param(22_222, f"{round(22_222 / 1024, 3)} KB", id="KB"),
        pytest.param(22_222_222, f"{round(22_222_222 / (1024**2), 3)} MB", id="MB"),
        pytest.param(22_222_222_222, f"{round(22_222_222_222 / (1024**3), 3)} GB", id="GB"),
        pytest.param(22_222_222_222_222, f"{round(22_222_222_222_222 / (1024**4), 3)} TB", id="TB"),
    ],
)
def test_strfsize(size, expected_v):
    test_v = trail.strfsize(size=size)
    log.debug(pretty_repr(locals()))
    assert test_v == expected_v


def test_file_checksum():
    test_v = trail.file_checksum(file=path_macro.PROJECT_ROOT / ".gitignore", algorithm="md5")
    assert test_v == "5f1af9b17396ae20f8fb3991476079ec"
