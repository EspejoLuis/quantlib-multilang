#!/usr/bin/env bash
# activate_venv.sh — activates the quantlib-python venv

# Absolute path to your venv’s activate script
VENV_PATH="/Users/apple/github_repos/quantlib-multilang/code-python/.venv/bin/activate"

if [ -f "$VENV_PATH" ]; then
    # shellcheck disable=SC1090
    source "$VENV_PATH"
    echo "✅ Virtual environment activated."
else
    echo "❌ Could not find venv at $VENV_PATH"
    exit 1
fi
