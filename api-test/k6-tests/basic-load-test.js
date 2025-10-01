import http from 'k6/http';
import { check, sleep } from 'k6';

export let options = {
  vus: 10,
  duration: '30s',
};

export default function () {
  // Load BASE_URL from environment
  const baseUrl = __ENV.BASE_URL || 'http://localhost:8080';
  const healthPath = __ENV.HEALTH_PATH || '/health';
  
  const res = http.get(`${baseUrl}${healthPath}`);
  check(res, {
    'status is 200': (r) => r.status === 200,
    'response time < 500ms': (r) => r.timings.duration < 500,
  });
  
  sleep(1);
}