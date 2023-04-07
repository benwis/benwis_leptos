import { sleep, check } from 'k6'
import http from 'k6/http'

export const options = {
    thresholds: { http_req_duration: ['p(95)<2000'] },
    scenarios: {
        Scenario_1: {
            executor: 'ramping-arrival-rate',
            gracefulStop: '30s',
            stages: [
                { target: 100, duration: '30s' },
                { target: 200, duration: '30s' },
                { target: 300, duration: '30s' },
                { target: 400, duration: '30s' },
            ],
            preAllocatedVUs: 20,
            startRate: 0,
            timeUnit: '1s',
            maxVUs: 20,
            exec: 'load_home',
        },
    },
}

export function load_home() {
    let response

    // HomePage
    response = http.get('https://benwis-leptos.fly.dev')
    check(response, { 'status equals 200': response => response.status.toString() === '200' })

    // Automatically added sleep
    sleep(1)
}
