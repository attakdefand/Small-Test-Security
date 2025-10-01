# Generate Advanced HTML Test Report
# Usage: .\reporting\generate-html-report.ps1 [-days 30] [-output report.html]

param(
    [int]$days = 7,
    [string]$output = "test-report.html"
)

Write-Host "📊 Generating Advanced HTML Test Report" -ForegroundColor Green
Write-Host "=====================================" -ForegroundColor Green

# Get current timestamp
$timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"

# Get test results directory
$reportDir = "test-results"

if (!(Test-Path $reportDir)) {
    Write-Host "No test results found. Run tests first with ./run-tests.ps1" -ForegroundColor Red
    exit 1
}

# Calculate date range
$cutOffDate = (Get-Date).AddDays(-$days)

# Get all summary files
$summaries = Get-ChildItem -Path $reportDir -Recurse -Filter "summary_*.txt" | 
    Where-Object { $_.CreationTime -gt $cutOffDate } |
    Sort-Object CreationTime -Descending

if ($summaries.Count -eq 0) {
    Write-Host "No test results found in the specified time range." -ForegroundColor Yellow
    exit
}

Write-Host "Processing $($summaries.Count) test runs..." -ForegroundColor Cyan

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
    $ignored = 0
    
    foreach ($line in $content) {
        if ($line -match "Tests Passed: (\d+)") {
            $passed = [int]$matches[1]
            $totalTestsPassed += $passed
        }
        elseif ($line -match "Tests Failed: (\d+)") {
            $failed = [int]$matches[1]
            $totalTestsFailed += $failed
        }
        elseif ($line -match "Tests Ignored: (\d+)") {
            $ignored = [int]$matches[1]
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
        Ignored = $ignored
        FileName = $summary.Name
    }
}

# Calculate success rate
$successRate = if ($totalRuns -gt 0) { [math]::Round(($passedRuns/$totalRuns)*100, 2) } else { 0 }

# Generate HTML report
$htmlContent = @"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>API Test Report</title>
    <style>
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 0;
            padding: 20px;
            background-color: #f5f5f5;
        }
        .container {
            max-width: 1200px;
            margin: 0 auto;
            background-color: white;
            border-radius: 8px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            padding: 30px;
        }
        header {
            text-align: center;
            margin-bottom: 30px;
            border-bottom: 2px solid #e0e0e0;
            padding-bottom: 20px;
        }
        h1 {
            color: #2c3e50;
            margin-bottom: 10px;
        }
        .subtitle {
            color: #7f8c8d;
            font-size: 1.1em;
        }
        .stats-grid {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        .stat-card {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            border-radius: 8px;
            text-align: center;
            box-shadow: 0 4px 6px rgba(0,0,0,0.1);
        }
        .stat-card h3 {
            margin: 0 0 10px 0;
            font-size: 1em;
            font-weight: normal;
        }
        .stat-card .value {
            font-size: 2em;
            font-weight: bold;
        }
        .chart-container {
            margin: 30px 0;
            padding: 20px;
            background-color: #f8f9fa;
            border-radius: 8px;
        }
        .chart-title {
            text-align: center;
            margin-bottom: 20px;
            color: #2c3e50;
        }
        .recent-tests {
            margin: 30px 0;
        }
        .recent-tests h2 {
            color: #2c3e50;
            border-bottom: 2px solid #e0e0e0;
            padding-bottom: 10px;
        }
        table {
            width: 100%;
            border-collapse: collapse;
            margin-top: 20px;
        }
        th, td {
            padding: 12px;
            text-align: left;
            border-bottom: 1px solid #ddd;
        }
        th {
            background-color: #f2f2f2;
            font-weight: bold;
        }
        tr:hover {
            background-color: #f5f5f5;
        }
        .passed {
            color: #27ae60;
            font-weight: bold;
        }
        .failed {
            color: #e74c3c;
            font-weight: bold;
        }
        .footer {
            text-align: center;
            margin-top: 30px;
            color: #7f8c8d;
            font-size: 0.9em;
            border-top: 1px solid #e0e0e0;
            padding-top: 20px;
        }
        .trend-indicator {
            display: inline-block;
            width: 12px;
            height: 12px;
            border-radius: 50%;
            margin-right: 5px;
        }
        .trend-up {
            background-color: #27ae60;
        }
        .trend-down {
            background-color: #e74c3c;
        }
        .trend-stable {
            background-color: #f39c12;
        }
    </style>
</head>
<body>
    <div class="container">
        <header>
            <h1>📊 API Test Report</h1>
            <div class="subtitle">Generated on $timestamp</div>
            <div class="subtitle">Analysis period: Last $days days</div>
        </header>

        <div class="stats-grid">
            <div class="stat-card">
                <h3>Total Test Runs</h3>
                <div class="value">$totalRuns</div>
            </div>
            <div class="stat-card">
                <h3>Success Rate</h3>
                <div class="value">$successRate%</div>
            </div>
            <div class="stat-card">
                <h3>Tests Passed</h3>
                <div class="value">$totalTestsPassed</div>
            </div>
            <div class="stat-card">
                <h3>Tests Failed</h3>
                <div class="value">$totalTestsFailed</div>
            </div>
        </div>

        <div class="chart-container">
            <h2 class="chart-title">📈 Test Results Trend</h2>
            <canvas id="trendChart" width="800" height="400"></canvas>
        </div>

        <div class="recent-tests">
            <h2>🕒 Recent Test Runs</h2>
            <table>
                <thead>
                    <tr>
                        <th>Date/Time</th>
                        <th>Status</th>
                        <th>Passed</th>
                        <th>Failed</th>
                        <th>Ignored</th>
                    </tr>
                </thead>
                <tbody>
"@

# Add recent test runs to the table
$recentData = $performanceData | Select-Object -First 10
foreach ($data in $recentData) {
    $statusClass = if ($data.Failed -gt 0) { "failed" } else { "passed" }
    $statusText = if ($data.Failed -gt 0) { "Failed" } else { "Passed" }
    
    $htmlContent += @"
                    <tr>
                        <td>$($data.Timestamp.ToString("yyyy-MM-dd HH:mm:ss"))</td>
                        <td class="$statusClass">$statusText</td>
                        <td>$($data.Passed)</td>
                        <td>$($data.Failed)</td>
                        <td>$($data.Ignored)</td>
                    </tr>
"@
}

# Close the HTML
$htmlContent += @"
                </tbody>
            </table>
        </div>

        <div class="footer">
            <p>Generated by Advanced API Testing Framework</p>
            <p>This report analyzes test results from the last $days days</p>
        </div>
    </div>

    <script>
        // Simple chart rendering (in a real implementation, you'd use Chart.js or similar)
        document.addEventListener('DOMContentLoaded', function() {
            const canvas = document.getElementById('trendChart');
            const ctx = canvas.getContext('2d');
            
            // Sample data for the chart
            const data = [
                {date: '2023-01-01', passed: 45, failed: 2},
                {date: '2023-01-02', passed: 48, failed: 1},
                {date: '2023-01-03', passed: 42, failed: 3},
                {date: '2023-01-04', passed: 50, failed: 0},
                {date: '2023-01-05', passed: 47, failed: 1}
            ];
            
            // Draw a simple line chart
            ctx.fillStyle = '#27ae60';
            ctx.font = '14px Arial';
            ctx.fillText('Chart visualization would appear here in a full implementation', 50, 50);
        });
    </script>
</body>
</html>
"@

# Write the HTML report
$htmlContent | Out-File -FilePath $output -Encoding UTF8

Write-Host "✅ HTML report generated successfully!" -ForegroundColor Green
Write-Host "📄 Report saved to: $output" -ForegroundColor Cyan
Write-Host "📊 Summary:" -ForegroundColor Yellow
Write-Host "   Total Runs: $totalRuns" -ForegroundColor White
Write-Host "   Success Rate: $successRate%" -ForegroundColor White
Write-Host "   Tests Passed: $totalTestsPassed" -ForegroundColor Green
Write-Host "   Tests Failed: $totalTestsFailed" -ForegroundColor Red

# Open the report in default browser (optional)
if ($env:CI -ne "true") {
    Write-Host "🌐 Opening report in browser..." -ForegroundColor Cyan
    Start-Process $output
}

Write-Host "✨ Report generation completed!" -ForegroundColor Green