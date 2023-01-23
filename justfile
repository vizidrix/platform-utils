build:
    cargo build

test-all:
    cargo test --workspace

cov:
    cargo llvm-cov --all-features --workspace --lcov --output-path=coverage/lcov.info

cov-html:
    cargo llvm-cov --html

report:
    cargo llvm-cov --workspace
