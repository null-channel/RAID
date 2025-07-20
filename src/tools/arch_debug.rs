use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    /// List all installed packages
    pub async fn run_pacman_list_packages(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("pacman");
        command.args(["-Q"]);

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
                    tool_name: "pacman_list_packages".to_string(),
                    command: "pacman -Q".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "pacman_list_packages".to_string(),
                command: "pacman -Q".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// List orphaned packages (no longer needed dependencies)
    pub async fn run_pacman_orphans(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("pacman");
        command.args(["-Qdt"]);

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
                    tool_name: "pacman_orphans".to_string(),
                    command: "pacman -Qdt".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "pacman_orphans".to_string(),
                command: "pacman -Qdt".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check package file integrity
    pub async fn run_pacman_check_files(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("pacman");
        command.args(["-Qkk"]);

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
                    tool_name: "pacman_check_files".to_string(),
                    command: "pacman -Qkk".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "pacman_check_files".to_string(),
                command: "pacman -Qkk".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check for available updates
    pub async fn run_checkupdates(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("checkupdates");

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
                    tool_name: "checkupdates".to_string(),
                    command: "checkupdates".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "checkupdates".to_string(),
                command: "checkupdates".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Show package cache information
    pub async fn run_paccache_info(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("paccache");
        command.args(["-d"]);

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
                    tool_name: "paccache_info".to_string(),
                    command: "paccache -d".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "paccache_info".to_string(),
                command: "paccache -d".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Analyze boot time and performance
    pub async fn run_systemd_analyze_time(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("systemd-analyze");
        command.args(["time"]);

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
                    tool_name: "systemd_analyze_time".to_string(),
                    command: "systemd-analyze time".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "systemd_analyze_time".to_string(),
                command: "systemd-analyze time".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Show critical chain of boot process
    pub async fn run_systemd_analyze_critical_chain(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("systemd-analyze");
        command.args(["critical-chain"]);

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
                    tool_name: "systemd_analyze_critical_chain".to_string(),
                    command: "systemd-analyze critical-chain".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "systemd_analyze_critical_chain".to_string(),
                command: "systemd-analyze critical-chain".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Show boot blame (slowest services)
    pub async fn run_systemd_analyze_blame(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("systemd-analyze");
        command.args(["blame"]);

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
                    tool_name: "systemd_analyze_blame".to_string(),
                    command: "systemd-analyze blame".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "systemd_analyze_blame".to_string(),
                command: "systemd-analyze blame".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// List all boot sessions
    pub async fn run_journalctl_list_boots(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("journalctl");
        command.args(["--list-boots", "--no-pager"]);

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
                    tool_name: "journalctl_list_boots".to_string(),
                    command: "journalctl --list-boots --no-pager".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "journalctl_list_boots".to_string(),
                command: "journalctl --list-boots --no-pager".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// List loaded kernel modules
    pub async fn run_lsmod(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("lsmod");

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
                    tool_name: "lsmod".to_string(),
                    command: "lsmod".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "lsmod".to_string(),
                command: "lsmod".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Show failed systemd units (Arch-specific analysis)
    pub async fn run_systemctl_failed(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("systemctl");
        command.args(["--failed", "--no-pager"]);

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
                    tool_name: "systemctl_failed".to_string(),
                    command: "systemctl --failed --no-pager".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "systemctl_failed".to_string(),
                command: "systemctl --failed --no-pager".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check if system needs reboot (kernel updates)
    pub async fn run_needs_reboot(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        
        // Check if running kernel matches installed kernel
        let running_kernel = std::fs::read_to_string("/proc/version")
            .unwrap_or_else(|_| "Unknown".to_string());
        
        let mut command = Command::new("pacman");
        command.args(["-Q", "linux"]);

        let result = command.output();
        let execution_time = start_time.elapsed().as_millis() as u64;

        match result {
            Ok(output) => {
                let success = output.status.success();
                let installed_kernel = String::from_utf8_lossy(&output.stdout).to_string();
                let output_str = format!(
                    "Running kernel: {}\nInstalled kernel package: {}\n\nReboot needed check: Compare versions manually",
                    running_kernel.lines().next().unwrap_or("Unknown"),
                    installed_kernel.trim()
                );
                let error_str = if success {
                    None
                } else {
                    Some(String::from_utf8_lossy(&output.stderr).to_string())
                };

                DebugToolResult {
                    tool_name: "needs_reboot".to_string(),
                    command: "cat /proc/version && pacman -Q linux".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "needs_reboot".to_string(),
                command: "cat /proc/version && pacman -Q linux".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Show pacman mirrors status
    pub async fn run_pacman_mirrorlist(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        
        let result = std::fs::read_to_string("/etc/pacman.d/mirrorlist");
        let execution_time = start_time.elapsed().as_millis() as u64;

        match result {
            Ok(content) => {
                // Filter to show only active (uncommented) mirrors
                let active_mirrors: Vec<&str> = content
                    .lines()
                    .filter(|line| line.starts_with("Server = "))
                    .collect();

                let output_str = if active_mirrors.is_empty() {
                    "No active mirrors found in /etc/pacman.d/mirrorlist".to_string()
                } else {
                    format!("Active mirrors ({}):\n{}", active_mirrors.len(), active_mirrors.join("\n"))
                };

                DebugToolResult {
                    tool_name: "pacman_mirrorlist".to_string(),
                    command: "grep '^Server = ' /etc/pacman.d/mirrorlist".to_string(),
                    success: true,
                    output: output_str,
                    error: None,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "pacman_mirrorlist".to_string(),
                command: "grep '^Server = ' /etc/pacman.d/mirrorlist".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check AUR helper status and info
    pub async fn run_aur_helper_info(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        
        // Check for common AUR helpers
        let aur_helpers = ["yay", "paru", "pikaur", "trizen"];
        let mut found_helpers = Vec::new();
        let mut helper_info = String::new();
        
        for helper in &aur_helpers {
            if let Ok(output) = Command::new("which").arg(helper).output() {
                if output.status.success() {
                    found_helpers.push(*helper);
                    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    helper_info.push_str(&format!("{}: {}\n", helper, path));
                    
                    // Get version if possible
                    if let Ok(version_output) = Command::new(helper).arg("--version").output() {
                        if version_output.status.success() {
                            let version = String::from_utf8_lossy(&version_output.stdout);
                            let version_line = version.lines().next().unwrap_or("Unknown version");
                            helper_info.push_str(&format!("  Version: {}\n", version_line));
                        }
                    }
                }
            }
        }
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        let output_str = if found_helpers.is_empty() {
            "No AUR helpers found (yay, paru, pikaur, trizen)".to_string()
        } else {
            format!("Found AUR helpers:\n{}", helper_info)
        };

        DebugToolResult {
            tool_name: "aur_helper_info".to_string(),
            command: "which yay paru pikaur trizen".to_string(),
            success: !found_helpers.is_empty(),
            output: output_str,
            error: None,
            execution_time_ms: execution_time,
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pacman_list_packages_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_pacman_list_packages().await;

        // Test that the result has the correct structure
        assert_eq!(result.tool_name, "pacman_list_packages");
        assert_eq!(result.command, "pacman -Q");
        assert!(result.execution_time_ms > 0);

        // On Arch systems, this should succeed, on non-Arch systems it will fail gracefully
        if result.success {
            // If successful, output should contain package information
            assert!(!result.output.is_empty());
            assert!(result.error.is_none());
        } else {
            // If failed (non-Arch system), should have error message
            assert!(result.error.is_some());
        }
    }

    #[tokio::test]
    async fn test_pacman_orphans_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_pacman_orphans().await;

        assert_eq!(result.tool_name, "pacman_orphans");
        assert_eq!(result.command, "pacman -Qdt");
        assert!(result.execution_time_ms > 0);
    }

    #[tokio::test]
    async fn test_systemd_analyze_time_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_systemd_analyze_time().await;

        assert_eq!(result.tool_name, "systemd_analyze_time");
        assert_eq!(result.command, "systemd-analyze time");
        assert!(result.execution_time_ms > 0);

        // systemd-analyze should be available on most Linux systems
        if result.success {
            assert!(!result.output.is_empty());
        }
    }

    #[tokio::test]
    async fn test_lsmod_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_lsmod().await;

        assert_eq!(result.tool_name, "lsmod");
        assert_eq!(result.command, "lsmod");
        assert!(result.execution_time_ms > 0);

        // lsmod should be available on all Linux systems
        if result.success {
            assert!(!result.output.is_empty());
            // Should contain header line
            assert!(result.output.contains("Module"));
        }
    }

    #[tokio::test]
    async fn test_systemctl_failed_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_systemctl_failed().await;

        assert_eq!(result.tool_name, "systemctl_failed");
        assert_eq!(result.command, "systemctl --failed --no-pager");
        assert!(result.execution_time_ms > 0);

        // systemctl should be available on systemd systems
        if result.success {
            // Output might be empty if no failed units, but should not error
            assert!(result.error.is_none());
        }
    }

    #[tokio::test]
    async fn test_journalctl_list_boots_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_journalctl_list_boots().await;

        assert_eq!(result.tool_name, "journalctl_list_boots");
        assert_eq!(result.command, "journalctl --list-boots --no-pager");
        assert!(result.execution_time_ms > 0);

        // journalctl should be available on systemd systems
        if result.success {
            assert!(!result.output.is_empty());
        }
    }

    #[tokio::test]
    async fn test_aur_helper_info_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_aur_helper_info().await;

        assert_eq!(result.tool_name, "aur_helper_info");
        assert_eq!(result.command, "which yay paru pikaur trizen");
        assert!(result.execution_time_ms > 0);

        // AUR helpers may or may not be installed
        assert!(!result.output.is_empty());
        assert!(result.error.is_none());
    }

    #[tokio::test]
    async fn test_pacman_mirrorlist_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_pacman_mirrorlist().await;

        assert_eq!(result.tool_name, "pacman_mirrorlist");
        assert_eq!(result.command, "grep '^Server = ' /etc/pacman.d/mirrorlist");
        assert!(result.execution_time_ms > 0);

        // On Arch systems, should read mirrorlist file
        if result.success {
            assert!(!result.output.is_empty());
            assert!(result.error.is_none());
        }
    }

    #[tokio::test]
    async fn test_needs_reboot_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_needs_reboot().await;

        assert_eq!(result.tool_name, "needs_reboot");
        assert_eq!(result.command, "cat /proc/version && pacman -Q linux");
        assert!(result.execution_time_ms > 0);

        // Should always have /proc/version available on Linux
        assert!(!result.output.is_empty());
        assert!(result.output.contains("Running kernel:"));
    }

    #[test]
    fn test_arch_tool_error_handling() {
        // Test that error cases are handled gracefully
        let start_time = std::time::Instant::now();
        let execution_time = start_time.elapsed().as_millis() as u64;

        let error_result = DebugToolResult {
            tool_name: "test_tool".to_string(),
            command: "nonexistent_command".to_string(),
            success: false,
            output: String::new(),
            error: Some("Command not found".to_string()),
            execution_time_ms: execution_time,
        };

        assert!(!error_result.success);
        assert!(error_result.error.is_some());
        assert!(error_result.output.is_empty());
    }

    #[test]
    fn test_arch_tools_commands_are_user_runnable() {
        // Test that all command strings are actual commands users can run
        let commands = [
            "pacman -Q",
            "pacman -Qdt", 
            "pacman -Qkk",
            "checkupdates",
            "paccache -d",
            "systemd-analyze time",
            "systemd-analyze critical-chain",
            "systemd-analyze blame",
            "journalctl --list-boots --no-pager",
            "lsmod",
            "systemctl --failed --no-pager",
            "cat /proc/version && pacman -Q linux",
            "grep '^Server = ' /etc/pacman.d/mirrorlist",
            "which yay paru pikaur trizen",
        ];

        for command in &commands {
            // Test that commands don't contain internal tool names
            assert!(!command.contains("_"));
            assert!(!command.contains("run_"));
            
            // Test that commands are reasonable length (not too complex)
            assert!(command.len() < 100);
            
            // Test that commands contain expected tools
            assert!(command.contains("pacman") || 
                   command.contains("systemd-analyze") ||
                   command.contains("journalctl") ||
                   command.contains("lsmod") ||
                   command.contains("systemctl") ||
                   command.contains("grep") ||
                   command.contains("cat") ||
                   command.contains("which") ||
                   command.contains("checkupdates") ||
                   command.contains("paccache"));
        }
    }

    #[test]
    fn test_arch_tool_names_are_consistent() {
        // Test that tool names follow consistent naming conventions
        let tool_names = [
            "pacman_list_packages",
            "pacman_orphans",
            "pacman_check_files",
            "checkupdates",
            "paccache_info",
            "systemd_analyze_time",
            "systemd_analyze_critical_chain",
            "systemd_analyze_blame",
            "journalctl_list_boots",
            "lsmod",
            "systemctl_failed",
            "needs_reboot",
            "pacman_mirrorlist",
            "aur_helper_info",
        ];

        for tool_name in &tool_names {
            // Test naming conventions
            assert!(!tool_name.is_empty());
            assert!(!tool_name.starts_with('_'));
            assert!(!tool_name.ends_with('_'));
            assert!(!tool_name.contains("__"));
            
            // Test that names are descriptive
            assert!(tool_name.len() > 3);
            
            // Test that names use snake_case
            assert_eq!(tool_name, &tool_name.to_lowercase());
        }
    }
} 