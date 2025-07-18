use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    pub async fn run_systemctl_status(&self, service_name: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("systemctl");
        command.args(["status", service_name, "--no-pager"]);

        let result = command.output();
        let execution_time = start_time.elapsed().as_millis() as u64;

        match result {
            Ok(output) => {
                let success = output.status.success();
                let output_str = String::from_utf8_lossy(&output.stdout).to_string();
                let error_str = if success {
                    None
                } else {
                    Some(String::from_utf8_lossy(&output.stderr).to_string())
                };

                DebugToolResult {
                    tool_name: "systemctl_status".to_string(),
                    command: format!("systemctl status {} --no-pager", service_name),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "systemctl_status".to_string(),
                command: format!("systemctl status {} --no-pager", service_name),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }
} 