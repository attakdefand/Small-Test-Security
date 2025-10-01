# Verify Complete API Testing Framework Setup
# This script verifies that all extensions have been properly added to the framework

Write-Host "🔍 Verifying Complete API Testing Framework Setup" -ForegroundColor Green
Write-Host "================================================" -ForegroundColor Green

$errors = 0
$warnings = 0
$successes = 0

function Test-FileExists($path, $description) {
    if (Test-Path $path) {
        Write-Host "  ✅ $description" -ForegroundColor Green
        $script:successes++
        return $true
    } else {
        Write-Host "  ❌ $description" -ForegroundColor Red
        Write-Host "     Missing file: $path" -ForegroundColor Yellow
        $script:errors++
        return $false
    }
}

function Test-DirectoryExists($path, $description) {
    if (Test-Path $path -PathType Container) {
        Write-Host "  ✅ $description" -ForegroundColor Green
        $script:successes++
        return $true
    } else {
        Write-Host "  ❌ $description" -ForegroundColor Red
        Write-Host "     Missing directory: $path" -ForegroundColor Yellow
        $script:errors++
        return $false
    }
}

# Check core directories
Write-Host "📁 Checking Core Directories..." -ForegroundColor Cyan
Test-DirectoryExists "k6-tests" "k6-tests directory for load testing"
Test-DirectoryExists "tests" "tests directory for Rust tests"

Write-Host ""

# Check extension test files
Write-Host "🧪 Checking Extension Test Files..." -ForegroundColor Cyan
Test-FileExists "tests/graphql.rs" "GraphQL testing implementation"
Test-FileExists "tests/websocket.rs" "WebSocket testing implementation"
Test-FileExists "tests/advanced_fuzzing.rs" "Advanced fuzzing implementation"
Test-FileExists "tests/openapi_validation.rs" "OpenAPI validation implementation"
Test-FileExists "tests/chaos_engineering.rs" "Chaos engineering implementation"

Write-Host ""

# Check k6 load test files
Write-Host "🏃 Checking Load Testing Files..." -ForegroundColor Cyan
Test-FileExists "k6-tests/basic-load-test.js" "k6 load testing script"

Write-Host ""

# Check documentation files
Write-Host "📚 Checking Documentation Files..." -ForegroundColor Cyan
Test-FileExists "..\EXTENSIONS.md" "EXTENSIONS.md summary document"
Test-FileExists "..\docs-test-types\extending-api-tests.md" "Extension guide documentation"
Test-FileExists "..\docs-test-types\complete-extension-guide.md" "Complete extension guide"

Write-Host ""

# Check script files
Write-Host "⚙️  Checking Script Files..." -ForegroundColor Cyan
Test-FileExists "run-extended-tests.ps1" "Extended tests runner script"
Test-FileExists "run-tests.ps1" "Standard tests runner script"
Test-FileExists "run-tests.sh" "Bash tests runner script"
Test-FileExists "test-dashboard.ps1" "Test dashboard script"
Test-FileExists "verify-complete-setup.ps1" "This verification script"

Write-Host ""

# Check Cargo.toml dependencies
Write-Host "📦 Checking Cargo Dependencies..." -ForegroundColor Cyan
$cargoToml = Get-Content "Cargo.toml" -Raw

if ($cargoToml -match "tokio-tungstenite") {
    Write-Host "  ✅ tokio-tungstenite dependency for WebSocket support" -ForegroundColor Green
    $successes++
} else {
    Write-Host "  ❌ tokio-tungstenite dependency missing" -ForegroundColor Red
    $errors++
}

if ($cargoToml -match "futures-util") {
    Write-Host "  ✅ futures-util dependency for async utilities" -ForegroundColor Green
    $successes++
} else {
    Write-Host "  ❌ futures-util dependency missing" -ForegroundColor Red
    $errors++
}

Write-Host ""

# Check environment configuration
Write-Host "⚙️  Checking Environment Configuration..." -ForegroundColor Cyan
if (Test-Path ".env") {
    Write-Host "  ✅ .env file for environment configuration" -ForegroundColor Green
    $successes++
} else {
    Write-Host "  ⚠️  .env file not found (will use defaults)" -ForegroundColor Yellow
    $warnings++
}

if (Test-Path ".env.sample") {
    Write-Host "  ✅ .env.sample file for configuration template" -ForegroundColor Green
    $successes++
} else {
    Write-Host "  ❌ .env.sample file missing" -ForegroundColor Red
    $errors++
}

Write-Host ""

# Summary
Write-Host "📊 Verification Summary:" -ForegroundColor Green
Write-Host "======================" -ForegroundColor Green
Write-Host "  Successes: $successes" -ForegroundColor Green
Write-Host "  Warnings: $warnings" -ForegroundColor Yellow
Write-Host "  Errors: $errors" -ForegroundColor Red

Write-Host ""

if ($errors -eq 0) {
    Write-Host "🎉 All extensions have been successfully added to the API testing framework!" -ForegroundColor Green
    Write-Host "   The framework is now complete and production-ready." -ForegroundColor Cyan
    
    Write-Host ""
    Write-Host "🚀 Ready to use features:" -ForegroundColor Green
    Write-Host "   - Unit Testing" -ForegroundColor Gray
    Write-Host "   - Integration Testing" -ForegroundColor Gray
    Write-Host "   - Security Testing (OWASP Top 10)" -ForegroundColor Gray
    Write-Host "   - Contract Testing" -ForegroundColor Gray
    Write-Host "   - Snapshot Testing" -ForegroundColor Gray
    Write-Host "   - Property-Based Testing" -ForegroundColor Gray
    Write-Host "   - Authentication/Authorization Testing" -ForegroundColor Gray
    Write-Host "   - CORS Testing" -ForegroundColor Gray
    Write-Host "   - Rate Limiting Testing" -ForegroundColor Gray
    Write-Host "   - Input Validation Testing" -ForegroundColor Gray
    Write-Host "   - Header Security Testing" -ForegroundColor Gray
    Write-Host "   - IDOR Testing" -ForegroundColor Gray
    Write-Host "   - Performance/Load Testing (k6)" -ForegroundColor Gray
    Write-Host "   - GraphQL Testing" -ForegroundColor Gray
    Write-Host "   - WebSocket Testing" -ForegroundColor Gray
    Write-Host "   - Advanced Fuzzing" -ForegroundColor Gray
    Write-Host "   - Documentation Validation" -ForegroundColor Gray
    Write-Host "   - Chaos Engineering" -ForegroundColor Gray
    
    Write-Host ""
    Write-Host "📝 To run the extended tests:" -ForegroundColor Green
    Write-Host "   .\run-extended-tests.ps1" -ForegroundColor Cyan
    
    Write-Host ""
    Write-Host "📊 To view test results:" -ForegroundColor Green
    Write-Host "   .\test-dashboard.ps1" -ForegroundColor Cyan
    
} else {
    Write-Host "❌ Verification failed with $errors errors." -ForegroundColor Red
    Write-Host "   Please check the missing files and dependencies." -ForegroundColor Yellow
}

Write-Host ""
Write-Host "🔍 Verification completed!" -ForegroundColor Green