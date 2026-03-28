#!/bin/sh
# Install git hooks for materia development.
# Run once: ./scripts/install-hooks.sh

HOOK_DIR="$(git rev-parse --git-dir)/hooks"
mkdir -p "$HOOK_DIR"

cat > "$HOOK_DIR/pre-commit" << 'HOOK'
#!/bin/sh
# Pre-commit hook: auto-format and lint check.
# Formatting is applied automatically. Lint errors block the commit.

set -e

# Auto-format staged Rust files
echo "Running cargo fmt..."
cargo fmt --all

# Re-stage any files that were reformatted
git diff --name-only | xargs -r git add

# Lint check — block commit if there are warnings
echo "Running cargo clippy..."
cargo clippy --all-targets -- -D warnings
if [ $? -ne 0 ]; then
    echo ""
    echo "ERROR: clippy found warnings. Fix them before committing."
    exit 1
fi

echo "Pre-commit checks passed."
HOOK

chmod +x "$HOOK_DIR/pre-commit"
echo "Git hooks installed."
