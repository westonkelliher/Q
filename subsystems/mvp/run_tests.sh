#!/bin/bash
# Run all e2e test scripts

set -e

echo "======================================"
echo "Running E2E Test Suite"
echo "======================================"
echo ""

TESTS_DIR="tests"
PASSED=0
FAILED=0

# Color codes
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

run_test() {
    local test_file=$1
    local test_name=$(basename "$test_file" .txt)
    
    echo "Running: $test_name"
    if cargo run --quiet script "$test_file" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASSED${NC}: $test_name"
        ((PASSED++))
    else
        echo -e "${RED}✗ FAILED${NC}: $test_name"
        ((FAILED++))
    fi
    echo ""
}

# Run each test
for test_file in "$TESTS_DIR"/e2e_*.txt; do
    if [ -f "$test_file" ]; then
        run_test "$test_file"
    fi
done

# Summary
echo "======================================"
echo "Test Summary"
echo "======================================"
echo -e "Passed: ${GREEN}$PASSED${NC}"
echo -e "Failed: ${RED}$FAILED${NC}"
echo "Total:  $((PASSED + FAILED))"
echo ""

if [ $FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed.${NC}"
    exit 1
fi
