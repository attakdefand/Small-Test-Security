# Advanced Test Dashboard with Detailed Analytics
# Usage: .\dashboard\advanced-dashboard.ps1 [-days 30] [-format html]

param(
    [int]$days = 7,
    [string]$format = "console",
    [switch]$export
)

Write-Host "🔬 Advanced API Test Dashboard" -ForegroundColor Green
Write-Host "=============================" -ForegroundColor Green

# Get current timestamp
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
Write-Host "Generated: $timestamp" -ForegroundColor Yellow

# Get test results directory
$reportDir = "test-results"

if (!(Test-Path $reportDir)) {
    Write-Host "No test results found. Run tests first with ./run-tests.ps1" -ForegroundColor Red
    exit 1
}

# Calculate date range
$cutOffDate = (Get-Date).AddDays(-$days)
Write-Host "Analyzing test results from the last $days days" -ForegroundColor Cyan

# Get all summary files
$summaries = Get-ChildItem -Path $reportDir -Recurse -Filter "summary_*.txt" | 
    Where-Object { $_.CreationTime -gt $cutOffDate } |
    Sort-Object CreationTime -Descending

if ($summaries.Count -eq 0) {
    Write-Host "No test results found in the specified time range." -ForegroundColor Yellow
    exit
}

Write-Host "Found $($summaries.Count) test runs" -ForegroundColor Cyan

# Initialize statistics
$totalRuns = $summaries.Count
$passedRuns = 0
$failedRuns = 0
$totalTestsPassed = 0
$totalTestsFailed = 0
$performanceData = @()

# Process each summary
foreach ($summary in $summaries) {
    $content = Get-Content $summary.FullName
    
    # Extract test results
    $passed = 0
    $failed = 0
    
    foreach ($line in $content) {
        if ($line -match "Tests Passed: (\d+)") {
            $passed = [int]$matches[1]
            $totalTestsPassed += $passed
        }
        elseif ($line -match "Tests Failed: (\d+)") {
            $failed = [int]$matches[1]
            $totalTestsFailed += $failed
        }
    }
    
    # Update run statistics
    if ($failed -gt 0) {
        $failedRuns++
    } else {
        $passedRuns++
    }
    
    # Collect performance data
    $performanceData += [PSCustomObject]@{
        Timestamp = $summary.CreationTime
        Passed = $passed
        Failed = $failed
        FileName = $summary.Name
    }
}

# Display overall statistics
Write-Host ""
Write-Host "📊 Overall Statistics:" -ForegroundColor Green
Write-Host "=====================" -ForegroundColor Green
Write-Host "Total Test Runs: $totalRuns" -ForegroundColor White
Write-Host "Successful Runs: $passedRuns" -ForegroundColor Green
Write-Host "Failed Runs: $failedRuns" -ForegroundColor Red
Write-Host "Success Rate: $([math]::Round(($passedRuns/$totalRuns)*100, 2))%" -ForegroundColor Cyan

Write-Host ""
Write-Host "📈 Test Results:" -ForegroundColor Green
Write-Host "===============" -ForegroundColor Green
Write-Host "Total Tests Passed: $totalTestsPassed" -ForegroundColor Green
Write-Host "Total Tests Failed: $totalTestsFailed" -ForegroundColor Red
Write-Host "Total Tests Executed: $($totalTestsPassed + $totalTestsFailed)" -ForegroundColor White

# Display recent test runs
Write-Host ""
Write-Host "🕒 Recent Test Runs:" -ForegroundColor Green
Write-Host "===================" -ForegroundColor Green

$recentSummaries = $summaries | Select-Object -First 10
foreach ($summary in $recentSummaries) {
    $content = Get-Content $summary.FullName
    Write-Host ""
    Write-Host "Test Run: $($summary.CreationTime)" -ForegroundColor Yellow
    Write-Host "File: $($summary.Name)" -ForegroundColor Gray
    Write-Host "----------------------------------------"
    
    foreach ($line in $content) {
        if ($line -match "Tests Failed: [1-9]") {
            Write-Host $line -ForegroundColor Red
        } elseif ($line -match "Tests Passed: \d+") {
            Write-Host $line -ForegroundColor Green
        } elseif ($line -match "Tests Failed: 0") {
            Write-Host $line -ForegroundColor Green
        } else {
            Write-Host $line
        }
    }
}

# Performance trend analysis
Write-Host ""
Write-Host "📈 Performance Trend (Last 10 Runs):" -ForegroundColor Green
Write-Host "===================================" -ForegroundColor Green

# Show trend using simple ASCII chart
if ($performanceData.Count -gt 0) {
    $maxPassed = ($performanceData | Measure-Object -Property Passed -Maximum).Maximum
    $maxFailed = ($performanceData | Measure-Object -Property Failed -Maximum).Maximum
    $maxScale = [math]::Max($maxPassed, $maxFailed)
    
    if ($maxScale -eq 0) { $maxScale = 1 }
    
    Write-Host "Date/Time                Passed  Failed  Chart" -ForegroundColor Cyan
    Write-Host "---------------------------------------------" -ForegroundColor Cyan
    
    $recentData = $performanceData | Select-Object -First 10
    foreach ($data in $recentData) {
        $passedBars = [math]::Round(($data.Passed / $maxScale) * 20)
        $failedBars = [math]::Round(($data.Failed / $maxScale) * 20)
        
        $passedBar = "█" * $passedBars
        $failedBar = "▓" * $failedBars
        
        $statusColor = if ($data.Failed -gt 0) { "Red" } else { "Green" }
        
        Write-Host ("{0,-20} {1,6}  {2,6}  " -f $data.Timestamp.ToString("MM/dd HH:mm"), $data.Passed, $data.Failed) -NoNewline -ForegroundColor $statusColor
        Write-Host $passedBar -NoNewline -ForegroundColor Green
        Write-Host $failedBar -ForegroundColor Red
    }
    
    Write-Host ""
    Write-Host "Legend: █ Passed Tests  ▓ Failed Tests" -ForegroundColor Gray
}

# Export data if requested
if ($export) {
    $exportFile = "test-dashboard-export-$(Get-Date -Format 'yyyyMMdd-HHmmss').csv"
    $performanceData | Export-Csv -Path $exportFile -NoTypeInformation
    Write-Host ""
    Write-Host "💾 Data exported to: $exportFile" -ForegroundColor Cyan
}

Write-Host ""
Write-Host "✨ Dashboard generation completed!" -ForegroundColor Green

# Recommendations based on data
Write-Host ""
Write-Host "💡 Recommendations:" -ForegroundColor Yellow
Write-Host "==================" -ForegroundColor Yellow

if ($failedRuns -gt 0) {
    $failureRate = [math]::Round(($failedRuns/$totalRuns)*100, 2)
    if ($failureRate -gt 10) {
        Write-Host "⚠️  High failure rate detected ($failureRate%). Investigate failed tests." -ForegroundColor Red
    } else {
        Write-Host "✅ Acceptable failure rate ($failureRate%)." -ForegroundColor Green
    }
}

if ($totalRuns -gt 5) {
    # Calculate trend
    $recentRuns = $performanceData | Select-Object -First 5
    $olderRuns = $performanceData | Select-Object -Last 5
    
    $recentAvg = ($recentRuns | Measure-Object -Property Passed -Average).Average
    $olderAvg = ($olderRuns | Measure-Object -Property Passed -Average).Average
    
    if ($recentAvg -gt $olderAvg) {
        Write-Host "📈 Test performance is improving over time." -ForegroundColor Green
    } elseif ($recentAvg -lt $olderAvg) {
        Write-Host "📉 Test performance has declined recently. Consider investigation." -ForegroundColor Yellow
    } else {
        Write-Host "➡️  Test performance is stable." -ForegroundColor Cyan
    }
}

Write-Host ""
Write-Host "🔧 Quick Actions:" -ForegroundColor Yellow
Write-Host "================" -ForegroundColor Yellow
Write-Host "Run tests: .\run-tests.ps1" -ForegroundColor Gray
Write-Host "Run extended tests: .\run-extended-tests.ps1" -ForegroundColor Gray
Write-Host "View this dashboard again: .\dashboard\advanced-dashboard.ps1" -ForegroundColor Gray