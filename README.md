<div align="center">

# faye

**[Website](https://faye.fawn.moe) • [Documentation](https://faye.codeberg.page/docs) • [Changelog](CHANGELOG.md)** 

![logo](.meta/logo.png)

faye is the name of coquettish tiny grey cat, the middle name of a pretty ombre girl in missouri, and this lil lisp 🦋

[![crates.io](https://img.shields.io/crates/v/faye.svg)](https://crates.io/crates/faye)
[![status-badge](https://ci.codeberg.org/api/badges/12559/status.svg)](https://ci.codeberg.org/repos/12559)

</div>

## Installation

### crates.io

```sh
cargo install faye
```

### Codeberg

```sh
cargo install --git https://codeberg.org/fawn/faye
```

## Usage

![scrot](.meta/repl.png)

Run the repl:

```sh
faye
```

Evaluate an expression:

```sh
faye -e '(* 3 2)'
```

Evaluate a file:

```sh
faye main.fy
```

### Flags

- `-e`, `--eval`: Evaluate a string
- `-l`, `--lex`: Print the lexer output
- `-a`, `--ast`: Print the parser output
- `-m`, `--matching-brackets`: Highlight matching brackets in the repl

`faye -h` for more information.

## License

Source code is licensed under [Apache-2.0](LICENSE). faye's logo by [fawn](https://fawn.moe) and [rini](https://rinici.de) is licensed under [CC-BY-NC-SA-4.0](http://creativecommons.org/licenses/by-nc-sa/4.0/)
