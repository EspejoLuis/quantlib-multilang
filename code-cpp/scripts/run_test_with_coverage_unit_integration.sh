#!/bin/bash
# run_test_with_coverage.sh - Build and run C++ coverage for QuantLibCpp
# Usage:
#   ./run_test_with_coverage.sh          # reuse existing build
#   ./run_test_with_coverage.sh --clean  # delete build and start fresh

set -e  # stop if any command fails

# 1. Optional clean
if [ "$1" == "--clean" ]; then
    echo "Cleaning build-coverage folder..."
    rm -rf build-coverage
fi

# 2. Configure (only if folder doesn't exist or was cleaned)
if [ ! -d "build-coverage" ]; then
    echo "Configuring build with coverage instrumentation..."
    cmake -S . -B build-coverage -DCMAKE_BUILD_TYPE=Coverage
fi

# 3. Build
echo "Building project..."
cmake --build build-coverage

# 4. Run coverage target
echo "Running coverage..."
cmake --build build-coverage --target coverage
