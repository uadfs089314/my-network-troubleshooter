<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>Internet Troubleshooter</title>
    <link rel="stylesheet" href="./styles.css" />
  </head>
  <body>
    <main class="app-shell">
      <header class="topbar">
        <div>
          <p class="eyebrow">Connection monitor</p>
          <h1>Internet Troubleshooter</h1>
        </div>
        <span id="status-badge" class="status-badge neutral">Checking</span>
      </header>

      <section class="card intro-card">
        <div class="monitor-header">
          <div>
            <h2>Connection status</h2>
            <p id="issue-summary">Monitoring connectivity and looking for service issues.</p>
          </div>
          <label class="host-field">
            <span>Target</span>
            <input id="host-input" type="text" value="example.com" />
          </label>
        </div>
      </section>

      <section class="stats-grid">
        <article class="card stat-card">
          <span class="label">Latency</span>
          <strong id="latency-value">--</strong>
          <small>ms</small>
        </article>
        <article class="card stat-card">
          <span class="label">Connected</span>
          <strong id="connected-value">--</strong>
        </article>
        <article class="card stat-card">
          <span class="label">DNS</span>
          <strong id="dns-value">--</strong>
        </article>
        <article class="card stat-card">
          <span class="label">HTTPS</span>
          <strong id="https-value">--</strong>
        </article>
      </section>

      <section class="card output-card">
        <div class="section-header">
          <h3>What to look at first</h3>
        </div>
        <pre id="output">Monitoring your connection and surface-level issues.</pre>
      </section>
    </main>

    <script type="module" src="./main.js"></script>
  </body>
</html>
