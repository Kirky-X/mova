English | [中文](README_CN.md)

<div align="center">

[![CI](https://img.shields.io/github/actions/workflow/status/Kirky-X/mova/ci.yml?branch=main&label=CI&style=for-the-badge)](https://github.com/Kirky-X/mova/actions/workflows/ci.yml) [![Release](https://img.shields.io/github/v/release/Kirky-X/mova?style=for-the-badge)](https://github.com/Kirky-X/mova/releases) [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg?style=for-the-badge)](LICENSE) [![Rust 1.97.1+](https://img.shields.io/badge/Rust-1.97.1%2B-edded?logo=rust&style=for-the-badge)](https://www.rust-lang.org/) [![Coverage](https://img.shields.io/codecov/c/github/Kirky-X/mova?style=for-the-badge)](https://codecov.io/gh/Kirky-X/mova)

**A fast, reproducible software lifecycle manager**

[✨ Features](#-features) • [🚀 Quick Start](#-quick-start) • [📚 Documentation](#-documentation)

</div>

---

<div align="center">

### 🎯 One Manager for the Whole Software Lifecycle

Source acquisition, parameterized builds, dependency graphs, environment activation, binary management, and benchmark reports — all driven by structured data and guarded policies:

<table style="width:100%; border-collapse: collapse">
<tr>
<td align="center" width="25%">📦<br><b>Package Catalog</b><br><span style="color:#64748B">software · versions · tags</span></td>
<td align="center" width="25%">🔧<br><b>Parameterized Build</b><br><span style="color:#64748B">capability · surface · profile</span></td>
<td align="center" width="25%">🔗<br><b>Dependency Graph</b><br><span style="color:#64748B">required · optional · capabilities</span></td>
<td align="center" width="25%">📊<br><b>Benchmark Report</b><br><span style="color:#64748B">Markdown · Typst · PDF</span></td>
</tr>
</table>

</div>

---

## 📋 Table of Contents

<details open>

- [✨ Features](#-features)
- [🚀 Quick Start](#-quick-start)
  - [📦 Install](#-install)
  - [💡 Basic Usage](#-basic-usage)
- [📚 Documentation](#-documentation)
- [🔗 Further Reading](#-further-reading)
- [📄 License](#-license)
- [🙏 Acknowledgments](#-acknowledgments)
- [📞 Contact & Support](#-contact--support)
- [⭐ Star History](#-star-history)

</details>

---

## ✨ Features

mova manages the lifecycle of source-built software: from source and dependencies to compiled artifacts, environments, and benchmark evidence.

| Capability                 | Notes                                                                                                                    |
| -------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| **Software catalog**       | Software, versions, aliases, tags, capabilities, and signed snapshots                                                    |
| **Dependency graph**       | Capability constraints plus optional exact versions; required, optional, conflict, replace, and extend edges             |
| **Parameterized builds**   | `surface + capability + rule` model covers C/C++, Fortran, Rust, Go, Java, Python, linkers, and build systems            |
| **Security by default**    | `PolicyGuard`, argv-only execution, archive limits, path containment, read-only encrypted catalog, no commands in SQLite |
| **Environment management** | No module CLI and no modulefile generation; binary discovery plus structured `EnvDelta`                                  |
| **Offline support**        | `--package-dir`, signed bundles, local cache, and offline-first queries                                                  |
| **Benchmark workflow**     | Multiple tools per test domain, one combined report, deterministic statistics                                            |
| **Report pipeline**        | Markdown is canonical; PDF is derived through embedded Typst                                                             |
| **Cross-platform**         | Single crate, multiple modules, feature isolation, and `cfg`-gated platform implementations                              |

Version `0.0.0` is the bootstrap release. It contains the package metadata, quality harness, CLI skeleton, and documentation. Lifecycle commands are planned for later versions.

---

## 🚀 Quick Start

### 📦 Install

```bash
cargo install mova --version 0.0.0
```

Requires Rust 1.97.1 or newer.

From source:

```bash
git clone https://github.com/Kirky-X/mova.git
cd mova
cargo build --release
```

### 💡 Basic Usage

```bash
mova --version
mova --help
```

The lifecycle CLI will be added after the bootstrap release.

---

## 📚 Documentation

| Document                          | Notes         |
| --------------------------------- | ------------- |
| [📋 Changelog](docs/CHANGELOG.md) | Release notes |

---

## 🔗 Further Reading

- Version 0.0.0 is the bootstrap release.
- Lifecycle commands are planned for later versions.

---

## 📄 License

Released under the [MIT](LICENSE) license.

Copyright (c) 2026 Kirky.X🌠

---

## 🙏 Acknowledgments

Thanks to the Rust ecosystem and the tools that make reproducible, secure lifecycle management possible.

---

## 📞 Contact & Support

- Bugs and requests: [GitHub Issues](https://github.com/Kirky-X/mova/issues)
- Maintainer: Kirky.X

---

## ⭐ Star History

[![Star History Chart](https://api.star-history.com/svg?repos=Kirky-X/mova&type=Date)](https://star-history.com/#Kirky-X/mova&Date)

### 💝 Support the Project

If you find this project useful, please consider giving it a ⭐️!
