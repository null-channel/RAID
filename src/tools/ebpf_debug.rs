use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    /// List all loaded BPF programs
    pub async fn run_bpftool_prog_list(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["prog", "list"]);

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
                    tool_name: "bpftool_prog_list".to_string(),
                    command: "bpftool prog list".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_prog_list".to_string(),
                command: "bpftool prog list".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Show detailed information about a specific BPF program
    pub async fn run_bpftool_prog_show(&self, prog_id: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["prog", "show", "id", prog_id]);

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
                    tool_name: "bpftool_prog_show".to_string(),
                    command: format!("bpftool prog show id {}", prog_id),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_prog_show".to_string(),
                command: format!("bpftool prog show id {}", prog_id),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Dump BPF program bytecode (translated)
    pub async fn run_bpftool_prog_dump_xlated(&self, prog_id: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["prog", "dump", "xlated", "id", prog_id]);

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
                    tool_name: "bpftool_prog_dump_xlated".to_string(),
                    command: format!("bpftool prog dump xlated id {}", prog_id),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_prog_dump_xlated".to_string(),
                command: format!("bpftool prog dump xlated id {}", prog_id),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Dump BPF program JIT-compiled code
    pub async fn run_bpftool_prog_dump_jited(&self, prog_id: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["prog", "dump", "jited", "id", prog_id]);

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
                    tool_name: "bpftool_prog_dump_jited".to_string(),
                    command: format!("bpftool prog dump jited id {}", prog_id),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_prog_dump_jited".to_string(),
                command: format!("bpftool prog dump jited id {}", prog_id),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// List all BPF maps
    pub async fn run_bpftool_map_list(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["map", "list"]);

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
                    tool_name: "bpftool_map_list".to_string(),
                    command: "bpftool map list".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_map_list".to_string(),
                command: "bpftool map list".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Show detailed information about a specific BPF map
    pub async fn run_bpftool_map_show(&self, map_id: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["map", "show", "id", map_id]);

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
                    tool_name: "bpftool_map_show".to_string(),
                    command: format!("bpftool map show id {}", map_id),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_map_show".to_string(),
                command: format!("bpftool map show id {}", map_id),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Dump BPF map contents
    pub async fn run_bpftool_map_dump(&self, map_id: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["map", "dump", "id", map_id]);

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
                    tool_name: "bpftool_map_dump".to_string(),
                    command: format!("bpftool map dump id {}", map_id),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_map_dump".to_string(),
                command: format!("bpftool map dump id {}", map_id),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// List all BPF links (attachments)
    pub async fn run_bpftool_link_list(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["link", "list"]);

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
                    tool_name: "bpftool_link_list".to_string(),
                    command: "bpftool link list".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_link_list".to_string(),
                command: "bpftool link list".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Show BPF feature support information
    pub async fn run_bpftool_feature_probe(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["feature", "probe"]);

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
                    tool_name: "bpftool_feature_probe".to_string(),
                    command: "bpftool feature probe".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_feature_probe".to_string(),
                command: "bpftool feature probe".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// List BPF network attachments
    pub async fn run_bpftool_net_list(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["net", "list"]);

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
                    tool_name: "bpftool_net_list".to_string(),
                    command: "bpftool net list".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_net_list".to_string(),
                command: "bpftool net list".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// List BPF cgroup attachments
    pub async fn run_bpftool_cgroup_list(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["cgroup", "list", "/sys/fs/cgroup"]);

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
                    tool_name: "bpftool_cgroup_list".to_string(),
                    command: "bpftool cgroup list /sys/fs/cgroup".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_cgroup_list".to_string(),
                command: "bpftool cgroup list /sys/fs/cgroup".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// List BPF BTF (BPF Type Format) objects
    pub async fn run_bpftool_btf_list(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftool");
        command.args(["btf", "list"]);

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
                    tool_name: "bpftool_btf_list".to_string(),
                    command: "bpftool btf list".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftool_btf_list".to_string(),
                command: "bpftool btf list".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check if BPF filesystem is mounted
    pub async fn run_bpf_mount_check(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("mount");
        command.args(["-t", "bpf"]);

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
                    tool_name: "bpf_mount_check".to_string(),
                    command: "mount -t bpf".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpf_mount_check".to_string(),
                command: "mount -t bpf".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// List BPF pinned objects in filesystem
    pub async fn run_bpf_ls_pinned(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("find");
        command.args(["/sys/fs/bpf", "-type", "f", "2>/dev/null", "||", "echo", "BPF filesystem not mounted or no pinned objects"]);

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
                    tool_name: "bpf_ls_pinned".to_string(),
                    command: "find /sys/fs/bpf -type f".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpf_ls_pinned".to_string(),
                command: "find /sys/fs/bpf -type f".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Show kernel BPF configuration
    pub async fn run_bpf_kernel_config(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("grep");
        command.args(["-E", "CONFIG_BPF|CONFIG_CGROUP_BPF", "/proc/config.gz"]);

        // Fallback to checking boot config if /proc/config.gz doesn't exist
        let result = if let Ok(output) = command.output() {
            if output.status.success() {
                Ok(output)
            } else {
                Command::new("grep")
                    .args(["-E", "CONFIG_BPF|CONFIG_CGROUP_BPF", "/boot/config-$(uname -r)"])
                    .output()
                    .or_else(|_| {
                        Command::new("sh")
                            .args(["-c", "zcat /proc/config.gz 2>/dev/null | grep -E 'CONFIG_BPF|CONFIG_CGROUP_BPF' || echo 'BPF config not available'"])
                            .output()
                    })
            }
        } else {
            Command::new("sh")
                .args(["-c", "if [ -f /boot/config-$(uname -r) ]; then grep -E 'CONFIG_BPF|CONFIG_CGROUP_BPF' /boot/config-$(uname -r); else echo 'Kernel config not found'; fi"])
                .output()
        };

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
                    tool_name: "bpf_kernel_config".to_string(),
                    command: "grep CONFIG_BPF /proc/config.gz or /boot/config-$(uname -r)".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpf_kernel_config".to_string(),
                command: "grep CONFIG_BPF /proc/config.gz or /boot/config-$(uname -r)".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Simple BPF tracing one-liner (list syscalls)
    pub async fn run_bpftrace_syscalls(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("timeout");
        command.args(["5", "bpftrace", "-e", "tracepoint:raw_syscalls:sys_enter { @[comm] = count(); }"]);

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
                    tool_name: "bpftrace_syscalls".to_string(),
                    command: "timeout 5 bpftrace -e 'tracepoint:raw_syscalls:sys_enter { @[comm] = count(); }'".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftrace_syscalls".to_string(),
                command: "timeout 5 bpftrace -e 'tracepoint:raw_syscalls:sys_enter { @[comm] = count(); }'".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// List available BPF tracepoints
    pub async fn run_bpftrace_list_tracepoints(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("bpftrace");
        command.args(["-l", "tracepoint:*"]);

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
                    tool_name: "bpftrace_list_tracepoints".to_string(),
                    command: "bpftrace -l tracepoint:*".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpftrace_list_tracepoints".to_string(),
                command: "bpftrace -l tracepoint:*".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check BPF JIT status
    pub async fn run_bpf_jit_status(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("sysctl");
        command.args(["net.core.bpf_jit_enable"]);

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
                    tool_name: "bpf_jit_status".to_string(),
                    command: "sysctl net.core.bpf_jit_enable".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bpf_jit_status".to_string(),
                command: "sysctl net.core.bpf_jit_enable".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bpftool_prog_list_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_bpftool_prog_list().await;

        // Test that the result has the correct structure
        assert_eq!(result.tool_name, "bpftool_prog_list");
        assert_eq!(result.command, "bpftool prog list");
        assert!(result.execution_time_ms >= 0);

        // bpftool might not be available on all systems
        if result.success {
            assert!(!result.output.is_empty());
            assert!(result.error.is_none());
        } else {
            assert!(result.error.is_some());
        }
    }

    #[tokio::test]
    async fn test_bpftool_map_list_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_bpftool_map_list().await;

        assert_eq!(result.tool_name, "bpftool_map_list");
        assert_eq!(result.command, "bpftool map list");
        assert!(result.execution_time_ms >= 0);
    }

    #[tokio::test]
    async fn test_bpftool_link_list_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_bpftool_link_list().await;

        assert_eq!(result.tool_name, "bpftool_link_list");
        assert_eq!(result.command, "bpftool link list");
        assert!(result.execution_time_ms >= 0);
    }

    #[tokio::test]
    async fn test_bpf_mount_check_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_bpf_mount_check().await;

        assert_eq!(result.tool_name, "bpf_mount_check");
        assert_eq!(result.command, "mount -t bpf");
        assert!(result.execution_time_ms >= 0);
    }

    #[tokio::test]
    async fn test_bpf_jit_status_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_bpf_jit_status().await;

        assert_eq!(result.tool_name, "bpf_jit_status");
        assert_eq!(result.command, "sysctl net.core.bpf_jit_enable");
        assert!(result.execution_time_ms >= 0);
    }

    #[tokio::test]
    async fn test_bpftool_prog_show_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_bpftool_prog_show("1").await;

        assert_eq!(result.tool_name, "bpftool_prog_show");
        assert_eq!(result.command, "bpftool prog show id 1");
        assert!(result.execution_time_ms >= 0);
    }

    #[tokio::test]
    async fn test_bpftool_feature_probe_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_bpftool_feature_probe().await;

        assert_eq!(result.tool_name, "bpftool_feature_probe");
        assert_eq!(result.command, "bpftool feature probe");
        assert!(result.execution_time_ms >= 0);
    }

    #[test]
    fn test_ebpf_tool_commands_are_user_runnable() {
        let commands = [
            "bpftool prog list",
            "bpftool prog show id 1",
            "bpftool prog dump xlated id 1",
            "bpftool prog dump jited id 1",
            "bpftool map list",
            "bpftool map show id 1",
            "bpftool map dump id 1",
            "bpftool link list",
            "bpftool feature probe",
            "bpftool net list",
            "bpftool cgroup list /sys/fs/cgroup",
            "bpftool btf list",
            "mount -t bpf",
            "find /sys/fs/bpf -type f",
            "sysctl net.core.bpf_jit_enable",
            "bpftrace -l tracepoint:*",
        ];

        for command in &commands {
            // Ensure commands don't contain internal naming patterns (but allow technical terms like CONFIG_BPF)
            assert!(!command.contains("run_"));
            
            // Ensure commands are reasonable length
            assert!(command.len() < 100);
            
            // Ensure commands contain expected tools
            assert!(command.contains("bpftool") || 
                   command.contains("mount") ||
                   command.contains("find") ||
                   command.contains("sysctl") ||
                   command.contains("bpftrace") ||
                   command.contains("grep"));
        }
    }
} 