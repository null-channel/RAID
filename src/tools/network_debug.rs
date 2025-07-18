use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    pub async fn run_ip_addr(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ip");
        command.args(["addr", "show"]);

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
                    tool_name: "ip_addr".to_string(),
                    command: "ip addr show".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "ip_addr".to_string(),
                command: "ip addr show".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_ip_route(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ip");
        command.args(["route", "show"]);

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
                    tool_name: "ip_route".to_string(),
                    command: "ip route show".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "ip_route".to_string(),
                command: "ip route show".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_ss(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ss");
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
                    tool_name: "ss".to_string(),
                    command: "ss -tuln".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "ss".to_string(),
                command: "ss -tuln".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_ping(&self, host: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ping");
        command.args(["-c", "3", host]);

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
                    tool_name: "ping".to_string(),
                    command: format!("ping -c 3 {}", host),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "ping".to_string(),
                command: format!("ping -c 3 {}", host),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_traceroute(&self, host: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("traceroute");
        command.args([host]);

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
                    tool_name: "traceroute".to_string(),
                    command: format!("traceroute {}", host),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "traceroute".to_string(),
                command: format!("traceroute {}", host),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_dig(&self, domain: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("dig");
        command.args([domain]);

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
                    tool_name: "dig".to_string(),
                    command: format!("dig {}", domain),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "dig".to_string(),
                command: format!("dig {}", domain),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_iptables(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("iptables");
        command.args(["-L", "-n", "-v"]);

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
                    tool_name: "iptables".to_string(),
                    command: "iptables -L -n -v".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "iptables".to_string(),
                command: "iptables -L -n -v".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_ethtool(&self, interface: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ethtool");
        command.args([interface]);

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
                    tool_name: "ethtool".to_string(),
                    command: format!("ethtool {}", interface),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "ethtool".to_string(),
                command: format!("ethtool {}", interface),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }
} 