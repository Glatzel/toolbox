# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.34] - 2026-01-20

### Changed

- Release conda package in demon-forge by @Glatzel in [#314](https://github.com/Glatzel/toolbox/pull/314)

### Refactor

- C string handing in envoy by @Glatzel in [#319](https://github.com/Glatzel/toolbox/pull/319)

## [0.0.33] - 2025-12-27

### Fixed

- Release ci by @Glatzel in [#310](https://github.com/Glatzel/toolbox/pull/310)

### Revert

- Add vscode settings by @Glatzel in [#305](https://github.com/Glatzel/toolbox/pull/305)

## [0.0.32] - 2025-10-15

### Added

- Add compile error on both defmt and tracing enabled by @Glatzel in [#254](https://github.com/Glatzel/toolbox/pull/254)
- Add trait to derive by @Glatzel in [#263](https://github.com/Glatzel/toolbox/pull/263)

### Fixed

- Changelog ci by @Glatzel in [#269](https://github.com/Glatzel/toolbox/pull/269)

### Removed

- Remove useless ref param by @Glatzel in [#264](https://github.com/Glatzel/toolbox/pull/264)

## [0.0.31] - 2025-10-05

### Added

- Add readme by @Glatzel in [#239](https://github.com/Glatzel/toolbox/pull/239)
- Add defmt log by @Glatzel in [#246](https://github.com/Glatzel/toolbox/pull/246)

### Removed

- Remove useless features by @Glatzel in [#244](https://github.com/Glatzel/toolbox/pull/244)

## [0.0.30] - 2025-09-10

### Changed

- Impl display for report by @Glatzel in [#232](https://github.com/Glatzel/toolbox/pull/232)

## [0.0.29] - 2025-09-09

### Changed

- New terminal error render for mischief by @Glatzel in [#221](https://github.com/Glatzel/toolbox/pull/221)
- Use standard error for library by @Glatzel in [#224](https://github.com/Glatzel/toolbox/pull/224)
- Use hashbrown instead of std hashmap by @Glatzel in [#225](https://github.com/Glatzel/toolbox/pull/225)

### Documentation

- Add docs for rust crates by @Glatzel in [#226](https://github.com/Glatzel/toolbox/pull/226)

### Fixed

- No std rax by @Glatzel in [#223](https://github.com/Glatzel/toolbox/pull/223)

## [0.0.28] - 2025-09-06

### Added

- Add other fields to mischief protocol like miette by @Glatzel in [#213](https://github.com/Glatzel/toolbox/pull/213)

### Changed

- Use mischief instead of miette by @Glatzel in [#214](https://github.com/Glatzel/toolbox/pull/214)

### Fixed

- Mischief long line length by @Glatzel in [#216](https://github.com/Glatzel/toolbox/pull/216)

## [0.0.27] - 2025-09-04

### Performance

- Use vec to store error message by @Glatzel in [#205](https://github.com/Glatzel/toolbox/pull/205)

### Refactor

- New framework for mischief by @Glatzel in [#209](https://github.com/Glatzel/toolbox/pull/209)

### Removed

- Remove unused features by @Glatzel in [#211](https://github.com/Glatzel/toolbox/pull/211)

### Testing

- Add mischief test by @Glatzel in [#202](https://github.com/Glatzel/toolbox/pull/202)

## [0.0.26] - 2025-08-30

### Changed

- Mischief by @Glatzel in [#200](https://github.com/Glatzel/toolbox/pull/200)

### Refactor

- Clerk by @Glatzel in [#198](https://github.com/Glatzel/toolbox/pull/198)

## [0.0.25] - 2025-07-26

### Added

- Add arm64 test by @Glatzel in [#169](https://github.com/Glatzel/toolbox/pull/169)
- Add rax by @Glatzel in [#171](https://github.com/Glatzel/toolbox/pull/171)

## [0.0.24] - 2025-07-20

### Changed

- Use cchar by @Glatzel in [#167](https://github.com/Glatzel/toolbox/pull/167)

## [0.0.23] - 2025-07-18

### Added

- Add serde for loglevel by @Glatzel in [#162](https://github.com/Glatzel/toolbox/pull/162)

### Removed

- Remove re-export by @Glatzel in [#154](https://github.com/Glatzel/toolbox/pull/154)

### Revert

- "feat: remove re-export" by @Glatzel in [#156](https://github.com/Glatzel/toolbox/pull/156)

## [0.0.22] - 2025-07-09

### Fixed

- Add null fn by @Glatzel in [#152](https://github.com/Glatzel/toolbox/pull/152)

## [0.0.21] - 2025-07-03

### Added

- Add fast init log by @Glatzel in [#147](https://github.com/Glatzel/toolbox/pull/147)

## [0.0.20] - 2025-06-06

### Fixed

- Envoy by @Glatzel in [#134](https://github.com/Glatzel/toolbox/pull/134)

## [0.0.19] - 2025-06-05

### Fixed

- ToVecCStr by @Glatzel in [#129](https://github.com/Glatzel/toolbox/pull/129)

## [0.0.18] - 2025-06-05

### Fixed

- Add test back by @Glatzel in [#123](https://github.com/Glatzel/toolbox/pull/123)
- Correct name by @Glatzel in [#126](https://github.com/Glatzel/toolbox/pull/126)

## [0.0.17] - 2025-06-04

### Changed

- New tools by @Glatzel in [#119](https://github.com/Glatzel/toolbox/pull/119)

## [0.0.16] - 2025-06-03

### Added

- Add support for *const *const i8 by @Glatzel in [#116](https://github.com/Glatzel/toolbox/pull/116)

## [0.0.15] - 2025-06-01

### Fixed

- CStrToString for *mut i8 and [i8] by @Glatzel in [#110](https://github.com/Glatzel/toolbox/pull/110)

## [0.0.14] - 2025-06-01

### Added

- Add more cstr tools by @Glatzel in [#108](https://github.com/Glatzel/toolbox/pull/108)

## [0.0.13] - 2025-05-31

### Changed

- Impl PtrToString for *mut i8 by @Glatzel in [#104](https://github.com/Glatzel/toolbox/pull/104)

## [0.0.12] - 2025-05-31

### Fixed

- Pre commit by @Glatzel in [#100](https://github.com/Glatzel/toolbox/pull/100)

## [0.0.10] - 2025-05-12

### Fixed

- **(rust)** Null macro by @Glatzel in [#74](https://github.com/Glatzel/toolbox/pull/74)

## [0.0.9] - 2025-04-21

### Added

- Initial renovate bot by @Glatzel in [#39](https://github.com/Glatzel/toolbox/pull/39)

## [0.0.8] - 2025-04-11

### Added

- Add level filter back by @Glatzel in [#34](https://github.com/Glatzel/toolbox/pull/34)

## [0.0.7] - 2025-04-11

### Added

- Add color option to terminal layer by @Glatzel in [#31](https://github.com/Glatzel/toolbox/pull/31)

## [0.0.6] - 2025-04-11

### Removed

- Remove log level setting in layer by @Glatzel in [#29](https://github.com/Glatzel/toolbox/pull/29)

## [0.0.5] - 2025-03-26

### Changed

- Python by @Glatzel in [#19](https://github.com/Glatzel/toolbox/pull/19)

## [0.0.4] - 2025-03-09

### Added

- Add macro to wrap tracing macro by @Glatzel in [#16](https://github.com/Glatzel/toolbox/pull/16)

## [0.0.2] - 2025-02-14

### Added

- Add changelog by @Glatzel in [#8](https://github.com/Glatzel/toolbox/pull/8)
- Add vscode setting and markdown lint config by @Glatzel in [#9](https://github.com/Glatzel/toolbox/pull/9)

### Changed

- File layer by @Glatzel in [#7](https://github.com/Glatzel/toolbox/pull/7)
- Deal with some writting cases in file layer by @Glatzel in [#10](https://github.com/Glatzel/toolbox/pull/10)
- Simplify terminal layer fn by @Glatzel in [#11](https://github.com/Glatzel/toolbox/pull/11)

### Documentation

- Add sample to `terminal_layer` by @Glatzel in [#2](https://github.com/Glatzel/toolbox/pull/2)

### Performance

- Use static styled string by @Glatzel in [#6](https://github.com/Glatzel/toolbox/pull/6)

## [0.0.1] - 2025-02-13

### Added

- Add terminal layer by @Glatzel in [#1](https://github.com/Glatzel/toolbox/pull/1)

[0.0.34]: https://github.com/Glatzel/toolbox/compare/v0.0.33..v0.0.34
[0.0.33]: https://github.com/Glatzel/toolbox/compare/v0.0.32..v0.0.33
[0.0.32]: https://github.com/Glatzel/toolbox/compare/v0.0.31..v0.0.32
[0.0.31]: https://github.com/Glatzel/toolbox/compare/v0.0.30..v0.0.31
[0.0.30]: https://github.com/Glatzel/toolbox/compare/v0.0.29..v0.0.30
[0.0.29]: https://github.com/Glatzel/toolbox/compare/v0.0.28..v0.0.29
[0.0.28]: https://github.com/Glatzel/toolbox/compare/v0.0.27..v0.0.28
[0.0.27]: https://github.com/Glatzel/toolbox/compare/v0.0.26..v0.0.27
[0.0.26]: https://github.com/Glatzel/toolbox/compare/v0.0.25..v0.0.26
[0.0.25]: https://github.com/Glatzel/toolbox/compare/v0.0.24..v0.0.25
[0.0.24]: https://github.com/Glatzel/toolbox/compare/v0.0.23..v0.0.24
[0.0.23]: https://github.com/Glatzel/toolbox/compare/v0.0.22..v0.0.23
[0.0.22]: https://github.com/Glatzel/toolbox/compare/v0.0.21..v0.0.22
[0.0.21]: https://github.com/Glatzel/toolbox/compare/v0.0.20..v0.0.21
[0.0.20]: https://github.com/Glatzel/toolbox/compare/v0.0.19..v0.0.20
[0.0.19]: https://github.com/Glatzel/toolbox/compare/v0.0.18..v0.0.19
[0.0.18]: https://github.com/Glatzel/toolbox/compare/v0.0.17..v0.0.18
[0.0.17]: https://github.com/Glatzel/toolbox/compare/v0.0.16..v0.0.17
[0.0.16]: https://github.com/Glatzel/toolbox/compare/v0.0.15..v0.0.16
[0.0.15]: https://github.com/Glatzel/toolbox/compare/v0.0.14..v0.0.15
[0.0.14]: https://github.com/Glatzel/toolbox/compare/v0.0.13..v0.0.14
[0.0.13]: https://github.com/Glatzel/toolbox/compare/v0.0.12..v0.0.13
[0.0.12]: https://github.com/Glatzel/toolbox/compare/v0.0.11..v0.0.12
[0.0.10]: https://github.com/Glatzel/toolbox/compare/v0.0.9..v0.0.10
[0.0.9]: https://github.com/Glatzel/toolbox/compare/v0.0.8..v0.0.9
[0.0.8]: https://github.com/Glatzel/toolbox/compare/v0.0.7..v0.0.8
[0.0.7]: https://github.com/Glatzel/toolbox/compare/v0.0.6..v0.0.7
[0.0.6]: https://github.com/Glatzel/toolbox/compare/v0.0.5..v0.0.6
[0.0.5]: https://github.com/Glatzel/toolbox/compare/v0.0.4..v0.0.5
[0.0.4]: https://github.com/Glatzel/toolbox/compare/v0.0.3..v0.0.4
[0.0.2]: https://github.com/Glatzel/toolbox/compare/v0.0.1..v0.0.2

<!-- generated by git-cliff -->
