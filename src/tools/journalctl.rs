use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    pub async fn run_journalctl_recent(&self, lines: Option<usize>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("journalctl");
        command.arg("--no-pager");

        if let Some(n) = lines {
            command.args(["-n", &n.to_string()]);
        } else {
            command.arg("-n").arg("50"); // Default to 50 lines
        }

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
                    tool_name: "journalctl_recent".to_string(),
                    command: format!("journalctl --no-pager -n {}", lines.unwrap_or(50)),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "journalctl_recent".to_string(),
                command: "journalctl --no-pager -n 50".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_journalctl_service(
        &self,
        service_name: &str,
        lines: Option<usize>,
    ) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("journalctl");
        command.args(["-u", service_name, "--no-pager"]);

        if let Some(n) = lines {
            command.args(["-n", &n.to_string()]);
        } else {
            command.arg("-n").arg("50"); // Default to 50 lines
        }

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
                    tool_name: "journalctl_service".to_string(),
                    command: format!(
                        "journalctl -u {} --no-pager -n {}",
                        service_name,
                        lines.unwrap_or(50)
                    ),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "journalctl_service".to_string(),
                command: format!("journalctl -u {} --no-pager -n 50", service_name),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_journalctl_boot(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("journalctl");
        command.args(["-b", "--no-pager", "-n", "100"]);

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
                    tool_name: "journalctl_boot".to_string(),
                    command: "journalctl -b --no-pager -n 100".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "journalctl_boot".to_string(),
                command: "journalctl -b --no-pager -n 100".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_journalctl_errors(&self, lines: Option<usize>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("journalctl");
        command.args(["-p", "err", "--no-pager"]);

        if let Some(n) = lines {
            command.args(["-n", &n.to_string()]);
        } else {
            command.arg("-n").arg("50"); // Default to 50 lines
        }

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
                    tool_name: "journalctl_errors".to_string(),
                    command: format!("journalctl -p err --no-pager -n {}", lines.unwrap_or(50)),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "journalctl_errors".to_string(),
                command: "journalctl -p err --no-pager -n 50".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }
}
