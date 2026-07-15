name := 'wmde-start-menu'
export APPID := 'fun.wmde.start-menu'
import "res/packaging.just"

# Default recipe which runs `just build-release`
[private]
default:
    @just --list

# Runs `cargo clean`
clean:
    cargo clean

# Removes vendored dependencies
clean-vendor:
    rm -rf .cargo vendor vendor.tar

# Runs `cargo clean` and removes vendored dependencies
clean-dist: clean clean-vendor

# Runs `cargo fmt`
fmt:
    cargo fmt

# Runs a clippy check
check *args: fmt
    cargo clippy --all-features {{args}} -- -W clippy::pedantic

# Runs a clippy check with JSON message format
check-json: (check '--message-format=json')

# Run the applet with args
run *args:
    cargo run -p wmde-start-menu-applet {{args}}

# Run the applet with debug logs (simple_logger parses RUST_LOG as a bare level)
run-logs *args:
    env RUST_LOG=info RUST_BACKTRACE=full cargo run --release -p wmde-start-menu-applet {{args}}

spellcheck *args:
	@codespell --skip="./i18n" --skip="./.git" --skip="./target" --builtin clear,rare,informal,code --ignore-words-list mut,crate {{args}}
	@echo Spellings look good!
