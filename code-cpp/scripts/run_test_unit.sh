#!/bin/bash
# run_test.sh - Build and run C++ coverage for QuantLibCpp
# Usage:
#   ./run_test.sh          # reuse existing build
#   ./run_test.sh --clean  # delete build and start fresh

set -e  # stop if any command fails

# 1. Optional clean
if [ "$1" == "--clean" ]; then
    echo "Cleaning build folder..."
    rm -rf build
fi

# 2. Configure (only if folder doesn't exist or was cleaned)
if [ ! -d "build" ]; then
    echo "Configuring build with coverage instrumentation..."
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug
fi

# 3. Build target
echo "Running coverage..."
cmake --build build --target date_tests

# 4. Run tests from inside build folder
echo "Running date_tests from build/..."
cd build
./date_tests
cd ..