#!/usr/bin/env bash
# run_coverage_unit.sh — run unit tests with coverage, build HTML report, and open it.

set -euo pipefail

# --- Config ---
VENV_PATH="/Users/apple/github_repos/quantlib-multilang/code-python/.venv/bin/activate"
REPORT_DIR="htmlcov_unit"
REPORT_PATH="$REPORT_DIR/index.html"

# --- Activate venv ---
if [ -f "$VENV_PATH" ]; then
    # shellcheck disable=SC1090
    source "$VENV_PATH"
    echo "✅ Virtual environment activated."
else
    echo "❌ Could not find venv at $VENV_PATH"
    exit 1
fi

# --- Run coverage ---
echo "▶️ Running unit tests with coverage..."
coverage run --source=src -m pytest tests/unit

echo "▶️ Building HTML coverage report..."
coverage html -d "$REPORT_DIR"

# --- Open report ---
if [ -f "$REPORT_PATH" ]; then
    echo "🌐 Opening unit report in browser..."
    open "$REPORT_PATH"
else
    echo "❌ Report not found at $REPORT_PATH"
    exit 1
fi
