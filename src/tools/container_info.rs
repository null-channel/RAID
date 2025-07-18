use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    pub async fn run_cat_proc_cgroups(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("cat");
        command.arg("/proc/cgroups");

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
                    tool_name: "cat_proc_cgroups".to_string(),
                    command: "cat /proc/cgroups".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "cat_proc_cgroups".to_string(),
                command: "cat /proc/cgroups".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_ls_cgroup(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ls");
        command.args(["-la", "/sys/fs/cgroup"]);

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
                    tool_name: "ls_cgroup".to_string(),
                    command: "ls -la /sys/fs/cgroup".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "ls_cgroup".to_string(),
                command: "ls -la /sys/fs/cgroup".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_cat_proc_self_cgroup(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("cat");
        command.arg("/proc/self/cgroup");

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
                    tool_name: "cat_proc_self_cgroup".to_string(),
                    command: "cat /proc/self/cgroup".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "cat_proc_self_cgroup".to_string(),
                command: "cat /proc/self/cgroup".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_cat_proc_self_mountinfo(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("cat");
        command.arg("/proc/self/mountinfo");

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
                    tool_name: "cat_proc_self_mountinfo".to_string(),
                    command: "cat /proc/self/mountinfo".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "cat_proc_self_mountinfo".to_string(),
                command: "cat /proc/self/mountinfo".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_lsns(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("lsns");
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
                    tool_name: "lsns".to_string(),
                    command: "lsns -l".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "lsns".to_string(),
                command: "lsns -l".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_cat_proc_self_status(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("cat");
        command.arg("/proc/self/status");

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
                    tool_name: "cat_proc_self_status".to_string(),
                    command: "cat /proc/self/status".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "cat_proc_self_status".to_string(),
                command: "cat /proc/self/status".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_cat_proc_self_ns(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ls");
        command.args(["-la", "/proc/self/ns"]);

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
                    tool_name: "cat_proc_self_ns".to_string(),
                    command: "ls -la /proc/self/ns".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "cat_proc_self_ns".to_string(),
                command: "ls -la /proc/self/ns".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_docker_ps(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("docker");
        command.args([
            "ps",
            "-a",
            "--format",
            "table {{.Names}}\t{{.Status}}\t{{.Ports}}\t{{.Image}}",
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
                    tool_name: "docker_ps".to_string(),
                    command: "docker ps -a --format \"table {{.Names}}\\t{{.Status}}\\t{{.Ports}}\\t{{.Image}}\"".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "docker_ps".to_string(),
                command: "docker ps -a --format \"table {{.Names}}\\t{{.Status}}\\t{{.Ports}}\\t{{.Image}}\"".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_docker_ps_running(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("docker");
        command.args([
            "ps",
            "--format",
            "table {{.Names}}\t{{.Status}}\t{{.Ports}}\t{{.Image}}",
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
                    tool_name: "docker_ps_running".to_string(),
                    command: "docker ps --format \"table {{.Names}}\\t{{.Status}}\\t{{.Ports}}\\t{{.Image}}\"".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "docker_ps_running".to_string(),
                command: "docker ps --format \"table {{.Names}}\\t{{.Status}}\\t{{.Ports}}\\t{{.Image}}\"".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_docker_inspect(&self, container_name: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("docker");
        command.args([
            "inspect",
            container_name,
            "--format",
            "{{.State.Status}} - {{.State.Running}} - {{.Config.Image}}",
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
                    tool_name: "docker_inspect".to_string(),
                    command: format!(
                        "docker inspect {} --format \"{{{{.State.Status}}}} - {{{{.State.Running}}}} - {{{{.Config.Image}}}}\"",
                        container_name
                    ),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "docker_inspect".to_string(),
                command: format!(
                    "docker inspect {} --format \"{{{{.State.Status}}}} - {{{{.State.Running}}}} - {{{{.Config.Image}}}}\"",
                    container_name
                ),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_docker_logs(
        &self,
        container_name: &str,
        lines: Option<usize>,
    ) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("docker");
        command.args(["logs"]);

        if let Some(n) = lines {
            command.args(["--tail", &n.to_string()]);
        } else {
            command.args(["--tail", "20"]); // Default to 20 lines
        }

        command.arg(container_name);

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
                    tool_name: "docker_logs".to_string(),
                    command: format!(
                        "docker logs --tail {} {}",
                        lines.unwrap_or(20),
                        container_name
                    ),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "docker_logs".to_string(),
                command: format!(
                    "docker logs --tail {} {}",
                    lines.unwrap_or(20),
                    container_name
                ),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }
}
