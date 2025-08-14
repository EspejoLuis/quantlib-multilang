#!/usr/bin/env bash
set -euo pipefail

# coverage_test.sh — run C# tests with coverage and open the HTML report.
# Usage:
#   ./coverage_test.sh        # run and open report
#   ./coverage_test.sh --clean# clean, run, and open report

# --- Resolve paths relative to this script ---
SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)"
CSHARP_ROOT="$(cd "$SCRIPT_DIR/.." && pwd -P)"
TEST_PROJ="$CSHARP_ROOT/tests/QuantLibCSharp.Tests"
RESULTS_DIR="$TEST_PROJ/TestResults"
REPORT_DIR="$CSHARP_ROOT/CoverageReport"
REPORT_INDEX="$REPORT_DIR/index.html"

# --- Options: only '--clean' is supported ---
CLEAN=false
if [[ "${1:-}" == "--clean" ]]; then
  CLEAN=true
elif [[ "${1:-}" != "" ]]; then
  echo "Usage: $(basename "$0") [--clean]"
  exit 1
fi

# --- Clean if requested ---
if $CLEAN; then
  echo "[coverage] Cleaning old artifacts…"
  rm -rf "$RESULTS_DIR" "$REPORT_DIR"
fi

# --- Run tests with Coverlet collector ---
echo "[coverage] Running tests with coverage…"
pushd "$TEST_PROJ" >/dev/null
dotnet test --nologo --collect:"XPlat Code Coverage" --results-directory "$RESULTS_DIR"
popd >/dev/null

# --- Generate HTML report via reportgenerator tool ---
echo "[coverage] Generating HTML report…"
pushd "$CSHARP_ROOT" >/dev/null
dotnet tool run reportgenerator \
  -reports:"tests/QuantLibCSharp.Tests/TestResults/**/coverage.cobertura.xml" \
  -targetdir:"CoverageReport" \
  -reporttypes:"Html;TextSummary"
popd >/dev/null

# --- Open the report (macOS 'open', Linux 'xdg-open' fallback) ---
if [[ -f "$REPORT_INDEX" ]]; then
  echo "[coverage] Report ready: $REPORT_INDEX"
  if command -v open >/dev/null 2>&1; then
    open "$REPORT_INDEX"
  elif command -v xdg-open >/dev/null 2>&1; then
    xdg-open "$REPORT_INDEX"
  else
    echo "[coverage] Please open in your browser: $REPORT_INDEX"
  fi
else
  echo "[coverage] ERROR: Report not found at $REPORT_INDEX" >&2
  exit 1
fi

echo "[coverage] Done."