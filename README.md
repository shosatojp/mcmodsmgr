# Minecraft Mods Manager

[![Rust](https://github.com/shosatojp/mcmodsmgr/actions/workflows/rust.yml/badge.svg)](https://github.com/shosatojp/mcmodsmgr/actions/workflows/rust.yml)

## Install

```sh
cargo install mcmodsmgr
```

> `$HOME/.cargo/bin` should be in your `$PATH`

## Usage

- Search

```sh
mcmodsmgr search jei
```

- Mod Information

```sh
mcmodsmgr describe jei
```

- Install

```sh
# filter by version
mcmodsmgr -v 1.12.2 install jei

# filter by version and modloader
mcmodsmgr -v 1.12.2 -l forge install jei
```
