import { invoke } from '@tauri-apps/api/core';

const statusBadge = document.getElementById('status-badge');
const issueSummary = document.getElementById('issue-summary');
const output = document.getElementById('output');
const latencyValue = document.getElementById('latency-value');
const connectedValue = document.getElementById('connected-value');
const dnsValue = document.getElementById('dns-value');
const httpsValue = document.getElementById('https-value');
const hostInput = document.getElementById('host-input');

const DEFAULT_INTERVAL_MS = 15000;

const setBadge = (status = 'neutral') => {
  const normalized = status.toLowerCase();
  statusBadge.className = `status-badge ${normalized}`;
  if (normalized === 'healthy') {
    statusBadge.textContent = 'Healthy';
  } else if (normalized === 'degraded') {
    statusBadge.textContent = 'Degraded';
  } else if (normalized === 'issue') {
    statusBadge.textContent = 'Issue';
  } else {
    statusBadge.textContent = 'Checking';
  }
};

const formatMetric = (value, suffix = '') => {
  if (value === null || value === undefined || Number.isNaN(value)) {
    return '--';
  }
  return `${value}${suffix}`;
};

const explainIssues = (report = {}) => {
  const { tests = {} } = report;
  const messages = [];

  if (!tests.ping?.success) messages.push('Ping is failing');
  if (!tests.dns?.success) messages.push('DNS is failing');
  if (!tests.https?.success) messages.push('HTTPS/TLS is failing');
  if (!tests.traceroute?.success) messages.push('Route is unstable');

  if (messages.length === 0) {
    return 'No major issues detected. Keep monitoring for latency or service degradation.';
  }

  return `Priority: ${messages.join(' • ')}`;
};

const renderReport = (report = {}) => {
  const summary = report.summary || 'Monitoring your connection and looking for service issues.';
  const status = report.status || 'neutral';

  setBadge(status);
  issueSummary.textContent = summary;

  output.textContent = [
    `Status: ${status}`,
    `Recommended action: ${report.recommended_action || 'Keep monitoring and check the local gateway first.'}`,
    `Issue summary: ${explainIssues(report)}`,
    `Latency: ${formatMetric(report.latency_ms, ' ms')}`,
    `Connected: ${report.is_connected ? 'Yes' : 'No'}`,
    `DNS: ${report.tests?.dns?.success ? 'OK' : 'Failed'}`,
    `HTTPS: ${report.tests?.https?.success ? 'OK' : 'Failed'}`,
  ].join('\n');

  latencyValue.textContent = formatMetric(report.latency_ms, '');
  connectedValue.textContent = report.is_connected ? 'Yes' : 'No';
  dnsValue.textContent = report.tests?.dns?.success ? 'OK' : 'Failed';
  httpsValue.textContent = report.tests?.https?.success ? 'OK' : 'Failed';
};

const runMonitor = async () => {
  const host = hostInput.value.trim() || 'example.com';

  try {
    statusBadge.textContent = 'Checking';
    statusBadge.className = 'status-badge neutral';
    const report = await invoke('run_connectivity_test', { host });
    renderReport(report);
  } catch (error) {
    issueSummary.textContent = 'Monitoring failed to update. Check the app and backend configuration.';
    output.textContent = `Unable to run connectivity checks:\n${String(error)}`;
    setBadge('issue');
  }
};

hostInput.addEventListener('keydown', (event) => {
  if (event.key === 'Enter') {
    runMonitor();
  }
});

window.addEventListener('DOMContentLoaded', () => {
  runMonitor();
  setInterval(runMonitor, DEFAULT_INTERVAL_MS);
});
