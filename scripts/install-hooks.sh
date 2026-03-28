#!/bin/sh
# Install git hooks for materia development.
# Run once: ./scripts/install-hooks.sh
#
# The pre-commit hook runs the SAME checks as CI:
#   1. cargo fmt (auto-applied)
#   2. cargo clippy -D warnings (blocks commit)
#   3. cargo test (blocks commit)
#   4. cargo doc (blocks commit)
#
# This means: if pre-commit passes, CI WILL pass. No surprises.

HOOK_DIR="$(git rev-parse --git-dir)/hooks"
mkdir -p "$HOOK_DIR"

cat > "$HOOK_DIR/pre-commit" << 'HOOK'
#!/bin/sh
set -e

echo "=== Pre-commit: same checks as CI ==="

# 1. Auto-format (applied, not just checked)
echo "[1/4] cargo fmt..."
cargo fmt --all
git diff --name-only | xargs -r git add

# 2. Lint (blocks commit)
echo "[2/4] cargo clippy..."
cargo clippy --all-targets -- -D warnings

# 3. Test (blocks commit)
echo "[3/4] cargo test..."
cargo test --all --quiet

# 4. Docs (blocks commit)
echo "[4/4] cargo doc..."
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --quiet

echo "=== All checks passed ==="
HOOK

chmod +x "$HOOK_DIR/pre-commit"
echo "Git hooks installed. Pre-commit mirrors CI pipeline."
