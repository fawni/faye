set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

test:
    cargo test --all-features

lint:
    cargo clippy -- -W clippy::pedantic -W clippy::nursery

repl:
    @cargo run -q

push:
    git push
    git push gh