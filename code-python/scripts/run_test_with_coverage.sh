#!/usr/bin/env bash
# run_coverage.sh — run pytest with coverage, build HTML report, and open it.

set -euo pipefail

# --- Config ---
VENV_PATH="/Users/apple/github_repos/quantlib-multilang/code-python/.venv/bin/activate"
REPORT_PATH="htmlcov/index.html"

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
echo "▶️ Running tests with coverage..."
coverage run --source=src -m pytest

echo "▶️ Building HTML coverage report..."
coverage html

# --- Open report ---
if [ -f "$REPORT_PATH" ]; then
    echo "🌐 Opening report in browser..."
    open "$REPORT_PATH"
else
    echo "❌ Report not found at $REPORT_PATH"
    exit 1
fi
