#!/bin/bash

# Test Runner Script with Timestamped Reports
# Usage: ./run-tests.sh [--ignored] [--specific-test test_name]

# Parse command line arguments
IGNORED=false
SPECIFIC_TEST=""

while [[ $# -gt 0 ]]; do
    case $1 in
        --ignored)
            IGNORED=true
            shift
            ;;
        --specific-test)
            SPECIFIC_TEST="$2"
            shift 2
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Get current timestamp for report naming
TIMESTAMP=$(date +"%Y-%m-%d_%H-%M-%S")
REPORT_DIR="test-results"
DATE_DIR=$(date +"%Y-%m")

# Create directory structure
mkdir -p "$REPORT_DIR/$DATE_DIR"

# Define output files
STDOUT_FILE="$REPORT_DIR/$DATE_DIR/test-report_$TIMESTAMP.txt"
SUMMARY_FILE="$REPORT_DIR/$DATE_DIR/summary_$TIMESTAMP.txt"

echo "Running API tests..."
echo "Timestamp: $TIMESTAMP"
echo "Output will be saved to: $STDOUT_FILE"

# Build the cargo test command
CARGO_COMMAND="cargo test"
if [ "$IGNORED" = true ]; then
    CARGO_COMMAND="$CARGO_COMMAND -- --ignored"
fi
if [ -n "$SPECIFIC_TEST" ]; then
    CARGO_COMMAND="$CARGO_COMMAND $SPECIFIC_TEST"
fi

# Save command and timestamp to summary
echo "Test Run Summary" > "$SUMMARY_FILE"
echo "================" >> "$SUMMARY_FILE"
echo "Timestamp: $TIMESTAMP" >> "$SUMMARY_FILE"
echo "Command: $CARGO_COMMAND" >> "$SUMMARY_FILE"
echo "Base URL: ${BASE_URL:-Not set}" >> "$SUMMARY_FILE"
echo "" >> "$SUMMARY_FILE"

echo "Executing: $CARGO_COMMAND"

# Run tests and capture output
if eval "$CARGO_COMMAND" > "$STDOUT_FILE" 2>&1; then
    # Parse results for summary
    PASSED=$(grep -o "test result: ok. [0-9]* passed" "$STDOUT_FILE" | grep -o "[0-9]* passed" | grep -o "[0-9]*")
    FAILED=$(grep -o "test result: FAILED. [0-9]* passed" "$STDOUT_FILE" | grep -o "[0-9]* passed" | grep -o "[0-9]*")
    
    if [ -n "$PASSED" ]; then
        echo "Tests Passed: $PASSED" >> "$SUMMARY_FILE"
    fi
    if [ -n "$FAILED" ]; then
        echo "Tests Failed: $FAILED" >> "$SUMMARY_FILE"
    fi
    
    echo "Test Results Summary:"
    cat "$SUMMARY_FILE"
    
    echo "Full output saved to: $STDOUT_FILE"
    echo "Summary saved to: $SUMMARY_FILE"
else
    echo "Error occurred during test execution" >> "$SUMMARY_FILE"
    echo "Error running tests. Check $STDOUT_FILE for details."
fi

echo "Test run completed!"