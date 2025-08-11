#!/bin/bash
# run_test_integration.sh - Build and run C++ integration tests (Debug)
# Usage:
#   ./run_test_integration.sh          # reuse existing build
#   ./run_test_integration.sh --clean  # delete build and start fresh

set -e

# 1) Optional clean
if [ "$1" == "--clean" ]; then
  echo "Cleaning build folder..."
  rm -rf build
fi

# 2) Configure (Debug) if needed
if [ ! -d "build" ]; then
  echo "Configuring Debug build..."
  cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug
fi

# 3) Build integration test target
echo "Building date_integration..."
cmake --build build --target date_tests_integration

# 4) Run integration tests
echo "Running date_tests_integration from build/..."
(cd build && ./date_tests_integration)
