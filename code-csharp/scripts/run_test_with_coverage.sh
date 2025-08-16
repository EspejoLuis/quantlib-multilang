#!/usr/bin/env bash
set -euo pipefail

# run_test_with_coverage.sh — run C# Unit + Integration tests with coverage and open ONE merged HTML report.
# Usage:
#   ./run_test_with_coverage.sh
#   ./run_test_with_coverage.sh --clean    # wipe outputs, rebuild, re-run

# --- Resolve paths relative to this script ---
SCRIPT_DIR="$(cd -- "$(dirname "${BASH_SOURCE[0]}")" &>/dev/null && pwd -P)"
CSHARP_ROOT="$(cd "$SCRIPT_DIR/.." && pwd -P)"

UNIT_PROJ="$CSHARP_ROOT/tests/QuantLibCSharp.UnitTests"
INTEG_PROJ="$CSHARP_ROOT/tests/QuantLibCSharp.IntegrationTests"

UNIT_RESULTS_DIR="$UNIT_PROJ/TestResults"
INTEG_RESULTS_DIR="$INTEG_PROJ/TestResults"

REPORT_DIR="$CSHARP_ROOT/CoverageReportAll"
REPORT_INDEX="$REPORT_DIR/index.html"

# --- Options ---
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
  rm -rf "$UNIT_RESULTS_DIR" "$INTEG_RESULTS_DIR" "$REPORT_DIR"

  # Also wipe build outputs so PDBs are regenerated (prevents stale paths in coverage)
  rm -rf "$CSHARP_ROOT/src/QuantLibCSharp/bin" "$CSHARP_ROOT/src/QuantLibCSharp/obj"
  rm -rf "$UNIT_PROJ/bin" "$UNIT_PROJ/obj"
  rm -rf "$INTEG_PROJ/bin" "$INTEG_PROJ/obj"

  # Optional: clear local NuGet caches if anything still looks stale
  # dotnet nuget locals all --clear
fi

# --- Run Unit tests with Coverlet collector ---
echo "[coverage] Running UNIT tests with coverage…"
pushd "$UNIT_PROJ" >/dev/null
dotnet test --nologo --collect:"XPlat Code Coverage" --results-directory "$UNIT_RESULTS_DIR"
popd >/dev/null

# --- Run Integration tests with Coverlet collector ---
echo "[coverage] Running INTEGRATION tests with coverage…"
pushd "$INTEG_PROJ" >/dev/null
dotnet test --nologo --collect:"XPlat Code Coverage" --results-directory "$INTEG_RESULTS_DIR"
popd >/dev/null

# --- Generate ONE merged HTML report via ReportGenerator ---
echo "[coverage] Generating merged HTML report…"
pushd "$CSHARP_ROOT" >/dev/null

# NOTE: We intentionally include both test result globs. ReportGenerator will merge them.
# If you want to exclude test assemblies from the report, uncomment the -assemblyfilters line.
dotnet tool run reportgenerator \
  -reports:"tests/QuantLibCSharp.UnitTests/TestResults/**/coverage.cobertura.xml;tests/QuantLibCSharp.IntegrationTests/TestResults/**/coverage.cobertura.xml" \
  -targetdir:"CoverageReportAll" \
  -reporttypes:"Html;TextSummary"
  # -assemblyfilters:"+QuantLibCSharp;-QuantLibCSharp.*Tests"

popd >/dev/null

# --- Open the merged report (macOS 'open', Linux 'xdg-open' fallback) ---
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
