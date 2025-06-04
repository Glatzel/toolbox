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
    # Create a test file
    file_path = path_macro.TEMP_DIR / "test.txt"
    content = b"hello world"
    file_path.write_bytes(content)

    # Known checksums for "hello world"
    expected_md5 = "5eb63bbbe01eeed093cb22bb8f5acdc3"
    expected_sha256 = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9"

    # Test md5
    result_md5 = trail.file_checksum(file_path, algorithm="md5")
    assert result_md5 == expected_md5

    # Test sha256
    result_sha256 = trail.file_checksum(file_path, algorithm="sha256")
    assert result_sha256 == expected_sha256

    # Test file not exists
    with pytest.raises(AssertionError):
        trail.file_checksum(path_macro.TEMP_DIR / "not_exists.txt")
