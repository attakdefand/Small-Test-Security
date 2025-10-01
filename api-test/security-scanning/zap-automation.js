// This is a Node.js script to automate OWASP ZAP scanning
// Run with: node zap-automation.js

const axios = require('axios');
const fs = require('fs');

// Configuration
const ZAP_API_URL = process.env.ZAP_API_URL || 'http://localhost:8080';
const TARGET_URL = process.env.BASE_URL || 'http://localhost:8080';
const API_KEY = process.env.ZAP_API_KEY || ''; // Set your ZAP API key

const zap = axios.create({
  baseURL: ZAP_API_URL,
  timeout: 30000,
});

async function runSecurityScan() {
  try {
    console.log(`Starting security scan for: ${TARGET_URL}`);
    
    // 1. Access the target URL
    await zap.get(`/JSON/core/action/accessUrl/?url=${encodeURIComponent(TARGET_URL)}&apikey=${API_KEY}`);
    console.log('Target URL accessed');
    
    // 2. Wait for passive scanning
    await new Promise(resolve => setTimeout(resolve, 5000));
    
    // 3. Start active scan
    const scanResponse = await zap.get(`/JSON/ascan/action/scan/?url=${encodeURIComponent(TARGET_URL)}&apikey=${API_KEY}`);
    const scanId = scanResponse.data.scan;
    console.log(`Active scan started with ID: ${scanId}`);
    
    // 4. Wait for scan completion
    let status = 0;
    while (status < 100) {
      const statusResponse = await zap.get(`/JSON/ascan/view/status/?scanId=${scanId}&apikey=${API_KEY}`);
      status = parseInt(statusResponse.data.status);
      console.log(`Scan progress: ${status}%`);
      await new Promise(resolve => setTimeout(resolve, 2000));
    }
    
    console.log('Scan completed');
    
    // 5. Get alerts
    const alertsResponse = await zap.get(`/JSON/core/view/alerts/?baseurl=${encodeURIComponent(TARGET_URL)}&apikey=${API_KEY}`);
    const alerts = alertsResponse.data.alerts;
    
    // 6. Generate report
    const reportResponse = await zap.get(`/OTHER/core/other/htmlreport/?apikey=${API_KEY}`);
    fs.writeFileSync('security-report.html', reportResponse.data);
    console.log('Security report saved as security-report.html');
    
    // 7. Log high-risk alerts
    const highRiskAlerts = alerts.filter(alert => 
      alert.risk === 'High' || alert.risk === 'Medium'
    );
    
    if (highRiskAlerts.length > 0) {
      console.log('\n⚠️  High/Medium Risk Alerts Found:');
      highRiskAlerts.forEach(alert => {
        console.log(`- ${alert.risk} Risk: ${alert.name}`);
        console.log(`  URL: ${alert.url}`);
        console.log(`  Description: ${alert.description.substring(0, 100)}...\n`);
      });
    } else {
      console.log('✅ No high or medium risk alerts found');
    }
    
  } catch (error) {
    console.error('Security scan failed:', error.message);
    process.exit(1);
  }
}

// Run the scan if this script is executed directly
if (require.main === module) {
  runSecurityScan();
}

module.exports = { runSecurityScan };