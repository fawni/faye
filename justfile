set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

_default:
    @just --list

test:
    cargo test --all-features

lint:
    cargo clippy -- -W clippy::pedantic -W clippy::nursery

install:
    @cargo install --path .

uninstall:
    @cargo uninstall faye

# Clean up all build artifacts across all the crates
clean:
    cargo clean
    cargo clean -p faye-web
    trunk clean -d faye-web/dist

pull:
    git pull
    git pull gh master

push: (test)
    git push
    git push gh

sync: (pull) (push)

changelog:
    git cliff -o CHANGELOG.md