# toolbox

[![codecov](https://codecov.io/gh/Glatzel/toolbox/graph/badge.svg?token=biKBmTxt99)](https://codecov.io/gh/Glatzel/toolbox)

**toolbox** is a curated collection of utility libraries for Python and Rust, designed to support common development tasks across multiple projects.  
It provides reusable components for logging, concurrency, decorators, path management, error handling, text parsing, and other general-purpose utilities.

This repository is intended as a personal utility suite rather than a single-purpose library.

---

## Components

### Python

- **clerk-logtool** — Advanced logging utilities for consistent message handling.  
- **cycler-itertool** — Extended iterator and sequence manipulation tools.  
- **linker-import** — Dynamic module and import management helpers.  
- **magic-decorator** — General-purpose function and class decorators.  
- **rayon-multiprocess/thread** — Simplified multiprocessing and threading utilities.  
- **trail-path** — Path and filesystem management helpers.  

### rust

- **clerk** — Logging wrapper built on `tracing` with additional helper functions.  
- **envoy** — C string conversion and related utilities.  
- **mischief** — Error handling and reporting utilities.  
- **rax** — Nom-inspired parser for text and structured data.  

Each Rust crate is independent and can be used separately in other projects.
