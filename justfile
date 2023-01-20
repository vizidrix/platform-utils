build:
    cargo build

test-all:
    cargo test --workspace

cov:
    cargo llvm-cov --output-path=coverage/lcov.info --lcov

cov-html:
    cargo llvm-cov --html

report:
    cargo llvm-cov --workspace
