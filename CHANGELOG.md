# Changelog

## [0.6.3] - 2025-08-19

### Documentation

- 📝 doc(changelog): add v0.6.2 changes

### Features

- ✨ feature: add `%`

### Fixes

- 🐛 fix(ci): use updated woodpecker syntax

### Miscellaneous

- 🔨 chore: matches
- 📦 deps: update crates, bump version

## [0.6.2] - 2025-06-06

### Deployment

- 🚀 deploy: fix deploy config and update trunk
- 🚀 deploy: oops

### Fixes

- 🐛 fix(repl): change autoparen behaviour

### Miscellaneous

- 📦 deps: bump
- 🔨 chore: bump version

## [0.6.1] - 2024-08-01

### Features

- ✨ feature: special vars (codeberg #10)
- ✨ feature: add `len` function

### Fixes

- 🐛 fix: downgrade wasm-bindgen to supported trunk ver
- 🐛 fix(web): update website source code url
- 🐛 fix: make parse-num accept numbers

### Miscellaneous

- 🔨 chore: add changelog
- 🔨 chore(readme): link to changelog
- 🔨 chore: wording
- 🔨 chore: simpler sample
- 🔨 chore(readme): add missing flag
- 📦 deps: use pomprt fork, bump deps
- 📦 deps: bump pomprt

### Styling

- 🎨 style(cli): pretty print ast

### Testing

- ✅ test: upload test coverage to codecov
- ✅ test: nvm actually codecov is bloat anyway

### Chore

- Rename `samples` to `examples`, bump version

## [0.6.0] - 2023-11-08

### Refactor

- ♻️ refactor: node `Span`s with more context

## [0.5.2] - 2023-11-08

### Features

- ✨ feature: add faye's logo
- ✨ feat(repl): completion, multiline and auto parens bloat

### Fixes

- 🐛 fix(lexer): skip over whitespace properly and fix highlighter
- 🐛 fix: ignore commas, treat comments as a seperator

### Miscellaneous

- 🔨 chore(readme): link to docs
- 🔨 chore: update logo
- 🔨 chore: clarify licenses
- 🔨 chore: export LexerErrorKind in prelude and Separator
- 🔨 chore(website): make logo open graph image
- 🔨 chore: bump version `0.5.2`

### Refactor

- ♻️ refactor: switch to `pomprt` (codeberg #8)
- ♻️ refactor: add extreme lsp bloat
- ♻️ refactor: lsp unbloat
- ♻️ refactor: make BuiltinFn take a static fn pointer

### Styling

- 🎨 style(repl): revert lambda prompt

## [0.5.1] - 2023-09-27

### Features

- ✨ feat(web): implement input highlighting and improve history (#7)

### Fixes

- 🐛 fix(web): linebreak oopsie
- 🐛 fix: cli repl errors

### Miscellaneous

- 🔨 chore(web): update website source link
- 🔨 chore: update repl scrot
- 🔨 chore: link correct readme to workspace members
- 🔨 chore: revert linguist language
- 🔨 chore(readme): a little more details

### Refactor

- ♻️ refactor: move website to main repo
- ♻️ refactor: workspace
- ♻️ refactor: make `fn` and lambda args a vector

## [0.5.0] - 2023-09-24

### Refactor

- ♻️ refactor: move re-exports to `prelude`

## [0.4.2] - 2023-09-24

### Features

- ✨ feature: add `Vector`, `Char` and related functions

### Refactor

- ♻️ refactor: optimise highlighter with `get_unparsed`

### Styling

- 🎨 style(highlighter): refresh colors

## [0.4.1] - 2023-09-23

### Features

- ✨ feature(lexer): get unparsed method

## [0.4.0] - 2023-09-22

### Features

- ✨ feature: `let` and `const` bindings and `lambda` closures (codeberg #6)

## [0.3.3] - 2023-09-22

### Fixes

- 🐛 fix: make println return String when not a terminal

### Miscellaneous

- 🔨 chore: bump version

## [0.3.2] - 2023-09-20

### Refactor

- ♻️ refactor: debloat exports

## [0.3.1] - 2023-09-20

### Refactor

- ♻️ refactor: make cli deps optional for usage as a lib

## [0.3.0] - 2023-09-15

### Refactor

- ♻️ refactor: restructure everything and add "documentations"

## [0.2.3] - 2023-09-12

### Fixes

- 🐛 fix: highlight tokens at their actual start

### Miscellaneous

- 🔨 chore(readme): update scrot

### Refactor

- ♻️ refactor: move things around

## [0.2.2] - 2023-09-11

### Features

- ✨ feature: add compare functions

## [0.2.1] - 2023-09-11

### Features

- ✨ feature: if/and/or conditionals
- ✨ feature: add full syntax highlighting to errors
- ✨ feature: rustyline bloat
- ✨ feature: extreme rustyline bloat

### Fixes

- 🐛 fix: reset after highlighting
- 🐛 fix: handle multiline highlighting properly

### Miscellaneous

- 🔨 chore: silly
- 🔨 chore: add factorial example

### Refactor

- ♻️ refactor: avoid cloning on `ctx.get_n`
- ♻️ refactor: faye is a lib

### Styling

- 🎨 style: highlight comments in errors

## [0.2.0] - 2023-09-09

### Miscellaneous

- 🔨 chore: add header to examples and fix linguist detection
- 🔨 chore: bump version and add sof

### Refactor

- ♻️ refactor: a much improved eval (codeberg #5)

## [0.1.3] - 2023-09-05

### Fixes

- 🐛 fix: string newlines

### Miscellaneous

- 🔨 chore(test): remove ugly newlines because it won't error
- 🔨 chore: bump version

### Refactor

- ♻️ refactor: sane string parsing (codeberg #4)

## [0.1.2] - 2023-09-04

### Features

- ✨ feature: eval multiple expressions
- ✨ feature: cli eval bloat
- ✨ feature: add comments, evaluate files

### Refactor

- ♻️ refactor: err macro

## [0.1.1] - 2023-09-01

### Features

- ✨ feature: parse multiple expressions

### Fixes

- 🐛 fix: dumass
- 🐛 fix: negative numbers not being real

### Miscellaneous

- 🔨 chore(readme): scrot
- 🔨 chore(readme): guh

### Refactor

- ♻️ refactor: repl error macro
- ♻️ refactor: improve error locations (codeberg #1)
- ♻️ refactor: unrecursive `parse` fn (codeberg #2)

## [0.1.0] - 2023-08-29

### Features

- ✨ feature: error locations
- ✨ feature: fancy repl error

### Fixes

- 🐛 fix(eval): stupid error location bug
- 🐛 fix: janky prompt, incorrect eval error location
- 🐛 fix: accurate line and column

### Miscellaneous

- 🔨 chore: mew
- 🔨 chore: add the repo thingys
- 🔨 chore(justfile): :333
- 🔨 chore(justfile): test before pushing

### Refactor

- ♻️ refactor: symbols are not just chars, better tokenization

### Testing

- ✅ test: add more tests

## [0.0.1] - 2023-08-27

