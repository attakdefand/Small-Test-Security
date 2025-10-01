import http from 'k6/http';
import { check, sleep, group } from 'k6';
import { Trend, Counter } from 'k6/metrics';

// Custom metrics
const loginTrend = new Trend('login_duration');
const tradeTrend = new Trend('trade_duration');
const balanceRequests = new Counter('balance_requests');

export let options = {
  stages: [
    { duration: '30s', target: 10 }, // Ramp up
    { duration: '1m', target: 20 },  // Stay at 20 users
    { duration: '30s', target: 0 },  // Ramp down
  ],
  thresholds: {
    http_req_duration: ['p(95)<500'], // 95% of requests should be below 500ms
    http_req_failed: ['rate<0.01'],   // Error rate should be less than 1%
  },
};

const BASE_URL = __ENV.BASE_URL || 'http://localhost:8080';
const USER_TOKEN = __ENV.USER_TOKEN || 'your-jwt-token';

export default function () {
  // Scenario: User login, check balance, place trade, logout
  group('User Trading Flow', function () {
    // 1. Check health endpoint
    const healthStart = new Date().getTime();
    const healthRes = http.get(`${BASE_URL}/health`);
    const healthDuration = new Date().getTime() - healthStart;
    check(healthRes, {
      'health status is 200': (r) => r.status === 200,
      'health response time < 200ms': () => healthDuration < 200,
    });
    
    // 2. Get user profile
    const profileStart = new Date().getTime();
    const profileRes = http.get(`${BASE_URL}/api/v1/me`, {
      headers: {
        'Authorization': `Bearer ${USER_TOKEN}`,
      },
    });
    const profileDuration = new Date().getTime() - profileStart;
    loginTrend.add(profileDuration);
    check(profileRes, {
      'profile status is 200': (r) => r.status === 200,
    });
    
    // 3. Check balance (simulate multiple requests)
    for (let i = 0; i < 3; i++) {
      const balanceRes = http.get(`${BASE_URL}/api/v1/balance`, {
        headers: {
          'Authorization': `Bearer ${USER_TOKEN}`,
        },
      });
      balanceRequests.add(1);
      check(balanceRes, {
        'balance status is 200': (r) => r.status === 200,
      });
      sleep(0.1); // Small delay between requests
    }
    
    // 4. Place trades (if withdrawals feature is enabled)
    if (__ENV.FEATURE_WITHDRAWALS === 'on') {
      const tradeStart = new Date().getTime();
      const tradePayload = JSON.stringify({
        symbol: 'BTC-USD',
        amount: Math.random() * 100,
        type: 'market',
      });
      
      const tradeRes = http.post(`${BASE_URL}/api/v1/trading`, tradePayload, {
        headers: {
          'Authorization': `Bearer ${USER_TOKEN}`,
          'Content-Type': 'application/json',
        },
      });
      const tradeDuration = new Date().getTime() - tradeStart;
      tradeTrend.add(tradeDuration);
      check(tradeRes, {
        'trade status is 200 or 201': (r) => r.status === 200 || r.status === 201,
      });
    }
  });
  
  sleep(1);
}