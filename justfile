set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

test:
    cargo test --all-features

lint:
    cargo clippy -- -W clippy::pedantic -W clippy::nursery

repl:
    @cargo run -q

install:
    @cargo install --path .

uninstall:
    @cargo uninstall faye

pull:
    git pull
    git pull gh master

push: (test)
    git push
    git push gh

sync: (pull) (push)