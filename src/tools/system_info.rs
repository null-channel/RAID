use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    pub async fn run_ps_aux(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ps");
        command.args(["aux"]);

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
                    tool_name: "ps_aux".to_string(),
                    command: "ps aux".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "ps_aux".to_string(),
                command: "ps aux".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_netstat(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("netstat");
        command.args(["-tuln"]);

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
                    tool_name: "netstat".to_string(),
                    command: "netstat -tuln".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "netstat".to_string(),
                command: "netstat -tuln".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_df(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("df");
        command.args(["-h"]);

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
                    tool_name: "df".to_string(),
                    command: "df -h".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "df".to_string(),
                command: "df -h".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_free(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("free");
        command.args(["-h"]);

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
                    tool_name: "free".to_string(),
                    command: "free -h".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "free".to_string(),
                command: "free -h".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }
}
