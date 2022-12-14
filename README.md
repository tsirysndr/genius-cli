# Genius CLI

<p>
  <a href="https://crates.io/crates/genius-cli" target="_blank">
    <img src="https://img.shields.io/crates/v/genius-cli.svg" />
  </a>
   <a href="https://crates.io/crates/genius-cli" target="_blank">
    <img src="https://img.shields.io/crates/dr/genius-cli" />
  </a>
  <a href="https://docs.rs/genius-cli" target="_blank">
    <img src="https://docs.rs/genius-cli/badge.svg" />
  </a>
  <a href="LICENSE" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-blue.svg" />
  </a>
   <a href="https://github.com/tsirysndr/genius-cli/actions/workflows/release.yml" target="_blank">
    <img alt="License: MIT" src="https://github.com/tsirysndr/genius-cli/actions/workflows/release.yml/badge.svg" />
  </a>
  <a href="https://github.com/tsirysndr/genius-cli/actions/workflows/rust-clippy.yml" target="_blank">
    <img alt="release" src="https://github.com/tsirysndr/genius-cli/actions/workflows/rust-clippy.yml/badge.svg?branch=master" />
  </a>
</p>

<p>
<a href="https://www.buymeacoffee.com/tsiry">
  <img src="https://cdn.buymeacoffee.com/buttons/v2/default-red.png" alt="Buy Me A Coffee" height="40" />
</a>
</p>

Genius CLI helps you search for lyrics or song informations from [Genius](https://genius.com), right from your terminal.

<img width="800" src="./preview.svg">

## Installation

```bash
cargo install genius-cli
```

### macOS/Linux
```bash
brew install tsirysndr/tap/genius
```

## Usage

```
USAGE:
    genius <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    help      Print this message or the help of the given subcommand(s)
    lyrics    Get the lyrics of a song
    search    Search for a song
    song      Get song informations
```
