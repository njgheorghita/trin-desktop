use crate::types::config::TrinConfig;
use crate::types::node::NodeHistoryLog;
use crate::utils::node_rpc::check_trin_status;
use crate::AppData;
use log::{info, warn};
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use sysinfo::{Pid, System};
use tauri::Emitter;
use tauri::Manager;
use tauri::State;
use tauri_plugin_shell::process::CommandEvent;
use tauri_plugin_shell::ShellExt;

#[tauri::command]
pub async fn launch_trin<'l>(
    app: tauri::AppHandle,
    app_data: State<'l, Mutex<AppData>>,
    trin_config: TrinConfig,
) -> Result<String, String> {
    info!("starting trin with config: {:?}", trin_config);

    let web3_http_address = format!("http://127.0.0.1:{}", trin_config.httpPort);
    let (mut rx, child) = app
        .shell()
        .sidecar("trin")
        .expect("failed to create `trin` binary command")
        .args([
            "--web3-transport=http",
            "--portal-subnetworks=history",
            format!("--web3-http-address={}", web3_http_address).as_str(),
            format!("--mb={}", trin_config.storage).as_str(),
        ])
        .spawn()
        .map_err(|e| e.to_string())?;

    // spawn a thread that will read the stdout of the trin process
    let app_clone = app.clone();
    let log_handle = tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line_bytes) = event {
                let line = String::from_utf8_lossy(&line_bytes);
                info!("Child process stdout: {}", line);
                if line.contains("reports~ data:") {
                    let node_history_log = NodeHistoryLog::parse_log_line(&line);
                    match node_history_log {
                        Ok(log) => {
                            let state = app_clone.state::<Mutex<AppData>>();
                            let mut state = state.lock().unwrap();
                            state.node_stats.node_history_log = log;
                        }
                        Err(e) => {
                            warn!("Failed to parse log line: {}", e);
                        }
                    }
                }
            }
        }
    });

    // if trin is not responding to jsonrpc requests after 30 seconds,
    // we assume it crashed
    let mut i = 0;
    while i <= 30 {
        info!("checking trin");
        // trin has successfully started
        if check_trin_status(&trin_config.httpPort).await {
            break;
        }
        sleep(Duration::from_secs(1));
        i += 1;
        if i == 20 {
            let _ = child.kill();
            return Err("unable to get a response from the rpc server".to_string());
        }
    }

    // spawn a thread that will ping the trin node every 3 seconds
    // to make sure it is still running
    let app_clone = app.clone();
    let pid = child.pid();
    let status_handle = tauri::async_runtime::spawn(async move {
        info!("checking trin status, pid: {:?}", pid);
        // Initialize system information gatherer
        let mut sys = System::new_all();
        loop {
            // Refresh process list and CPU usage
            sys.refresh_all();

            // Get process ID for the port
            let pid = Pid::from(pid as usize);

            // Get the main process and all its children
            let mut total_cpu = 0.0;
            let mut process_count = 0;

            // Check main process
            if let Some(process) = sys.process(pid) {
                total_cpu += process.cpu_usage();
                process_count += 1;

                // Get all processes to check for children
                for (_pid_check, process_check) in sys.processes() {
                    if process_check.parent() == Some(pid) {
                        total_cpu += process_check.cpu_usage();
                        process_count += 1;
                    }
                }
            }
            println!(
                "Port {}: {} process(es), Total CPU Usage: {:.1}%",
                pid, process_count, total_cpu
            );

            // idk why but this has to happen before updating the trin stats
            if !check_trin_status(&trin_config.httpPort).await {
                app_clone
                    .emit("trin-crashed", ())
                    .expect("failed to emit event");
                break;
            }

            // update cpu in trin stats
            {
                let state = app_clone.state::<Mutex<AppData>>();
                let mut state = state.lock().unwrap();
                state.node_stats.cpu = total_cpu as f32;
                state.node_stats.pid = pid.into();
                app_clone
                    .emit("trin-stats", state.node_stats.clone())
                    .expect("failed to emit event");
            }
            // drop mutex lock here asap
            sleep(Duration::from_secs(3));
        }
    });

    // todo: test by killing this - then remove
    info!("Child process started: {:?}", pid);
    let mut app_data = app_data.lock().unwrap();
    app_data.status_handle = Some(status_handle);
    app_data.log_handle = Some(log_handle);
    app_data.trin_handle = Some(child);
    Ok("started".to_string())
}

#[tauri::command]
pub async fn shutdown_trin<'l>(app_data: State<'l, Mutex<AppData>>) -> Result<String, String> {
    info!("stopping trin");
    let mut app_data = app_data.lock().unwrap();
    if let Some(child) = app_data.trin_handle.take() {
        child.kill().expect("failed to kill child process");
    } else {
        warn!("unable to kill trin child process");
    }
    if let Some(handle) = app_data.log_handle.take() {
        handle.abort();
    } else {
        warn!("unable to kill log handle");
    }
    if let Some(handle) = app_data.status_handle.take() {
        handle.abort();
    } else {
        warn!("unable to kill status handle");
    }
    Ok(format!("stopped trin"))
}
