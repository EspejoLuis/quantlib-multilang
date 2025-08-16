#!/usr/bin/env bash
set -euo pipefail

# coverage_integration.sh — run only the Integration tests with coverage.
# Usage:
#   ./coverage_integration.sh        # run and open report
#   ./coverage_integration.sh --clean# clean, run, and open report

# --- Resolve paths relative to this script ---
SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)"
CSHARP_ROOT="$(cd "$SCRIPT_DIR/.." && pwd -P)"
TEST_PROJ="$CSHARP_ROOT/tests/QuantLibCSharp.IntegrationTests"
RESULTS_DIR="$TEST_PROJ/TestResults"
REPORT_DIR="$CSHARP_ROOT/CoverageReportIntegration"
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

  # Also wipe build outputs so PDBs are regenerated
  rm -rf "$CSHARP_ROOT/src/QuantLibCSharp/bin" "$CSHARP_ROOT/src/QuantLibCSharp/obj"
  rm -rf "$TEST_PROJ/bin" "$TEST_PROJ/obj"

  # Optional: clear local NuGet caches if things still look stale
  # dotnet nuget locals all --clear
fi

# --- Run only the Integration tests with Coverlet collector ---
echo "[coverage] Running integration tests with coverage…"
pushd "$TEST_PROJ" >/dev/null
dotnet test --nologo --collect:"XPlat Code Coverage" --results-directory "$RESULTS_DIR"
popd >/dev/null

# --- Generate HTML report via reportgenerator tool ---
echo "[coverage] Generating HTML report…"
pushd "$CSHARP_ROOT" >/dev/null
dotnet tool run reportgenerator \
  -reports:"tests/QuantLibCSharp.IntegrationTests/TestResults/**/coverage.cobertura.xml" \
  -targetdir:"CoverageReportIntegration" \
  -reporttypes:"Html;TextSummary"
popd >/dev/null

# --- Open the report (macOS 'open', Linux 'xdg-open' fallback) ---
if [[ -f "$REPORT_INDEX" ]]; then
  echo "[coverage] Report ready: $REPORT_INDEX"
  if command -v open >/dev/null 2>&1; then
    open "$REPORT_INDEX"
  else
    echo "[coverage] Please open in your browser: $REPORT_INDEX"
  fi
else
  echo "[coverage] ERROR: Report not found at $REPORT_INDEX" >&2
  exit 1
fi

echo "[coverage] Done."
