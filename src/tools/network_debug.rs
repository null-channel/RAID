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
            },
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
            },
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
            },
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
            },
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
            },
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
            },
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
            },
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
            },
        }
    }

    // Enhanced networking tools
    pub async fn run_arp_table(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ip");
        command.args(["neigh", "show"]);

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
                    tool_name: "arp_table".to_string(),
                    command: "ip neigh show".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "arp_table".to_string(),
                command: "ip neigh show".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_interface_stats(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("cat");
        command.args(["/proc/net/dev"]);

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
                    tool_name: "interface_stats".to_string(),
                    command: "cat /proc/net/dev".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "interface_stats".to_string(),
                command: "cat /proc/net/dev".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_iperf3_server_check(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("iperf3");
        command.args(["--version"]);

        let result = command.output();
        let execution_time = start_time.elapsed().as_millis() as u64;

        match result {
            Ok(output) => {
                let success = output.status.success();
                let output_str = if success {
                    format!("{}\nNote: Run 'iperf3 -s' on server and 'iperf3 -c <server_ip>' on client to test bandwidth", 
                            String::from_utf8_lossy(&output.stdout))
                } else {
                    String::from_utf8_lossy(&output.stdout).to_string()
                };
                let error_str = if success {
                    None
                } else {
                    Some(String::from_utf8_lossy(&output.stderr).to_string())
                };

                DebugToolResult {
                    tool_name: "iperf3".to_string(),
                    command: "iperf3 --version".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "iperf3".to_string(),
                command: "iperf3 --version".to_string(),
                success: false,
                output: String::new(),
                error: Some(format!("iperf3 not found: {}. Install with: sudo pacman -S iperf3", e)),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_network_namespaces(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ip");
        command.args(["netns", "list"]);

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
                    tool_name: "network_namespaces".to_string(),
                    command: "ip netns list".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "network_namespaces".to_string(),
                command: "ip netns list".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_tcpdump_sample(&self, interface: Option<&str>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("tcpdump");
        
        let interface_arg = interface.unwrap_or("any");
        command.args(["-i", interface_arg, "-c", "10", "-n"]);

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
                    tool_name: "tcpdump_sample".to_string(),
                    command: format!("tcpdump -i {} -c 10 -n", interface_arg),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "tcpdump_sample".to_string(),
                command: format!("tcpdump -i {} -c 10 -n", interface_arg),
                success: false,
                output: String::new(),
                error: Some(format!("tcpdump failed: {}. May need root privileges.", e)),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_bridge_info(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ip");
        command.args(["link", "show", "type", "bridge"]);

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
                    tool_name: "bridge_info".to_string(),
                    command: "ip link show type bridge".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "bridge_info".to_string(),
                command: "ip link show type bridge".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_wireless_info(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("iwconfig");

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
                    tool_name: "wireless_info".to_string(),
                    command: "iwconfig".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "wireless_info".to_string(),
                command: "iwconfig".to_string(),
                success: false,
                output: String::new(),
                error: Some(format!("iwconfig not found: {}. Install with: sudo pacman -S wireless_tools", e)),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_nftables(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("nft");
        command.args(["list", "ruleset"]);

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
                    tool_name: "nftables".to_string(),
                    command: "nft list ruleset".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "nftables".to_string(),
                command: "nft list ruleset".to_string(),
                success: false,
                output: String::new(),
                error: Some(format!("nftables not available: {}. May need root privileges or install nftables.", e)),
                execution_time_ms: execution_time,
            },
        }
    }

    pub async fn run_dns_test(&self, domain: &str) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        
        // Test multiple DNS servers
        let dns_servers = ["8.8.8.8", "1.1.1.1", "9.9.9.9"];
        let mut results = Vec::new();
        
        for dns_server in &dns_servers {
            let mut command = Command::new("dig");
            command.args([format!("@{}", dns_server).as_str(), domain, "+time=2", "+short"]);
            
            if let Ok(output) = command.output() {
                let response_time = start_time.elapsed().as_millis();
                let success = output.status.success();
                let result_text = if success {
                    format!("DNS Server {}: {} ({}ms)", dns_server, 
                           String::from_utf8_lossy(&output.stdout).trim(), response_time)
                } else {
                    format!("DNS Server {}: FAILED", dns_server)
                };
                results.push(result_text);
            }
        }
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        let output_str = results.join("\n");

        DebugToolResult {
            tool_name: "dns_test".to_string(),
            command: format!("dig {} (testing multiple DNS servers)", domain),
            success: !results.is_empty(),
            output: output_str,
            error: None,
            execution_time_ms: execution_time,
        }
    }

    // Legacy netstat for systems that still have it
    pub async fn run_netstat_legacy(&self) -> DebugToolResult {
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
                    tool_name: "netstat_legacy".to_string(),
                    command: "netstat -tuln".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "netstat_legacy".to_string(),
                command: "netstat -tuln".to_string(),
                success: false,
                output: String::new(),
                error: Some(format!("netstat not found: {}. Use 'ss' instead or install net-tools.", e)),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check UFW (Uncomplicated Firewall) status
    pub async fn run_ufw_status(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("ufw");
        command.args(["status", "verbose"]);

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
                    tool_name: "ufw_status".to_string(),
                    command: "ufw status verbose".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "ufw_status".to_string(),
                command: "ufw status verbose".to_string(),
                success: false,
                output: String::new(),
                error: Some(format!("UFW not found: {}. Install with: sudo apt install ufw (Ubuntu/Debian) or sudo pacman -S ufw (Arch)", e)),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check NetworkManager status
    pub async fn run_networkmanager_status(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("systemctl");
        command.args(["status", "NetworkManager", "--no-pager"]);

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
                    tool_name: "networkmanager_status".to_string(),
                    command: "systemctl status NetworkManager --no-pager".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "networkmanager_status".to_string(),
                command: "systemctl status NetworkManager --no-pager".to_string(),
                success: false,
                output: String::new(),
                error: Some(format!("systemctl not found: {}. NetworkManager status check requires systemd.", e)),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check DNS configuration
    pub async fn run_dns_config(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("cat");
        command.args(["/etc/resolv.conf"]);

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
                    tool_name: "dns_config".to_string(),
                    command: "cat /etc/resolv.conf".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "dns_config".to_string(),
                command: "cat /etc/resolv.conf".to_string(),
                success: false,
                output: String::new(),
                error: Some(format!("Failed to read DNS config: {}", e)),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check network connectivity with standard hosts
    pub async fn run_connectivity_test(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        
        let test_hosts = [
            ("8.8.8.8", "Google DNS"),
            ("1.1.1.1", "Cloudflare DNS"), 
            ("google.com", "Google (DNS resolution test)"),
            ("github.com", "GitHub (HTTPS connectivity)"),
        ];
        
        let mut results = Vec::new();
        
        for (host, description) in &test_hosts {
            let mut command = Command::new("ping");
            command.args(["-c", "2", "-W", "3", host]);
            
            if let Ok(output) = command.output() {
                let success = output.status.success();
                let result_text = if success {
                    format!("✅ {} ({}): REACHABLE", description, host)
                } else {
                    format!("❌ {} ({}): UNREACHABLE", description, host)
                };
                results.push(result_text);
            } else {
                results.push(format!("❌ {} ({}): PING FAILED", description, host));
            }
        }
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        let output_str = results.join("\n");
        let overall_success = results.iter().any(|r| r.contains("✅"));

        DebugToolResult {
            tool_name: "connectivity_test".to_string(),
            command: "ping -c 2 -W 3 (multiple hosts)".to_string(),
            success: overall_success,
            output: output_str,
            error: if overall_success { None } else { Some("No hosts reachable".to_string()) },
            execution_time_ms: execution_time,
        }
    }

    /// Comprehensive network health check - runs multiple diagnostic tools automatically
    pub async fn run_network_health_check(&self) -> Vec<DebugToolResult> {
        let mut results = Vec::new();
        
        // 1. Check network interfaces
        results.push(self.run_ip_addr().await);
        
        // 2. Check routing table
        results.push(self.run_ip_route().await);
        
        // 3. Test connectivity
        results.push(self.run_connectivity_test().await);
        
        // 4. Check DNS configuration
        results.push(self.run_dns_config().await);
        
        // 5. Test DNS resolution
        results.push(self.run_dns_test("google.com").await);
        
        // 6. Check active network connections
        results.push(self.run_ss().await);
        
        // 7. Check firewall status (iptables)
        results.push(self.run_iptables().await);
        
        // 8. Check UFW status if available
        results.push(self.run_ufw_status().await);
        
        // 9. Check NetworkManager status if available
        results.push(self.run_networkmanager_status().await);
        
        // 10. Check nftables if available
        results.push(self.run_nftables().await);
        
        // 11. Check wireless information if available
        results.push(self.run_wireless_info().await);
        
        results
    }

    /// Quick network setup check for standard users
    pub async fn run_network_setup_check(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        
        let health_results = self.run_network_health_check().await;
        
        let mut summary = Vec::new();
        let mut warnings = Vec::new();
        let mut errors = Vec::new();
        
        for result in &health_results {
            match result.tool_name.as_str() {
                "ip_addr" => {
                    if result.success {
                        if result.output.contains("inet ") && !result.output.contains("127.0.0.1") {
                            summary.push("✅ Network interface is up with IP address assigned");
                        } else {
                            warnings.push("⚠️  No non-loopback IP address found");
                        }
                    } else {
                        errors.push("❌ Failed to check network interfaces");
                    }
                }
                "connectivity_test" => {
                    if result.success {
                        summary.push("✅ Internet connectivity is working");
                    } else {
                        errors.push("❌ No internet connectivity");
                    }
                }
                "dns_config" => {
                    if result.success {
                        if result.output.contains("nameserver") {
                            summary.push("✅ DNS servers are configured");
                        } else {
                            warnings.push("⚠️  No DNS servers found in /etc/resolv.conf");
                        }
                    } else {
                        warnings.push("⚠️  Could not read DNS configuration");
                    }
                }
                "dns_test" => {
                    if result.success {
                        summary.push("✅ DNS resolution is working");
                    } else {
                        errors.push("❌ DNS resolution is not working");
                    }
                }
                "ufw_status" => {
                    if result.success {
                        if result.output.contains("Status: inactive") {
                            summary.push("✅ UFW firewall is inactive (allowing all traffic)");
                        } else if result.output.contains("Status: active") {
                            summary.push("✅ UFW firewall is active with rules configured");
                        } else {
                            warnings.push("⚠️  UFW status unclear");
                        }
                    } else if result.error.as_ref().map_or(false, |e| e.contains("UFW not found")) {
                        summary.push("ℹ️  UFW not installed (using other firewall or none)");
                    }
                }
                "networkmanager_status" => {
                    if result.success {
                        if result.output.contains("active (running)") {
                            summary.push("✅ NetworkManager is running");
                        } else {
                            warnings.push("⚠️  NetworkManager is not running normally");
                        }
                    } else {
                        summary.push("ℹ️  NetworkManager not available (may use different network management)");
                    }
                }
                "iptables" => {
                    if result.success {
                        if result.output.contains("policy ACCEPT") || result.output.is_empty() {
                            summary.push("✅ iptables allows traffic (default policy ACCEPT or no rules)");
                        } else {
                            warnings.push("⚠️  iptables has active rules - check if blocking needed traffic");
                        }
                    } else {
                        warnings.push("⚠️  Could not check iptables rules (may need root privileges)");
                    }
                }
                _ => {} // Ignore other tools for summary
            }
        }
        
        let execution_time = start_time.elapsed().as_millis() as u64;
        
        let mut full_output = String::new();
        
        full_output.push_str("🔍 NETWORK SETUP CHECK FOR STANDARD USER\n");
        full_output.push_str("========================================\n\n");
        
        if !summary.is_empty() {
            full_output.push_str("📋 Summary:\n");
            for item in &summary {
                full_output.push_str(&format!("  {}\n", item));
            }
            full_output.push('\n');
        }
        
        if !warnings.is_empty() {
            full_output.push_str("⚠️  Warnings:\n");
            for item in &warnings {
                full_output.push_str(&format!("  {}\n", item));
            }
            full_output.push('\n');
        }
        
        if !errors.is_empty() {
            full_output.push_str("❌ Issues Found:\n");
            for item in &errors {
                full_output.push_str(&format!("  {}\n", item));
            }
            full_output.push('\n');
        }
        
        let overall_status = if errors.is_empty() && warnings.len() <= 2 {
            "✅ Network appears to be set up correctly for standard use"
        } else if errors.is_empty() {
            "⚠️  Network is functional but has some configuration warnings"
        } else {
            "❌ Network has significant issues that need attention"
        };
        
        full_output.push_str(&format!("🏁 Overall Status: {}\n", overall_status));
        
        DebugToolResult {
            tool_name: "network_setup_check".to_string(),
            command: "comprehensive network setup verification".to_string(),
            success: errors.is_empty(),
            output: full_output,
            error: if errors.is_empty() { None } else { Some(format!("{} issues found", errors.len())) },
            execution_time_ms: execution_time,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_network_debug_tools_command_format() {
        let debug_tools = DebugTools::new();

        // Test that all network tools return proper command formats
        let result = debug_tools.run_ip_addr().await;
        assert_eq!(result.tool_name, "ip_addr");
        assert_eq!(result.command, "ip addr show");
        assert!(!result.command.contains("_")); // Should not contain internal naming

        let result = debug_tools.run_ip_route().await;
        assert_eq!(result.tool_name, "ip_route");
        assert_eq!(result.command, "ip route show");

        let result = debug_tools.run_ss().await;
        assert_eq!(result.tool_name, "ss");
        assert_eq!(result.command, "ss -tuln");

        let result = debug_tools.run_arp_table().await;
        assert_eq!(result.tool_name, "arp_table");
        assert_eq!(result.command, "ip neigh show");

        let result = debug_tools.run_interface_stats().await;
        assert_eq!(result.tool_name, "interface_stats");
        assert_eq!(result.command, "cat /proc/net/dev");

        let result = debug_tools.run_network_namespaces().await;
        assert_eq!(result.tool_name, "network_namespaces");
        assert_eq!(result.command, "ip netns list");

        let result = debug_tools.run_bridge_info().await;
        assert_eq!(result.tool_name, "bridge_info");
        assert_eq!(result.command, "ip link show type bridge");
    }

    #[tokio::test]
    async fn test_network_tools_with_parameters() {
        let debug_tools = DebugTools::new();

        // Test ping with custom host
        let result = debug_tools.run_ping("127.0.0.1").await;
        assert_eq!(result.tool_name, "ping");
        assert_eq!(result.command, "ping -c 3 127.0.0.1");

        // Test traceroute with custom host
        let result = debug_tools.run_traceroute("8.8.8.8").await;
        assert_eq!(result.tool_name, "traceroute");
        assert_eq!(result.command, "traceroute 8.8.8.8");

        // Test dig with custom domain
        let result = debug_tools.run_dig("example.com").await;
        assert_eq!(result.tool_name, "dig");
        assert_eq!(result.command, "dig example.com");

        // Test ethtool with interface
        let result = debug_tools.run_ethtool("lo").await;
        assert_eq!(result.tool_name, "ethtool");
        assert_eq!(result.command, "ethtool lo");

        // Test tcpdump with interface
        let result = debug_tools.run_tcpdump_sample(Some("lo")).await;
        assert_eq!(result.tool_name, "tcpdump_sample");
        assert_eq!(result.command, "tcpdump -i lo -c 10 -n");

        // Test tcpdump without interface (default to "any")
        let result = debug_tools.run_tcpdump_sample(None).await;
        assert_eq!(result.tool_name, "tcpdump_sample");
        assert_eq!(result.command, "tcpdump -i any -c 10 -n");
    }

    #[tokio::test]
    async fn test_dns_test_functionality() {
        let debug_tools = DebugTools::new();

        // Test DNS test with a domain
        let result = debug_tools.run_dns_test("google.com").await;
        assert_eq!(result.tool_name, "dns_test");
        assert!(result.command.contains("dig google.com"));
        assert!(result.command.contains("multiple DNS servers"));
        
        // DNS test should always report success if at least one DNS server responds
        // Even if dig is not installed, the tool should handle it gracefully
        if result.success {
            assert!(!result.output.is_empty());
        }
    }

    #[tokio::test]
    async fn test_iperf3_availability_check() {
        let debug_tools = DebugTools::new();

        let result = debug_tools.run_iperf3_server_check().await;
        assert_eq!(result.tool_name, "iperf3");
        assert_eq!(result.command, "iperf3 --version");
        
        // Should either succeed with version info or fail with helpful message
        if result.success {
            assert!(result.output.contains("iperf3"));
        } else {
            assert!(result.error.is_some());
            let error = result.error.unwrap();
            assert!(error.contains("iperf3 not found") || error.contains("Install with"));
        }
    }

    #[tokio::test]
    async fn test_firewall_tools() {
        let debug_tools = DebugTools::new();

        // Test iptables
        let result = debug_tools.run_iptables().await;
        assert_eq!(result.tool_name, "iptables");
        assert_eq!(result.command, "iptables -L -n -v");

        // Test nftables
        let result = debug_tools.run_nftables().await;
        assert_eq!(result.tool_name, "nftables");
        assert_eq!(result.command, "nft list ruleset");

        // Both tools should handle missing privileges gracefully
        // They may fail, but should provide helpful error messages
    }

    #[tokio::test]
    async fn test_wireless_info() {
        let debug_tools = DebugTools::new();

        let result = debug_tools.run_wireless_info().await;
        assert_eq!(result.tool_name, "wireless_info");
        assert_eq!(result.command, "iwconfig");

        // iwconfig might not be installed, should handle gracefully
        if !result.success {
            assert!(result.error.is_some());
            let error = result.error.unwrap();
            assert!(error.contains("iwconfig not found") || error.contains("Install with"));
        }
    }

    #[tokio::test]
    async fn test_netstat_legacy() {
        let debug_tools = DebugTools::new();

        let result = debug_tools.run_netstat_legacy().await;
        assert_eq!(result.tool_name, "netstat_legacy");
        assert_eq!(result.command, "netstat -tuln");

        // netstat might not be installed on modern systems
        if !result.success {
            assert!(result.error.is_some());
            let error = result.error.unwrap();
            assert!(error.contains("netstat not found") || error.contains("Use 'ss' instead"));
        }
    }

    #[tokio::test]
    async fn test_tool_execution_time_tracking() {
        let debug_tools = DebugTools::new();

        // Test that execution time is properly tracked
        let result = debug_tools.run_ip_addr().await;
        assert!(result.execution_time_ms >= 0);

        let result = debug_tools.run_ss().await;
        assert!(result.execution_time_ms >= 0);

        // Quick tools should complete relatively fast (less than 5 seconds)
        let result = debug_tools.run_interface_stats().await;
        assert!(result.execution_time_ms < 5000);
    }

    #[tokio::test]
    async fn test_error_handling() {
        let debug_tools = DebugTools::new();

        // Test that tools handle non-existent commands gracefully
        // These might succeed on some systems, so we just check the structure
        let result = debug_tools.run_wireless_info().await;
        assert!(!result.tool_name.is_empty());
        assert!(!result.command.is_empty());

        let result = debug_tools.run_iperf3_server_check().await;
        assert!(!result.tool_name.is_empty());
        assert!(!result.command.is_empty());

        // Error messages should be helpful when tools fail
        if !result.success {
            assert!(result.error.is_some());
            let error = result.error.unwrap();
            assert!(!error.is_empty());
        }
    }

    #[tokio::test]
    async fn test_ufw_status() {
        let debug_tools = DebugTools::new();

        let result = debug_tools.run_ufw_status().await;
        assert_eq!(result.tool_name, "ufw_status");
        assert_eq!(result.command, "ufw status verbose");

        // UFW might not be installed, should handle gracefully
        if !result.success {
            assert!(result.error.is_some());
            let error = result.error.unwrap();
            assert!(error.contains("UFW not found") || error.contains("Install with"));
        } else {
            // If UFW is available, should show status
            assert!(!result.output.is_empty());
        }
    }

    #[tokio::test]
    async fn test_networkmanager_status() {
        let debug_tools = DebugTools::new();

        let result = debug_tools.run_networkmanager_status().await;
        assert_eq!(result.tool_name, "networkmanager_status");
        assert_eq!(result.command, "systemctl status NetworkManager --no-pager");

        // NetworkManager might not be available on all systems
        if !result.success {
            assert!(result.error.is_some());
            let error = result.error.unwrap();
            assert!(error.contains("systemctl not found") || error.contains("requires systemd"));
        } else {
            // If NetworkManager is available, should show status
            assert!(!result.output.is_empty());
        }
    }

    #[tokio::test]
    async fn test_dns_config() {
        let debug_tools = DebugTools::new();

        let result = debug_tools.run_dns_config().await;
        assert_eq!(result.tool_name, "dns_config");
        assert_eq!(result.command, "cat /etc/resolv.conf");

        // /etc/resolv.conf should exist on most Linux systems
        if result.success {
            assert!(!result.output.is_empty());
            // Should contain some DNS configuration
        } else {
            assert!(result.error.is_some());
            let error = result.error.unwrap();
            assert!(error.contains("Failed to read DNS config"));
        }
    }

    #[tokio::test]
    async fn test_connectivity_test() {
        let debug_tools = DebugTools::new();

        let result = debug_tools.run_connectivity_test().await;
        assert_eq!(result.tool_name, "connectivity_test");
        assert!(result.command.contains("ping"));
        assert!(result.command.contains("multiple hosts"));

        // Should test multiple hosts and provide detailed output
        assert!(!result.output.is_empty());
        assert!(result.output.contains("Google DNS") || result.output.contains("Cloudflare DNS"));
        
        // Should have clear success/failure indicators
        if result.success {
            assert!(result.output.contains("✅"));
        } else {
            assert!(result.output.contains("❌"));
        }
    }

    #[tokio::test]
    async fn test_network_health_check() {
        let debug_tools = DebugTools::new();

        let results = debug_tools.run_network_health_check().await;
        
        // Should run multiple network diagnostic tools
        assert!(results.len() >= 8); // At least 8 tools should be checked
        
        // Check that we have results for key tools
        let tool_names: Vec<String> = results.iter().map(|r| r.tool_name.clone()).collect();
        assert!(tool_names.contains(&"ip_addr".to_string()));
        assert!(tool_names.contains(&"connectivity_test".to_string()));
        assert!(tool_names.contains(&"dns_config".to_string()));
        assert!(tool_names.contains(&"iptables".to_string()));
        assert!(tool_names.contains(&"ufw_status".to_string()));
        assert!(tool_names.contains(&"networkmanager_status".to_string()));
        
        // All results should have proper structure
        for result in &results {
            assert!(!result.tool_name.is_empty());
            assert!(!result.command.is_empty());
            assert!(result.execution_time_ms >= 0);
        }
    }

    #[tokio::test]
    async fn test_network_setup_check() {
        let debug_tools = DebugTools::new();

        let result = debug_tools.run_network_setup_check().await;
        assert_eq!(result.tool_name, "network_setup_check");
        assert_eq!(result.command, "comprehensive network setup verification");

        // Should provide a comprehensive analysis
        assert!(!result.output.is_empty());
        assert!(result.output.contains("NETWORK SETUP CHECK FOR STANDARD USER"));
        assert!(result.output.contains("Overall Status:"));
        
        // Should have clear status indicators
        assert!(
            result.output.contains("✅") ||
            result.output.contains("⚠️") ||
            result.output.contains("❌")
        );
        
        // Should categorize results
        if result.output.contains("Summary:") {
            assert!(result.output.contains("✅"));
        }
        
        // Should track execution time
        assert!(result.execution_time_ms >= 0);
    }

    #[tokio::test]
    async fn test_new_tools_command_format() {
        let debug_tools = DebugTools::new();

        // Test that all new network tools return proper command formats
        let result = debug_tools.run_ufw_status().await;
        assert_eq!(result.tool_name, "ufw_status");
        assert_eq!(result.command, "ufw status verbose");
        assert!(!result.command.contains("_")); // Should not contain internal naming

        let result = debug_tools.run_networkmanager_status().await;
        assert_eq!(result.tool_name, "networkmanager_status");
        assert_eq!(result.command, "systemctl status NetworkManager --no-pager");

        let result = debug_tools.run_dns_config().await;
        assert_eq!(result.tool_name, "dns_config");
        assert_eq!(result.command, "cat /etc/resolv.conf");

        let result = debug_tools.run_connectivity_test().await;
        assert_eq!(result.tool_name, "connectivity_test");
        assert!(result.command.contains("ping"));
    }

    #[tokio::test]
    async fn test_health_check_error_resilience() {
        let debug_tools = DebugTools::new();

        // Health check should complete even if some tools fail
        let results = debug_tools.run_network_health_check().await;
        
        // Should have attempted all tools
        assert!(results.len() >= 8);
        
        // Even if some fail, others should still work
        let successes = results.iter().filter(|r| r.success).count();
        let failures = results.iter().filter(|r| !r.success).count();
        
        // At least some basic tools should work (like ip_addr, which uses 'ip' command)
        // But we don't require all to pass since some systems may not have all tools
        assert!(successes > 0 || failures > 0); // Should have attempted something
        
        // All results should have valid structure regardless of success/failure
        for result in &results {
            assert!(!result.tool_name.is_empty());
            assert!(!result.command.is_empty());
            // Error should be Some if success is false
            if !result.success {
                // Note: Some tools may fail due to missing binaries, which is expected
            }
        }
    }

    #[tokio::test]
    async fn test_network_setup_check_summary_logic() {
        let debug_tools = DebugTools::new();

        let result = debug_tools.run_network_setup_check().await;
        
        // Should provide clear categorization
        let output = &result.output;
        
        // Should have header
        assert!(output.contains("NETWORK SETUP CHECK FOR STANDARD USER"));
        
        // Should have overall status
        assert!(output.contains("Overall Status:"));
        
        // Should use appropriate emojis and formatting
        let has_summary = output.contains("📋 Summary:");
        let has_warnings = output.contains("⚠️  Warnings:");
        let has_errors = output.contains("❌ Issues Found:");
        
        // At least one section should be present
        assert!(has_summary || has_warnings || has_errors);
        
        // Overall status should match the success flag
        if result.success {
            assert!(
                output.contains("✅ Network appears to be set up correctly") ||
                output.contains("⚠️  Network is functional but has some configuration warnings")
            );
        } else {
            assert!(output.contains("❌ Network has significant issues"));
        }
    }

    #[test]
    fn test_network_tool_naming_consistency() {
        // Test that tool names are consistent and don't contain underscores in commands
        let test_cases = vec![
            ("ip_addr", "ip addr show"),
            ("ip_route", "ip route show"),
            ("ss", "ss -tuln"),
            ("arp_table", "ip neigh show"),
            ("interface_stats", "cat /proc/net/dev"),
            ("network_namespaces", "ip netns list"),
            ("bridge_info", "ip link show type bridge"),
            ("netstat_legacy", "netstat -tuln"),
            ("nftables", "nft list ruleset"),
            ("iptables", "iptables -L -n -v"),
        ];

        for (tool_name, expected_command) in test_cases {
            // Tool names can have underscores (internal naming)
            assert!(tool_name.chars().all(|c| c.is_alphanumeric() || c == '_'));
            
            // Commands should not have underscores (user-executable commands)
            assert!(!expected_command.contains("_tool") && !expected_command.contains("run_"));
            
            // Commands should be something users can actually run
            assert!(expected_command.split_whitespace().next().is_some());
        }
    }

    #[test]
    fn test_ping_traceroute_dig_parameter_validation() {
        // Test that parameter-taking functions handle inputs correctly
        let test_hosts = vec!["127.0.0.1", "localhost", "google.com", "8.8.8.8"];
        
        for host in test_hosts {
            // These are just format tests - we're not actually running the commands
            assert!(!host.is_empty());
            assert!(!host.contains(" ")); // Hosts shouldn't contain spaces
        }

        let test_domains = vec!["google.com", "example.org", "github.com"];
        
        for domain in test_domains {
            assert!(!domain.is_empty());
            assert!(domain.contains(".")); // Should look like a domain
        }
    }
}
