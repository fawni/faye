<div align="center">
<h1>faye</h1>

**[Website](https://faye.codeberg.page) • [Docs](https://faye.codeberg.page/docs)**

![logo](.meta/logo.png)

faye is the name of coquettish tiny grey cat, the middle name of a pretty ombre girl in Missouri, and this lil lisp 🦋

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
- `-m`, `--matching-brackets`: Highlight matching brackets

`faye -h` for more information.

## License

[Apache-2.0](LICENSE)