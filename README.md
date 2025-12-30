
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0)

[![Version](https://img.shields.io/badge/version-0.0.0-limegreen?label=Version&color=limegreen)]()

<!-- [![Version](https://img.shields.io/github/v/tag/username/repo?label=Release&color=limegreen)](https://github.com/yourusername/yourrepo/releases) -->

> ## Disclaimer
> *All trademarks, third-party assets/logos, and brand names used in this repository/project are property of their respective owners. 
This project is an independent educational resource and is not sanctioned, sponsored, or managed by any third-party trademark holders.*

## Project/folder layout
```bash
#REM: (beginner)
x00_setup/
|--- .gitignore
|--- README.md
|--- Cargo.toml
|--- rustfmt.toml
\--- src/
     |--- main/rs/x00_setup/
     |    |--- lib.rs
     |    \--- main.rs
     \--- test/rs/x00_setup/
          \--- lib.rs
```

> ### [!NOTE]
> ***crate** or a **cargo package (concrete project)**: can have/contain one (1) or more `bin_crate` and an `integration_test`, however, only one `lib_crate`.*
## Basic Cargo command(s)
### workspace basic cargo command
```bash
workspace_root > cargo test [--release] [-p <member|project_name>] [--all] [--] [--no-capture]
workspace_root > cargo test [--release] [-p <member|project_name>] [--test <integration_test_name>]
workspace_root > cargo test [--release] [-p <member|project_name>] [--bin <bin_crate_name>]
workspace_root > cargo test [--release] [-p <member|project_name>] [--lib]
workspace_root > cargo build [--release] [-p <member|project_name>] [--bin <bin_crate_name>]
workspace_root > cargo build [--release] [-p <member|project_name>] [--lib]
workspace_root > cargo run [--release] [-p <member|project_name>] [--bin <bin_crate_name>]
```
### project basic cargo command
```bash
member|project_name > cargo test [--release] [--all] [--] [--no-capture] 
member|project_name > cargo test [--release] [--test <integration_test_name>]
member|project_name > cargo test [--release] [--bin <bin_crate_name>]
member|project_name > cargo test [--release] [--lib]
member|project_name > cargo build [--release] [--bin <bin_crate_name>]
member|project_name > cargo build [--release] [--lib]
member|project_name > cargo run [--release] [--bin <bin_crate_name>]
```