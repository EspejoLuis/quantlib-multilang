#!/bin/bash
# run_test_unit.sh - Build and run C++ coverage for QuantLibCpp
# Usage:
#   ./run_test_unit.sh          # reuse existing build
#   ./run_test_unit.sh --clean  # delete build and start fresh

set -e  # stop if any command fails

# 1. Optional clean
if [ "$1" == "--clean" ]; then
    echo "Cleaning build folder..."
    rm -rf build
fi

# 2. Configure (only if folder doesn't exist or was cleaned)
if [ ! -d "build" ]; then
    echo "Configuring build for debug.."
    cmake -S . -B build -DCMAKE_BUILD_TYPE=Debug
fi

# 3. Build target
echo "Running date_tests_unit..."
cmake --build build --target date_tests_unit

# 4. Run tests from inside build folder
echo "Running date_tests_unit from build/..."
cd build
./date_tests_unit
cd ..