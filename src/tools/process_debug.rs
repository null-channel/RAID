use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    pub async fn run_lsof(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("lsof");
        command.args(["-i"]);

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
                    tool_name: "lsof".to_string(),
                    command: "lsof -i".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "lsof".to_string(),
                command: "lsof -i".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_lsof_pid(&self, pid: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("lsof");
        command.args(["-p", pid]);

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
                    tool_name: "lsof_pid".to_string(),
                    command: format!("lsof -p {}", pid),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "lsof_pid".to_string(),
                command: format!("lsof -p {}", pid),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_strace(&self, pid: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("strace");
        command.args(["-p", pid, "-c"]);

        // Removed command.timeout, as std::process::Command does not have this method

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
                    tool_name: "strace".to_string(),
                    command: format!("strace -p {} -c", pid),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "strace".to_string(),
                command: format!("strace -p {} -c", pid),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_pmap(&self, pid: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("pmap");
        command.args([pid]);

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
                    tool_name: "pmap".to_string(),
                    command: format!("pmap {}", pid),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "pmap".to_string(),
                command: format!("pmap {}", pid),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_pidstat(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("pidstat");
        command.args(["-u", "-r", "-d", "1", "1"]);

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
                    tool_name: "pidstat".to_string(),
                    command: "pidstat -u -r -d 1 1".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "pidstat".to_string(),
                command: "pidstat -u -r -d 1 1".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_pgrep(&self, pattern: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("pgrep");
        command.args(["-f", pattern]);

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
                    tool_name: "pgrep".to_string(),
                    command: format!("pgrep -f {}", pattern),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "pgrep".to_string(),
                command: format!("pgrep -f {}", pattern),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_pkill(&self, pattern: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("pkill");
        command.args(["-f", pattern]);

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
                    tool_name: "pkill".to_string(),
                    command: format!("pkill -f {}", pattern),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "pkill".to_string(),
                command: format!("pkill -f {}", pattern),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_nice(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ps");
        command.args(["ax", "-o", "pid,ni,comm"]);

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
                    tool_name: "nice".to_string(),
                    command: "ps ax -o pid,ni,comm".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "nice".to_string(),
                command: "ps ax -o pid,ni,comm".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }
}
