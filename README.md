<div align="center">

# 🗑️ unwhere

**Universal package uninstaller for Linux**

Searches and removes packages across **dnf**, **flatpak** and **pacman** from a single command line.

[![Rust](https://img.shields.io/badge/Rust-2024-orange?logo=rust)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-green)](#license)
[![Build](https://img.shields.io/github/actions/workflow/status/endersoul/unwhere/rust.yml?branch=master)](https://github.com/endersoul/unwhere/actions)

<p align="right">
  <a href="README.es.md">🇪🇸 Español</a>
</p>

</div>

---

## 📋 Table of Contents

- [Description](#description)
- [Features](#features)
- [Supported Package Managers](#supported-package-managers)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [How It Works](#how-it-works)
- [Building from Source](#building-from-source)
- [About This Project](#about-this-project)
- [License](#license)

---

## Description

`unwhere` is a command-line tool written in Rust that simplifies package uninstallation on Linux distributions. Instead of remembering which package manager installed each application, `unwhere` automatically searches across all installed managers and lets you remove what you need with a single command.

---

## Features

- 🔍 **Universal Search** — Searches packages across multiple managers simultaneously
- 🎯 **Smart Search** — Uses regex with case-insensitive matching
- ⚡ **Direct Removal** — If only one match is found, it is removed automatically
- 🤔 **Interactive Selection** — If multiple matches are found, it shows options to choose from
- 🔒 **Permission Handling** — Runs commands with `sudo` when required (dnf, pacman)
- 🦀 **Performance** — Written in Rust for speed and reliability

---

## Supported Package Managers

| Manager     | List Command                               | Delete Command      | Requires sudo? |
| ----------- | ------------------------------------------ | ------------------- | -------------- |
| **dnf**     | `dnf list --installed`                     | `dnf rm`            | ✅ Yes          |
| **flatpak** | `flatpak list --app --columns=application` | `flatpak uninstall` | ❌ No           |
| **pacman**  | `pacman -Q`                                | `pacman -R`         | ✅ Yes          |

---

## Installation

### From GitHub Releases

```bash
# Download the latest version (replace the URL with the correct version)
wget https://github.com/endersoul/unwhere/releases/latest/download/unwhere -O /usr/local/bin/unwhere

# Give execute permissions
chmod +x /usr/local/bin/unwhere
```

### Build from Source

```bash
# Clone the repository
git clone https://github.com/endersoul/unwhere.git
cd unwhere

# Build in release mode
cargo build --release

# The binary will be at target/release/unwhere
# Copy it to a location in your PATH
cp target/release/unwhere /usr/local/bin/
```

---

## Usage

```bash
unwhere <package_name>
```

### Basic Flow

1. You run `unwhere` with the name (or partial name) of a package
2. `unwhere` searches across **dnf**, **flatpak** and **pacman**
3. If **1 result** is found → it is removed automatically
4. If **multiple results** are found → a list is shown and you are asked to choose
5. If **nothing is found** → displays "no match found"

---

## Examples

### Remove a specific package

```bash
$ unwhere firefox

# If "firefox" only appears in one manager, it is removed directly
```

### Search with partial name

```bash
$ unwhere code

# Could find: vscode, codeblocks, code-server, etc.
# Would show something like:
# (0) vscode flatpak
# (1) codeblocks dnf
# choose the option
# 1
```

### Search with regular expression

```bash
# Find all packages containing "lib"
$ unwhere lib

# Find packages starting with "gnome-"
$ unwhere ^gnome-
```

---

## How It Works

```
┌─────────────────────────────────────────┐
│              unwhere                     │
├─────────────────────────────────────────┤
│  1. Receives the package name           │
│  2. Compiles a regex (case-insensitive) │
│  3. Iterates over each installed manager│
│     ┌───────┐ ┌─────────┐ ┌─────────┐  │
│     │  dnf  │ │ flatpak │ │ pacman  │  │
│     └───┬───┘ └────┬────┘ └────┬────┘  │
│         │          │           │        │
│         ▼          ▼           ▼        │
│  4. Lists packages from each manager    │
│  5. Filters by regex                    │
│  6. If 1 match → removes               │
│  7. If N matches → shows options        │
│  8. If 0 matches → "no match found"     │
└─────────────────────────────────────────┘
```

---

## Building from Source

### Requirements

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024 or later)
- Cargo (included with Rust)

### Dependencies

| Crate   | Version | Usage                     |
| ------- | ------- | ------------------------- |
| `regex` | 1.13.1  | Package search with regex |

### Build

```bash
# Debug mode
cargo build

# Release mode (recommended for daily use)
cargo build --release
```

---

## About This Project

> This is **my first project** written in Rust. As such, it likely has bugs, areas for improvement, and code that is not 100% idiomatic. Any contribution, issue, or feedback is welcome and will help me improve both the project and my skills as a developer.

### Known Limitations

- Search depends on the output format of each package manager; if a manager changes its format, it may stop working correctly
- Package versions of found packages are not displayed
- The list of supported managers is hardcoded (although adding new ones is straightforward)

### Future Roadmap

- [ ] Add more package managers (apt, snap, brew...)

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.

---

<div align="center>

Made with 🦀 Rust

</div>
