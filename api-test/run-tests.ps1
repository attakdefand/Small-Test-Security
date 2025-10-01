# Test Runner Script with Timestamped Reports
# Usage: .\run-tests.ps1 [-ignored] [-specific-test "test_name"]

param(
    [switch]$ignored,
    [string]$specificTest
)

# Get current timestamp for report naming
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$reportDir = "test-results"
$dateDir = Get-Date -Format "yyyy-MM"

# Create directory structure
if (!(Test-Path "$reportDir\$dateDir")) {
    New-Item -ItemType Directory -Path "$reportDir\$dateDir" -Force | Out-Null
}

# Define output files
$stdoutFile = "$reportDir\$dateDir\test-report_$timestamp.txt"
$summaryFile = "$reportDir\$dateDir\summary_$timestamp.txt"

Write-Host "Running API tests..." -ForegroundColor Green
Write-Host "Timestamp: $timestamp" -ForegroundColor Yellow
Write-Host "Output will be saved to: $stdoutFile" -ForegroundColor Cyan

# Build the cargo test command
$cargoCommand = "cargo test"
if ($ignored) {
    $cargoCommand += " -- --ignored"
}
if ($specificTest) {
    $cargoCommand += " $specificTest"
}

# Run tests and capture output
Write-Host "Executing: $cargoCommand" -ForegroundColor Blue

# Save command and timestamp to summary
"Test Run Summary" | Out-File -FilePath $summaryFile
"================" | Out-File -FilePath $summaryFile -Append
"Timestamp: $timestamp" | Out-File -FilePath $summaryFile -Append
"Command: $cargoCommand" | Out-File -FilePath $summaryFile -Append
"Base URL: $($env:BASE_URL)" | Out-File -FilePath $summaryFile -Append
"" | Out-File -FilePath $summaryFile -Append

try {
    # Run the tests and capture both stdout and stderr
    $output = Invoke-Expression $cargoCommand 2>&1
    $output | Out-File -FilePath $stdoutFile
    
    # Parse results for summary
    $passed = ($output | Select-String -Pattern "test result: ok. (\d+) passed").Matches.Groups[1].Value
    $failed = ($output | Select-String -Pattern "test result: FAILED. (\d+) passed").Matches.Groups[1].Value
    $ignoredTests = ($output | Select-String -Pattern "(\d+) ignored").Matches.Groups[1].Value
    
    if ($passed) {
        "Tests Passed: $passed" | Out-File -FilePath $summaryFile -Append
    }
    if ($failed) {
        "Tests Failed: $failed" | Out-File -FilePath $summaryFile -Append
    }
    if ($ignoredTests) {
        "Tests Ignored: $ignoredTests" | Out-File -FilePath $summaryFile -Append
    }
    
    # Show summary
    Write-Host "Test Results Summary:" -ForegroundColor Green
    Get-Content $summaryFile
    
    Write-Host "Full output saved to: $stdoutFile" -ForegroundColor Cyan
    Write-Host "Summary saved to: $summaryFile" -ForegroundColor Cyan
    
} catch {
    "Error occurred: $($_.Exception.Message)" | Out-File -FilePath $summaryFile -Append
    Write-Host "Error running tests: $($_.Exception.Message)" -ForegroundColor Red
    Write-Host "Full error details saved to: $stdoutFile" -ForegroundColor Cyan
}

Write-Host "Test run completed!" -ForegroundColor Green