README.md
# Internet Troubleshooter

A lightweight desktop monitor for internet and connectivity health. The goal is not to replicate a full network lab; it is to stay quiet in normal operation and surface the important issue, the likely cause, and the first place to look when something breaks.

## Product intent

This app is designed for internet troubleshooting in the background:

- monitor latency, reachability, and service availability
- detect issues early instead of requiring a manual button press
- surface the root problem first: connectivity, DNS, HTTPS/TLS, or routing
- recommend the first place to investigate, such as the local gateway, DNS, or browser/proxy configuration

## Current scope

The current implementation checks:

- ping reachability
- DNS resolution
- HTTPS/TLS availability
- traceroute quality
- port reachability (HTTP/80)

The UI is intentionally simple: it shows health status and issue-focused summaries instead of dumping raw diagnostics unless the user needs more detail.

## Local development

```bash
npm install
npm run dev
```

## Planned next steps

1. Add a proper background monitoring loop with configurable intervals.
2. Add notification severity levels for healthy, degraded, and broken connectivity.
3. Add a richer history panel showing recent issues and uptime/downtime windows.
4. Add better HTTPS/TLS validation and site-specific checks.
5. Add user-configurable targets to monitor multiple services and websites.

## Core product principles

- quiet by default
- issue-first summaries
- actionable recommendations
- minimal noise unless there is a problem
- transparent evidence for each diagnosis

This project is intentionally opinionated: if nothing is wrong, the app should stay out of the way, and when something is wrong, the app should tell the user what to inspect first.

Suffix












































































































































