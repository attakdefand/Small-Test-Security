# Run Extended API Tests
# This script runs all the extended tests that have been added to the framework

Write-Host "🧪 Running Extended API Tests" -ForegroundColor Green
Write-Host "============================" -ForegroundColor Green

# Get current timestamp for report naming
$timestamp = Get-Date -Format "yyyy-MM-dd_HH-mm-ss"
$reportDir = "test-results"
$dateDir = Get-Date -Format "yyyy-MM"

# Create directory structure
if (!(Test-Path "$reportDir\$dateDir")) {
    New-Item -ItemType Directory -Path "$reportDir\$dateDir" -Force | Out-Null
}

# Define output files
$stdoutFile = "$reportDir\$dateDir\extended-test-report_$timestamp.txt"
$summaryFile = "$reportDir\$dateDir\extended-summary_$timestamp.txt"

Write-Host "Timestamp: $timestamp" -ForegroundColor Yellow
Write-Host "Output will be saved to: $stdoutFile" -ForegroundColor Cyan

# Save command and timestamp to summary
"Extended Test Run Summary" | Out-File -FilePath $summaryFile
"=======================" | Out-File -FilePath $summaryFile -Append
"Timestamp: $timestamp" | Out-File -FilePath $summaryFile -Append
"Base URL: $($env:BASE_URL)" | Out-File -FilePath $summaryFile -Append
"" | Out-File -FilePath $summaryFile -Append

# Test categories to run
$testCategories = @(
    "Core Tests",
    "GraphQL Tests", 
    "WebSocket Tests",
    "OpenAPI Validation",
    "Chaos Engineering"
)

Write-Host "Running test categories:" -ForegroundColor Cyan
$testCategories | ForEach-Object { Write-Host "  - $_" }

Write-Host ""

# Run core tests first
Write-Host "1. Running Core Tests..." -ForegroundColor Yellow
try {
    $coreOutput = cargo test 2>&1
    $coreOutput | Out-File -FilePath $stdoutFile -Append
    Write-Host "   ✅ Core tests completed" -ForegroundColor Green
} catch {
    Write-Host "   ❌ Core tests failed: $($_.Exception.Message)" -ForegroundColor Red
}

Write-Host ""

# Run GraphQL tests
Write-Host "2. Running GraphQL Tests..." -ForegroundColor Yellow
try {
    $graphqlOutput = cargo test graphql_query_test 2>&1
    $graphqlOutput | Out-File -FilePath $stdoutFile -Append
    Write-Host "   ✅ GraphQL tests completed" -ForegroundColor Green
} catch {
    Write-Host "   ⚠️  GraphQL tests skipped or failed (might need GraphQL endpoint)" -ForegroundColor Yellow
}

Write-Host ""

# Run WebSocket tests
Write-Host "3. Running WebSocket Tests..." -ForegroundColor Yellow
try {
    $wsOutput = cargo test websocket_connection_test 2>&1
    $wsOutput | Out-File -FilePath $stdoutFile -Append
    Write-Host "   ✅ WebSocket tests completed" -ForegroundColor Green
} catch {
    Write-Host "   ⚠️  WebSocket tests skipped or failed (might need WebSocket endpoint)" -ForegroundColor Yellow
}

Write-Host ""

# Run OpenAPI validation tests
Write-Host "4. Running OpenAPI Validation Tests..." -ForegroundColor Yellow
try {
    $openapiOutput = cargo test openapi_spec_validation 2>&1
    $openapiOutput | Out-File -FilePath $stdoutFile -Append
    Write-Host "   ✅ OpenAPI validation tests completed" -ForegroundColor Green
} catch {
    Write-Host "   ⚠️  OpenAPI validation tests skipped or failed (might need OpenAPI spec)" -ForegroundColor Yellow
}

Write-Host ""

# Run chaos engineering tests
Write-Host "5. Running Chaos Engineering Tests..." -ForegroundColor Yellow
try {
    $chaosOutput = cargo test api_resilience_under_network_issues 2>&1
    $chaosOutput | Out-File -FilePath $stdoutFile -Append
    Write-Host "   ✅ Chaos engineering tests completed" -ForegroundColor Green
} catch {
    Write-Host "   ⚠️  Chaos engineering tests skipped or failed" -ForegroundColor Yellow
}

Write-Host ""

# Run advanced fuzzing tests
Write-Host "6. Running Advanced Fuzzing Tests..." -ForegroundColor Yellow
try {
    $fuzzOutput = cargo test fuzz_api_inputs 2>&1
    $fuzzOutput | Out-File -FilePath $stdoutFile -Append
    Write-Host "   ✅ Advanced fuzzing tests completed" -ForegroundColor Green
} catch {
    Write-Host "   ⚠️  Advanced fuzzing tests skipped or failed" -ForegroundColor Yellow
}

Write-Host ""

# Run all ignored tests (this will run all extended tests)
Write-Host "7. Running All Ignored Tests (Extended)..." -ForegroundColor Yellow
try {
    $ignoredOutput = cargo test -- --ignored 2>&1
    $ignoredOutput | Out-File -FilePath $stdoutFile -Append
    Write-Host "   ✅ All ignored tests completed" -ForegroundColor Green
    
    # Count passed/failed
    $passed = ($ignoredOutput | Select-String -Pattern "test result: ok. (\d+) passed").Matches.Groups[1].Value
    $failed = ($ignoredOutput | Select-String -Pattern "test result: FAILED. (\d+) passed").Matches.Groups[1].Value
    
    if ($passed) {
        "Extended Tests Passed: $passed" | Out-File -FilePath $summaryFile -Append
    }
    if ($failed) {
        "Extended Tests Failed: $failed" | Out-File -FilePath $summaryFile -Append
    }
} catch {
    Write-Host "   ⚠️  Some ignored tests failed" -ForegroundColor Yellow
}

Write-Host ""

# Show summary
Write-Host "📋 Test Results Summary:" -ForegroundColor Green
Write-Host "=======================" -ForegroundColor Green
Get-Content $summaryFile

Write-Host ""
Write-Host "📄 Full output saved to: $stdoutFile" -ForegroundColor Cyan
Write-Host "📊 Summary saved to: $summaryFile" -ForegroundColor Cyan

Write-Host ""
Write-Host "🎉 Extended testing completed!" -ForegroundColor Green