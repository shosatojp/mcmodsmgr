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
mcmodsmgr install jei -v 1.12.2

# filter by version and modloader
mcmodsmgr install jei -v 1.12.2 -l forge

# referencing published mods list by server
mcmodsmgr install -R https://some-modded-server.com/.mods-lock.json

# referencing mods list file
mcmodsmgr install -R other-env/.mods-lock.json
```
