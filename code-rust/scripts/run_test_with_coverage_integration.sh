#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/.." &>/dev/null && pwd -P)"
REPORT_DIR="$PROJECT_ROOT/CoverageReportIntegration"
REPORT_FILE="$REPORT_DIR/coverage_integration.lcov"
HTML_DIR="$REPORT_DIR/html"

CLEAN=false
if [[ "${1:-}" == "--clean" ]]; then
  CLEAN=true
elif [[ "${1:-}" != "" ]]; then
  echo "Usage: $0 [--clean]"
  exit 1
fi

if $CLEAN; then
  echo "Cleaning $REPORT_DIR..."
  rm -rf "$REPORT_DIR"
fi
mkdir -p "$REPORT_DIR"

echo "Running INTEGRATION tests with coverage..."
cargo +nightly llvm-cov --all-features --branch --lcov --output-path "$REPORT_FILE"

echo "Generating HTML from LCOV..."
genhtml "$REPORT_FILE" --output-directory "$HTML_DIR" --legend --branch-coverage

echo "Opening HTML report..."
open "$HTML_DIR/index.html"
