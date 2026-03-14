# toolbox

[![CI](https://github.com/Glatzel/toolbox/actions/workflows/ci.yml/badge.svg)](https://github.com/Glatzel/toolbox/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/Glatzel/toolbox/graph/badge.svg?token=biKBmTxt99)](https://codecov.io/gh/Glatzel/toolbox)

A collection of low-level tools and foundational libraries.

This repository serves as a workspace for building small, focused components that solve practical problems in systems development, data processing, and developer tooling. The projects contained here emphasize clarity of design, composability, and minimal abstraction overhead.

The code in this repository is intended to act as **building blocks** rather than full applications. Each component is designed to be reusable and independently useful, while also fitting into larger systems when combined.

## Scope

The repository contains utilities and libraries for areas such as:

- structured data processing
- diagnostics and error reporting
- rendering and formatting utilities
- stream and protocol handling
- small infrastructure helpers for systems programming

These tools are generally **low-level and infrastructure-oriented**, focusing on primitives that higher-level software can depend on.

## Design Principles

Projects in this repository typically follow a few consistent principles:

### Small and composable

Libraries are designed to solve a narrow problem well rather than becoming large frameworks.

### Explicit over implicit

APIs favor predictable behavior and explicit control over hidden automation.

### Minimal dependencies

External dependencies are kept small whenever possible to reduce build complexity and improve portability.

### Systems-oriented

Performance, memory behavior, and clear control flow are considered important aspects of design.

### Cross-language experimentation

Although many components are written in Rust, the repository may also contain utilities written in other languages where appropriate.

## Organization

The repository is organized as a collection of independent subprojects.
Each directory typically represents a standalone tool or library with its own documentation and build configuration.

## Status

This repository is primarily a **development workspace and experimentation ground**.
Some components may be early-stage, evolving, or subject to significant redesign as ideas are explored.

Stable components will generally have clearer documentation and versioning, while experimental modules may change more rapidly.

## License

Unless otherwise specified, the contents of this repository follow the license included at the root of the project.
