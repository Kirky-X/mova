[English](README.md) | 中文

<div align="center">

[![CI](https://img.shields.io/github/actions/workflow/status/Kirky-X/mova/ci.yml?branch=main&label=CI&style=for-the-badge)](https://github.com/Kirky-X/mova/actions/workflows/ci.yml) [![Release](https://img.shields.io/github/v/release/Kirky-X/mova?style=for-the-badge)](https://github.com/Kirky-X/mova/releases) [![License: Apache-2.0 + Commons Clause](https://img.shields.io/badge/License-Apache--2.0%20%2B%20Commons%20Clause-orange.svg?style=for-the-badge)](LICENSE) [![Rust 1.97.1+](https://img.shields.io/badge/Rust-1.97.1%2B-edded?logo=rust&style=for-the-badge)](https://www.rust-lang.org/) [![Coverage](https://img.shields.io/codecov/c/github/Kirky-X/mova?style=for-the-badge)](https://codecov.io/gh/Kirky-X/mova)

**快速、可靠、可复现的软件生命周期管理器**

[✨ 功能特性](#-功能特性) • [🚀 快速开始](#-快速开始) • [📚 文档](#-文档)

</div>

---

<div align="center">

### 🎯 一个管理器覆盖软件全生命周期

源码获取、参数化编译、依赖网络、环境加载、二进制管理与 benchmark 报告统一由结构化数据和安全策略驱动：

<table style="width:100%; border-collapse: collapse">
<tr>
<td align="center" width="25%">📦<br><b>软件目录</b><br><span style="color:#64748B">software · versions · tags</span></td>
<td align="center" width="25%">🔧<br><b>参数化构建</b><br><span style="color:#64748B">capability · surface · profile</span></td>
<td align="center" width="25%">🔗<br><b>依赖网络</b><br><span style="color:#64748B">required · optional · capabilities</span></td>
<td align="center" width="25%">📊<br><b>基准报告</b><br><span style="color:#64748B">Markdown · Typst · PDF</span></td>
</tr>
</table>

</div>

---

## 📋 目录

<details open>

- [✨ 功能特性](#-功能特性)
- [🚀 快速开始](#-快速开始)
  - [📦 安装](#-安装)
  - [💡 基本用法](#-基本用法)
- [📚 文档](#-文档)
- [🔗 延伸阅读](#-延伸阅读)
- [📄 许可证](#-许可证)
- [🙏 致谢](#-致谢)
- [📞 联系与支持](#-联系与支持)
- [⭐ Star 历史](#-star-历史)

</details>

---

## ✨ 功能特性

mova 管理源码构建软件的完整生命周期：从源码和依赖，到编译制品、运行环境与 benchmark 证据。

| 能力               | 说明                                                                                            |
| ------------------ | ----------------------------------------------------------------------------------------------- |
| **软件目录**       | 软件、版本、别名、tag、能力标记与签名快照                                                       |
| **依赖网络**       | 能力约束＋可选精确版本；required、optional、conflict、replace 与 extend 边                      |
| **参数化构建**     | `surface + capability + rule` 模型覆盖 C/C++、Fortran、Rust、Go、Java、Python、链接器与构建系统 |
| **默认安全**       | `PolicyGuard`、argv 执行、归档限额、路径约束、只读加密目录、SQLite 不存命令                     |
| **环境管理**       | 不依赖 module CLI，不生成 modulefile；基于二进制发现与结构化 `EnvDelta`                         |
| **离线支持**       | `--package-dir`、签名离线包、本地缓存与离线优先查询                                             |
| **benchmark 流程** | 每个测试域可选多个工具，输出一份组合报告，统计量确定性计算                                      |
| **报告流水线**     | Markdown 为主产物，内嵌 Typst 派生 PDF                                                          |
| **跨平台**         | 单 crate、多 mod、feature 隔离与 `cfg` 平台实现                                                 |

`0.0.0` 是引导版本，包含包元数据、质量门禁、CLI 骨架与文档。生命周期命令将在后续版本加入。

---

## 🚀 快速开始

### 📦 安装

```bash
cargo install mova --version 0.0.0
```

需要 Rust 1.97.1 或更新版本。

从源码构建：

```bash
git clone https://github.com/Kirky-X/mova.git
cd mova
cargo build --release
```

### 💡 基本用法

```bash
mova --version
mova --help
```

生命周期 CLI 将在引导版本之后加入。

---

## 📚 文档

| 文档                             | 说明     |
| -------------------------------- | -------- |
| [📋 更新日志](docs/CHANGELOG.md) | 版本说明 |

---

## 🔗 延伸阅读

- `0.0.0` 是引导版本。
- 生命周期命令将在后续版本加入。

---

## 📄 许可证

本项目使用 [Apache License 2.0 with Commons Clause](LICENSE) 许可条件。

商业使用需要获得许可方的单独书面授权。

Copyright (c) 2026 Kirky.X🌠

---

## 🙏 致谢

感谢 Rust 生态，以及让可复现、安全生命周期管理成为可能的工具链。

---

## 📞 联系与支持

- 问题与需求：[GitHub Issues](https://github.com/Kirky-X/mova/issues)
- 维护者：Kirky.X

---

## ⭐ Star 历史

[![Star History Chart](https://api.star-history.com/svg?repos=Kirky-X/mova&type=Date)](https://star-history.com/#Kirky-X/mova&Date)

### 💝 支持项目

如果你觉得这个项目有用，请为它点一个 ⭐️！
