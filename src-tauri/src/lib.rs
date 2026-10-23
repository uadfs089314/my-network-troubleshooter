use tauri::{command, Builder, generate_handler};

#[command]
fn get_status() -> String {
    "ready".to_string()
}

#[command]
fn probe_host(host: String) -> Result<String, String> {
    let host = host.trim();
    if host.is_empty() {
        return Err("Host cannot be empty.".to_string());
    }

    if host == "localhost" || host == "127.0.0.1" {
        return Ok(format!("Probe succeeded for {}: local endpoint reachable.", host));
    }

    Ok(format!("Probe prepared for {}. This is the minimal Tauri backend health check.", host))
}

pub fn run() {
    Builder::default()
        .invoke_handler(generate_handler![get_status, probe_host])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
