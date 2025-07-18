use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    pub async fn run_iostat(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("iostat");
        command.args(["-x", "1", "1"]);

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
                    tool_name: "iostat".to_string(),
                    command: "iostat -x 1 1".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "iostat".to_string(),
                command: "iostat -x 1 1".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_smartctl(&self, device: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("smartctl");
        command.args(["-a", device]);

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
                    tool_name: "smartctl".to_string(),
                    command: format!("smartctl -a {}", device),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "smartctl".to_string(),
                command: format!("smartctl -a {}", device),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_fdisk(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("fdisk");
        command.args(["-l"]);

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
                    tool_name: "fdisk".to_string(),
                    command: "fdisk -l".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "fdisk".to_string(),
                command: "fdisk -l".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_lsblk(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("lsblk");
        command.args(["-f"]);

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
                    tool_name: "lsblk".to_string(),
                    command: "lsblk -f".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "lsblk".to_string(),
                command: "lsblk -f".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_mount(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("mount");

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
                    tool_name: "mount".to_string(),
                    command: "mount".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "mount".to_string(),
                command: "mount".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_du(&self, path: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("du");
        command.args(["-sh", path]);

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
                    tool_name: "du".to_string(),
                    command: format!("du -sh {}", path),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "du".to_string(),
                command: format!("du -sh {}", path),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_find_large_files(&self, path: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("find");
        command.args([
            path, "-type", "f", "-size", "+100M", "-exec", "ls", "-lh", "{}", ";",
        ]);

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
                    tool_name: "find_large_files".to_string(),
                    command: format!("find {} -type f -size +100M -exec ls -lh {{}} \\;", path),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "find_large_files".to_string(),
                command: format!("find {} -type f -size +100M -exec ls -lh {{}} \\;", path),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_hdparm(&self, device: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("hdparm");
        command.args(["-I", device]);

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
                    tool_name: "hdparm".to_string(),
                    command: format!("hdparm -I {}", device),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "hdparm".to_string(),
                command: format!("hdparm -I {}", device),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_blkid(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("blkid");

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
                    tool_name: "blkid".to_string(),
                    command: "blkid".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "blkid".to_string(),
                command: "blkid".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }
}
