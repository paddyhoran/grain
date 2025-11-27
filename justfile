
dev: fmt-dev clippy test-dev

# Check formatting (development context)
fmt-dev:
    @echo "==> Formatting..."
    @cargo fmt --all

# Check formatting (CI version)
fmt-ci:
    @echo "==> Checking formatting..."
    @cargo fmt --all -- --check

# Lint with Clippy
clippy:
    @echo "==> Linting with Clippy..."
    @cargo clippy --all-targets -- -D warnings

# Audit dependencies for security vulnerabilities
audit:
    @echo "==> Auditing dependencies..."
    @cargo audit

# Check for license compliance
check-licenses:
    @echo "==> Checking licenses..."
    @cargo deny check licenses

# Check for outdated dependencies
check-outdated:
    @echo "==> Checking for outdated dependencies..."
    @cargo outdated --exit-code 0

# Unit-tests and doc-tests only in release mode
test-unit-rel package:
    @echo "==> Running unit tests (release)..."
    @cargo test --lib --release --package {{package}}
    @cargo test --doc --release --package {{package}}

# Integration tests only in release mode
test-integration-rel package:
    @echo "==> Running integration tests (release)..."
    @cargo test --tests --release --package {{package}}

# Runs all tests for all packages
test-dev:
    @echo "==> Running tests (debug)..."
    @cargo test

debug test_substring:
    # not filtering correctly
    @cargo test {{test_substring}} -- --nocapture | rg --color never --invert-match '^test result|^running|^\s+*Running'

# Runs all tests for all packages (RELEASE MODE)
test-rel:
    @echo "==> Running tests (debug)..."
    @cargo test --release

