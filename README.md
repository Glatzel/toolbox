# toolbox

[![codecov](https://codecov.io/gh/Glatzel/toolbox/graph/badge.svg?token=biKBmTxt99)](https://codecov.io/gh/Glatzel/toolbox)

toolbox is a personal collection of small, simple, but useful utilities I use across most of my projects.
It contains both Python helpers and Rust crates, organized by language and purpose.

This is not a single-purpose library — it’s a lightweight utility belt for everyday development tasks.

---

Components Overview:

Python (py/):

- clerk-logtool — Logging helpers
- cycler-itertool — Itertools and iterator utilities
- linker-import util — Import / module helpers
- magic-decorator — Common decorators
- rayon-multiprocess/thread tool — Multiprocessing and threading utilities
- trail-path util — Path and filesystem helpers

Rust Crates:

- clerk — Log wrapper for tracing and some log tools
- envoy — Environment and config helpers
- cstring — C string conversion utilities
- mischief — Error handling helpers
- rax — Nom-like text parser

Each Rust crate is independent and can be used on its own.
