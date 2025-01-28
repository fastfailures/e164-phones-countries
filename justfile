semver_baseline_file := ".semver-baseline"

default: check

# Alias for check-lite
c: check-lite

# Run quickest linters and tests
check-lite: format spell test-quick

# Run most important linters and tests. Better to be done before commit
check: check-lite doc lint lint-wasm test-slow
    @ echo -n "Ready to commit! "
    @ echo -n "🎆🎇🎉🎊🚀🎯🏅🥂🍾🍻" | grep -o . | shuf -n1

# Run complete test suite (not run: unused, coverage). Very time consuming
check-full: check outdated semver mutants audit
    @ echo -n "Everything's good! 🍰"

# Check for unused dependencies. ATTENTION: runs `cargo clean` and could modify Cargo.lock
unused:
    cargo clean
    cargo build
    cargo machete --with-metadata

# Scan for vulnerabilities
audit:
    cargo audit

# Automatic code formatting
format:
    cargo fmt

# Find common misspellings
spell:
    codespell

# Fix common misspellings
spell-fix:
    @ echo "In case of false positives: add exceptions to `.codespellrc`"
    codespell -wi3

# Aggressive linting
lint:
    cargo clippy --all-features --all-targets -- --forbid unsafe_code --deny warnings
    pre-commit run --all-files

# Check compatibility with WASM target
lint-wasm:
    cargo clippy --target=wasm32-unknown-unknown
# FIXME: in CI too!

# Run tests
test-quick:
    cargo test

# Run (only) expensive tests, usually ignored
test-slow:
    cargo test -- --ignored

# Build project documentation
doc:
    cargo doc --no-deps --all-features

# Build and open project documentation
doc-open: doc
    cargo doc --open --lib

# Check API changes for semver violations
semver:
    cargo semver-checks --baseline-rev "$(cat {{semver_baseline_file}})"

# Check for outdated dependencies
outdated:
    cargo outdated --root-deps-only --exit-code 1
# cargo outdated --root-deps-only --exit-code 1 --exclude {{outated_exclude}}

# Run mutation testing
mutants:
    cargo mutants
# https://mutants.rs/

# Check test coverage
coverage:
    cargo tarpaulin --skip-clean --target-dir target/coverage # to avoid recompilation

# Remove build artifacts and unused devenv files
clean:
    cargo clean
    devenv gc

# Publish library crates to crates.ceamgroup.it
publish-libs:
    @echo "Publishing to the crate registry is better done manually"
    @echo 'Example: "cargo publish -p coreapi_clock"'

# Update dependencies and commit changes to Cargo.lock
update-deps:
    cargo update
    git add Cargo.lock
    git status --short
    @echo '"Now commit with: "git commit -m "chore(deps): update"'

# Update development environment and commit changes to devenv.lock
update-devenv:
    devenv update
    git add devenv.lock
    git status --short
    @echo 'Now commit with: "git commit -m "chore(devenv): update"'

# Print help about 'just'
help:
    @ just --list
    @ echo "More help: https://just.systems/man/en"
