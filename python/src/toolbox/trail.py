import hashlib
import itertools
import math
import re
from collections.abc import Iterable
from pathlib import Path
from typing import Literal


def glob(dir: str | Path, *patterns: str, mode: Literal["glob", "re"] = "glob") -> Iterable:
    r"""
    Iterate over directory and yield all existing files matching the given relative patterns.

    Parameters
    ----------
    dir
        Directory to search.
    *patterns
        Patterns to match.
    mode
        Chooce pattern style.

    Returns
    -------
    Iterable
        Matched files generator.

    Examples
    --------
    >>> list(pathtool.glob(Path("./"), "*.jpg", "*.png"))
    [WindowsPath("a.jpg"), WindowsPath("b.png")]
    >>> list(pathtool.glob(Path("./"), r"test_\w+.py", mode="re"))
    [WindowsPath("test_a.py"), WindowsPath("test_b.py")]
    """
    dir = Path(dir)
    match mode:
        case "glob":
            generators = [Path.rglob(dir, ext) for ext in patterns]
            return itertools.chain(*generators)
        case "re":
            generators = [f for f in dir.glob("*.*") for p in patterns if re.match(p, f.name)]
            return generators


def rglob(dir: str | Path, *patterns: str, mode: Literal["glob", "re"] = "glob") -> Iterable:
    r"""
    Iterate over directory and subdirctory and yield all existing files matching the given relative patterns.

    Parameters
    ----------
    dir
        Directory to search.
    *patterns
        Patterns to match.
    mode
        Chooce pattern style.

    Returns
    -------
    Iterable
        Matched files generator.

    Examples
    --------
    >>> list(pathtool.rglob(Path("./"), "*.jpg", "*.png"))
    [WindowsPath("a.jpg"), WindowsPath("./sub/b.png")]
    >>> list(pathtool.glob(Path("./"), r"test_\w+.py", mode="re"))
    [WindowsPath("test_a.py"), WindowsPath("./test/test_b.py")]
    """
    dir = Path(dir)
    match mode:
        case "glob":
            generators = [Path.rglob(dir, ext) for ext in patterns]
            return itertools.chain(*generators)
        case "re":
            generators = [f for f in dir.rglob("*.*") for p in patterns if re.match(p, f.name)]
            return generators


def strfsize(size: int, digits: int = 3) -> str:
    r"""
    Format file size string.

    Parameters
    ----------
    size
        File size in byte unit.
    digits
        Precision in decimal digits.

    Returns
    -------
    str
        Formatted file size string.

    Examples
    --------
    >>> pathtool.strfsize(512)
    512.0 B
    >>> pathtool.strfsize(1234)
    1.205 KB
    """
    unit_dict = {0: "B", 1: "KB", 2: "MB", 3: "GB", 4: "TB"}
    bu = 1024
    exp: int = math.floor(math.log(size, bu))
    size_str = f"{round(size / bu**exp,digits)} {unit_dict[exp]}"

    return size_str


def file_checksum(
    file: str | Path,
    algorithm: Literal[
        "blake2b",
        "sm3",
        "sha256",
        "sha3_256",
        "sha512_256",
        "sha384",
        "md5",
        "shake_256",
        "sha512",
        "sha512_224",
        "sha3_384",
        "sha3_512",
        "sha1",
        "md5-sha1",
        "blake2s",
        "ripemd160",
        "sha3_224",
        "shake_128",
        "sha224",
    ] = "sha256",
    buffer_size=8_388_608,
):
    """
    Calculate the checksum of a file using the specified algorithm.

    Parameters
    ----------
    file
        Path to the file to calculate the checksum for.
    algorithm
        Hash algorithm to use. Check ``hashlib.algorithms_available`` for all
        available options.
    buffer_size
        Default 8 MB.

    Returns
    -------
    str
        Hexadecimal checksum of the file.

    Raises
    ------
    AssertionError
        If file is not existed.

    Examples
    --------
    >>> calculate_checksum("example.txt", algorithm="md5")
    'd41d8cd98f00b204e9800998ecf8427e'
    """
    file = Path(file)
    assert file.exists()
    assert file.is_file()
    hash_func = hashlib.new(algorithm)
    with file.open("rb") as f:
        while chunk := f.read(buffer_size):
            hash_func.update(chunk)
    return hash_func.hexdigest()
