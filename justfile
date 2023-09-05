default:
    @just -l

loc:
    find src/ -name "*.rs" | xargs cat | wc -l

build:
    cargo build -q

install:
    exit 1
    cargo build --release

publish:
    cargo clippy -q -- -D warnings
    cargo test -q
