use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::process::Command;
use std::time::{Duration, Instant};

#[derive(Debug, Serialize, Deserialize)]
struct PingResult {
  host: String,
  success: bool,
  latency_ms: Option<u64>,
  output: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct DNSResult {
  host: String,
  success: bool,
  addresses: Vec<String>,
  output: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HttpsResult {
  host: String,
  success: bool,
  status_code: Option<u16>,
  output: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct PortCheckResult {
  host: String,
  port: u16,
  success: bool,
  latency_ms: Option<u64>,
  output: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TracerouteResult {
  host: String,
  success: bool,
  hops: Vec<TracerouteHop>,
  output: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TracerouteHop {
  index: usize,
  hostname: Option<String>,
  ip: Option<String>,
  latency_ms: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConnectivityTest {
  ping: Option<PingResult>,
  dns: Option<DNSResult>,
  https: Option<HttpsResult>,
  traceroute: Option<TracerouteResult>,
  port_check: Option<PortCheckResult>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConnectivityReport {
  host: String,
  tests: ConnectivityTest,
  status: String,
  issues: Vec<String>,
  recommended_action: String,
  confidence_score: f64,
  confidence_level: String,
  latency_ms: Option<f64>,
  throughput_mbps: Option<f64>,
  is_connected: bool,
  summary: String,
}

#[tauri::command]
async fn ping_host(host: String) -> Result<PingResult, String> {
  let start = Instant::now();

  let output = if cfg!(target_os = "windows") {
    let output = Command::new("ping")
      .args(["-n", "1", "-w", "2000", &host])
      .output()
      .map_err(|e| e.to_string())?;
    String::from_utf8_lossy(&output.stdout).to_string()
  } else {
    let output = Command::new("ping")
      .args(["-c", "1", "-W", "2", &host])
      .output()
      .map_err(|e| e.to_string())?;
    String::from_utf8_lossy(&output.stdout).to_string()
  };

  let latency = start.elapsed().as_millis() as u64;
  let success = output.contains("ttl=") || output.contains("time=") || output.contains("Reply from");

  Ok(PingResult {
    host,
    success,
    latency_ms: Some(latency),
    output,
  })
}

#[tauri::command]
async fn resolve_dns(host: String) -> Result<DNSResult, String> {
  let output = Command::new("nslookup")
    .arg(&host)
    .output()
    .map_err(|e| e.to_string())?;

  let output_str = String::from_utf8_lossy(&output.stdout).to_string();
  let mut addresses = Vec::new();

  for line in output_str.lines() {
    if line.contains("Address:") || line.contains("address =") {
      if let Some(addr) = line.split_whitespace().last() {
        if addr.parse::<IpAddr>().is_ok() {
          addresses.push(addr.to_string());
        }
      }
    }
  }

  let success = !addresses.is_empty();

  Ok(DNSResult {
    host,
    success,
    addresses,
    output: output_str,
  })
}

#[tauri::command]
async fn check_https(host: String) -> Result<HttpsResult, String> {
  let target = if host.starts_with("http://") || host.starts_with("https://") {
    host.clone()
  } else {
    format!("https://{}", host)
  };

  let output = Command::new("curl")
    .args(["-I", "-sS", "--max-time", "5", &target])
    .output()
    .map_err(|e| e.to_string())?;

  let stdout = String::from_utf8_lossy(&output.stdout).to_string();
  let stderr = String::from_utf8_lossy(&output.stderr).to_string();
  let combined = format!("{}{}", stdout, stderr);

  let status_code = stdout
    .lines()
    .find_map(|line| {
      let cleaned = line.trim();
      let pieces: Vec<&str> = cleaned.split_whitespace().collect();
      if pieces.len() >= 2 && pieces[0].starts_with("HTTP/") {
        pieces[1].parse::<u16>().ok()
      } else {
        None
      }
    })
    .or_else(|| {
      if output.status.success() {
        Some(200)
      } else {
        None
      }
    });

  let success = output.status.success() && status_code.is_some();

  Ok(HttpsResult {
    host,
    success,
    status_code,
    output: combined,
  })
}

#[tauri::command]
async fn check_port(host: String, port: u16) -> Result<PortCheckResult, String> {
  let start = Instant::now();
  let timeout = Duration::from_secs(2);

  let socket_addr = (
    host.as_str(),
    port,
  )
    .to_socket_addrs()
    .map_err(|e| e.to_string())?
    .next()
    .ok_or_else(|| format!("Could not resolve {}:{} to a socket address", host, port))?;

  let result = TcpStream::connect_timeout(&socket_addr, timeout);

  match result {
    Ok(_) => {
      let latency = start.elapsed().as_millis() as u64;
      Ok(PortCheckResult {
        host,
        port,
        success: true,
        latency_ms: Some(latency),
        output: format!("Port {} is open on {}", port, host),
      })
    }
    Err(e) => Ok(PortCheckResult {
      host,
      port,
      success: false,
      latency_ms: None,
      output: format!("Port {} is closed or unreachable on {}: {}", port, host, e),
    }),
  }
}

#[tauri::command]
async fn trace_route(host: String) -> Result<TracerouteResult, String> {
  let cmd = if cfg!(target_os = "windows") {
    "tracert"
  } else {
    "traceroute"
  };

  let mut args: Vec<String> = vec![host.clone()];
  if !cfg!(target_os = "windows") {
    args.insert(0, "-n".to_string());
    args.insert(1, "-q".to_string());
    args.insert(2, "1".to_string());
    args.insert(3, "-w".to_string());
    args.insert(4, "1".to_string());
    args.insert(5, "-m".to_string());
    args.insert(6, "15".to_string());
  } else {
    args.insert(0, "-h".to_string());
    args.insert(1, "15".to_string());
    args.insert(2, "-w".to_string());
    args.insert(3, "1000".to_string());
  }

  let output = Command::new(cmd)
    .args(&args)
    .output()
    .map_err(|e| e.to_string())?;

  let output_str = String::from_utf8_lossy(&output.stdout).to_string();
  let mut hops = Vec::new();
  let mut success = false;

  for line in output_str.lines() {
    if line.contains("ms") || line.contains("ms ") {
      let parts: Vec<&str> = line.split_whitespace().collect();
      if parts.len() >= 4 {
        let index = parts[0].parse::<usize>().unwrap_or(0);
        let mut ip: Option<String> = None;
        let mut hostname: Option<String> = None;
        let mut latency: Option<f64> = None;

        for (i, part) in parts.iter().enumerate() {
          if part.contains(".") && part.parse::<IpAddr>().is_ok() {
            ip = Some(part.to_string());
          }
          if i > 0 && i < parts.len() - 3 && !part.contains("*") && !part.parse::<f64>().is_ok() {
            hostname = Some(part.to_string());
          }
          if let Ok(lat) = part.replace("ms", "").trim().parse::<f64>() {
            latency = Some(lat);
          }
        }

        if index > 0 {
          hops.push(TracerouteHop {
            index,
            hostname: hostname.clone(),
            ip: ip.clone(),
            latency_ms: latency,
          });
          success = true;
        }
      }
    }
  }

  Ok(TracerouteResult {
    host,
    success,
    hops,
    output: output_str,
  })
}

#[tauri::command]
async fn run_connectivity_test(host: String) -> Result<ConnectivityReport, String> {
  let ping_result = ping_host(host.clone()).await;
  let dns_result = resolve_dns(host.clone()).await;
  let https_result = check_https(host.clone()).await;
  let trace_result = trace_route(host.clone()).await;
  let port_result = check_port(host.clone(), 80).await;

  let mut confidence_score = 0.0f64;
  let mut latency_total = 0.0f64;
  let mut latency_count = 0usize;
  let mut throughput: Option<f64> = None;
  let mut is_connected = false;
  let mut issues = Vec::new();

  if let Ok(ref ping) = ping_result {
    if ping.success {
      confidence_score += 35.0;
      is_connected = true;
      if let Some(lat) = ping.latency_ms {
        latency_total += lat as f64;
        latency_count += 1;
      }
    } else {
      issues.push("Ping is failing".to_string());
    }
  } else {
    issues.push("Ping could not be completed".to_string());
  }

  if let Ok(ref dns) = dns_result {
    if dns.success {
      confidence_score += 25.0;
    } else {
      issues.push("DNS resolution is failing".to_string());
    }
  } else {
    issues.push("DNS could not be checked".to_string());
  }

  if let Ok(ref https) = https_result {
    if https.success {
      confidence_score += 20.0;
    } else {
      issues.push("HTTPS/TLS check is failing".to_string());
    }
  } else {
    issues.push("HTTPS/TLS check could not run".to_string());
  }

  if let Ok(ref trace) = trace_result {
    if trace.success && !trace.hops.is_empty() {
      confidence_score += 20.0;
      for hop in &trace.hops {
        if let Some(lat) = hop.latency_ms {
          latency_total += lat;
          latency_count += 1;
        }
      }
    } else {
      issues.push("Route quality is unstable".to_string());
    }
  }

  if let Ok(ref port) = port_result {
    if port.success {
      confidence_score += 5.0;
    } else {
      issues.push("Port 80 is blocked or unreachable".to_string());
    }
  }

  let avg_latency = if latency_count > 0 {
    Some(latency_total / latency_count as f64)
  } else {
    None
  };

  let status = if issues.is_empty() {
    "healthy".to_string()
  } else if issues.len() <= 2 {
    "degraded".to_string()
  } else {
    "issue".to_string()
  };

  let recommended_action = if issues.is_empty() {
    "No action needed. Keep monitoring for latency or service regressions.".to_string()
  } else if issues.iter().any(|issue| issue.contains("DNS")) {
    "Start by checking the local resolver, gateway, and whether the ISP is blocking DNS.".to_string()
  } else if issues.iter().any(|issue| issue.contains("HTTPS")) {
    "Check certificate validity, proxy settings, and whether the destination is blocking secure connections.".to_string()
  } else if issues.iter().any(|issue| issue.contains("Ping")) {
    "First check the modem or router, then validate local connectivity and power cycle the network path.".to_string()
  } else {
    "Check the local gateway, route, and any firewall or ISP restrictions before testing the destination again.".to_string()
  };

  let (confidence_level, summary) = if confidence_score >= 80.0 {
    ("Excellent".to_string(), "Internet is healthy and the path looks stable.".to_string())
  } else if confidence_score >= 60.0 {
    ("Good".to_string(), "Most checks passed, but keep an eye on latency or service degradation.".to_string())
  } else if confidence_score >= 40.0 {
    ("Fair".to_string(), "Some services are failing or performance is degraded. Investigate the gateway or DNS path first.".to_string())
  } else {
    ("Poor".to_string(), "Connectivity problems are likely. Start with local network and DNS diagnostics before chasing app issues.".to_string())
  };

  Ok(ConnectivityReport {
    host,
    tests: ConnectivityTest {
      ping: ping_result.ok(),
      dns: dns_result.ok(),
      https: https_result.ok(),
      traceroute: trace_result.ok(),
      port_check: port_result.ok(),
    },
    status,
    issues,
    recommended_action,
    confidence_score,
    confidence_level,
    latency_ms: avg_latency,
    throughput_mbps: throughput,
    is_connected,
    summary,
  })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      ping_host,
      resolve_dns,
      check_https,
      check_port,
      trace_route,
      run_connectivity_test
    ])
    .run(tauri::generate_context!())
    .expect("error while building tauri application");
}

