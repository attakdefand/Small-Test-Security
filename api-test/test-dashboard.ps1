# Simple Test Dashboard
# Shows recent test results

param(
    [int]$days = 7
)

Write-Host "API Test Dashboard" -ForegroundColor Green
Write-Host "==================" -ForegroundColor Green

# Get test results directory
$reportDir = "test-results"

if (!(Test-Path $reportDir)) {
    Write-Host "No test results found. Run tests first with ./run-tests.ps1" -ForegroundColor Yellow
    exit
}

Write-Host "Recent test runs (last $days days):" -ForegroundColor Cyan

# Get recent test summaries
$cutOffDate = (Get-Date).AddDays(-$days)
$summaries = Get-ChildItem -Path $reportDir -Recurse -Filter "summary_*.txt" | 
    Where-Object { $_.CreationTime -gt $cutOffDate } |
    Sort-Object CreationTime -Descending

if ($summaries.Count -eq 0) {
    Write-Host "No recent test results found." -ForegroundColor Yellow
    exit
}

foreach ($summary in $summaries) {
    Write-Host ""
    Write-Host "Test Run: $($summary.CreationTime)" -ForegroundColor Yellow
    Write-Host "File: $($summary.Name)" -ForegroundColor Gray
    Write-Host "----------------------------------------"
    Get-Content $summary.FullName | ForEach-Object {
        if ($_ -match "Tests Passed:|Tests Failed:") {
            if ($_ -match "Failed") {
                Write-Host $_ -ForegroundColor Red
            } else {
                Write-Host $_ -ForegroundColor Green
            }
        } else {
            Write-Host $_
        }
    }
}

Write-Host ""
Write-Host "Total test runs in the last $days days: $($summaries.Count)" -ForegroundColor Cyan