use std::net::TcpStream;
use std::time::Instant;

use serde::Serialize;
use tauri::{command, generate_handler, Builder};

#[derive(Clone, Serialize)]
struct ProbeResult {
  host: String,
  reachable: bool,
  latency_ms: Option<u64>,
  details: String,
}

#[command]
fn get_status() -> String {
  "ready".to_string()
}

#[command]
fn probe_host(host: String) -> Result<ProbeResult, String> {
  let host = host.trim();
  if host.is_empty() {
    return Err("Host cannot be empty.".to_string());
  }

  let start = Instant::now();

  let address = format!("{}:80", host);
  match TcpStream::connect(&address) {
    Ok(_) => Ok(ProbeResult {
      host: host.to_string(),
      reachable: true,
      latency_ms: Some(start.elapsed().as_millis() as u64),
      details: format!("TCP connection to {} succeeded.", address),
    }),
    Err(error) => Ok(ProbeResult {
      host: host.to_string(),
      reachable: false,
      latency_ms: Some(start.elapsed().as_millis() as u64),
      details: error.to_string(),
    }),
  }
}

pub fn run() {
  Builder::default()
    .invoke_handler(generate_handler![get_status, probe_host])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
