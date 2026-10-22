* {
  box-sizing: border-box;
}

:root {
  color-scheme: dark;
  --bg: #0f172a;
  --panel: #111827;
  --panel-alt: #1f2937;
  --accent: #38bdf8;
  --text: #e5e7eb;
  --muted: #94a3b8;
  --border: rgba(148, 163, 184, 0.2);
  --healthy: #34d399;
  --degraded: #fbbf24;
  --issue: #f87171;
  --neutral: #60a5fa;
}

body {
  margin: 0;
  min-height: 100vh;
  background: linear-gradient(180deg, var(--bg), #020817);
  color: var(--text);
  font-family: Inter, "Segoe UI", sans-serif;
}

.app-shell {
  width: min(900px, calc(100vw - 32px));
  margin: 48px auto;
}

.topbar {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 24px;
  gap: 16px;
}

.eyebrow {
  margin: 0 0 8px;
  font-size: 0.72rem;
  text-transform: uppercase;
  letter-spacing: 0.12em;
  color: var(--muted);
}

h1, h2, h3, p {
  margin-top: 0;
}

h1 {
  margin-bottom: 0;
}

.monitor-header {
  display: flex;
  align-items: end;
  justify-content: space-between;
  gap: 16px;
}

.host-field {
  display: flex;
  flex-direction: column;
  gap: 8px;
  min-width: 180px;
}

.host-field span {
  color: var(--muted);
  font-size: 0.8rem;
}

.host-field input {
  width: 100%;
  border: 1px solid var(--border);
  border-radius: 10px;
  background: rgba(15, 23, 42, 0.7);
  color: var(--text);
  padding: 10px 12px;
}

.status-badge {
  display: inline-flex;
  align-items: center;
  border-radius: 999px;
  padding: 7px 12px;
  border: 1px solid var(--border);
  font-size: 0.8rem;
  font-weight: 700;
  text-transform: uppercase;
}

.status-badge.healthy {
  background: rgba(52, 211, 153, 0.12);
  color: var(--healthy);
  border-color: rgba(52, 211, 153, 0.25);
}

.status-badge.degraded {
  background: rgba(251, 191, 36, 0.12);
  color: var(--degraded);
  border-color: rgba(251, 191, 36, 0.25);
}

.status-badge.issue {
  background: rgba(248, 113, 113, 0.12);
  color: var(--issue);
  border-color: rgba(248, 113, 113, 0.25);
}

.status-badge.neutral {
  background: rgba(96, 165, 250, 0.12);
  color: var(--neutral);
  border-color: rgba(96, 165, 250, 0.25);
}

.card {
  background: rgba(17, 24, 39, 0.9);
  border: 1px solid var(--border);
  border-radius: 16px;
  padding: 20px;
  margin-bottom: 20px;
  box-shadow: 0 10px 25px rgba(15, 23, 42, 0.25);
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 16px;
  margin-bottom: 20px;
}

.stat-card {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 6px;
  min-height: 120px;
}

.label {
  color: var(--muted);
  font-size: 0.78rem;
  text-transform: uppercase;
  letter-spacing: 0.08em;
}

.stat-card strong {
  font-size: clamp(1.6rem, 3vw, 2.2rem);
  line-height: 1;
}

.stat-card small {
  color: var(--muted);
}

.output-card pre {
  margin: 0;
  padding: 14px;
  min-height: 110px;
  background: var(--panel-alt);
  border: 1px solid var(--border);
  border-radius: 10px;
  white-space: pre-wrap;
  color: var(--text);
  line-height: 1.5;
}

@media (max-width: 720px) {
  .monitor-header,
  .topbar {
    flex-direction: column;
    align-items: flex-start;
  }

  .stats-grid {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}

@media (max-width: 420px) {
  .stats-grid {
    grid-template-columns: 1fr;
  }
}
