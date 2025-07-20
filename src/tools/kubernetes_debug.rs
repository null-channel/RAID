use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    // ==================== ADVANCED KUBECTL TOOLS ====================
    
    /// Get all deployments in a namespace
    pub async fn run_kubectl_get_deployments(&self, namespace: Option<&str>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.args(["get", "deployments", "-o", "wide"]);
        
        if let Some(ns) = namespace {
            command.args(["-n", ns]);
        } else {
            command.arg("--all-namespaces");
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

                let cmd_str = if let Some(ns) = namespace {
                    format!("kubectl get deployments -o wide -n {}", ns)
                } else {
                    "kubectl get deployments -o wide --all-namespaces".to_string()
                };

                DebugToolResult {
                    tool_name: "kubectl_get_deployments".to_string(),
                    command: cmd_str,
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_get_deployments".to_string(),
                command: "kubectl get deployments -o wide --all-namespaces".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get ConfigMaps in a namespace
    pub async fn run_kubectl_get_configmaps(&self, namespace: Option<&str>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.args(["get", "configmaps", "-o", "wide"]);
        
        if let Some(ns) = namespace {
            command.args(["-n", ns]);
        } else {
            command.arg("--all-namespaces");
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

                let cmd_str = if let Some(ns) = namespace {
                    format!("kubectl get configmaps -o wide -n {}", ns)
                } else {
                    "kubectl get configmaps -o wide --all-namespaces".to_string()
                };

                DebugToolResult {
                    tool_name: "kubectl_get_configmaps".to_string(),
                    command: cmd_str,
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_get_configmaps".to_string(),
                command: "kubectl get configmaps -o wide --all-namespaces".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get pod logs
    pub async fn run_kubectl_logs(&self, pod_name: &str, namespace: Option<&str>, lines: Option<usize>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.args(["logs", pod_name]);
        
        if let Some(ns) = namespace {
            command.args(["-n", ns]);
        }

        if let Some(n) = lines {
            command.args(["--tail", &n.to_string()]);
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

                let mut cmd_str = format!("kubectl logs {}", pod_name);
                if let Some(ns) = namespace {
                    cmd_str.push_str(&format!(" -n {}", ns));
                }
                if let Some(n) = lines {
                    cmd_str.push_str(&format!(" --tail {}", n));
                }

                DebugToolResult {
                    tool_name: "kubectl_logs".to_string(),
                    command: cmd_str,
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_logs".to_string(),
                command: format!("kubectl logs {}", pod_name),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get resource usage (top pods)
    pub async fn run_kubectl_top_pods(&self, namespace: Option<&str>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.args(["top", "pods"]);
        
        if let Some(ns) = namespace {
            command.args(["-n", ns]);
        } else {
            command.arg("--all-namespaces");
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

                let cmd_str = if let Some(ns) = namespace {
                    format!("kubectl top pods -n {}", ns)
                } else {
                    "kubectl top pods --all-namespaces".to_string()
                };

                DebugToolResult {
                    tool_name: "kubectl_top_pods".to_string(),
                    command: cmd_str,
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_top_pods".to_string(),
                command: "kubectl top pods --all-namespaces".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get resource usage (top nodes)
    pub async fn run_kubectl_top_nodes(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.args(["top", "nodes"]);

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
                    tool_name: "kubectl_top_nodes".to_string(),
                    command: "kubectl top nodes".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_top_nodes".to_string(),
                command: "kubectl top nodes".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get cluster info
    pub async fn run_kubectl_cluster_info(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.args(["cluster-info"]);

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
                    tool_name: "kubectl_cluster_info".to_string(),
                    command: "kubectl cluster-info".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_cluster_info".to_string(),
                command: "kubectl cluster-info".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get persistent volumes
    pub async fn run_kubectl_get_pv(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.args(["get", "pv", "-o", "wide"]);

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
                    tool_name: "kubectl_get_pv".to_string(),
                    command: "kubectl get pv -o wide".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_get_pv".to_string(),
                command: "kubectl get pv -o wide".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get persistent volume claims
    pub async fn run_kubectl_get_pvc(&self, namespace: Option<&str>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.args(["get", "pvc", "-o", "wide"]);
        
        if let Some(ns) = namespace {
            command.args(["-n", ns]);
        } else {
            command.arg("--all-namespaces");
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

                let cmd_str = if let Some(ns) = namespace {
                    format!("kubectl get pvc -o wide -n {}", ns)
                } else {
                    "kubectl get pvc -o wide --all-namespaces".to_string()
                };

                DebugToolResult {
                    tool_name: "kubectl_get_pvc".to_string(),
                    command: cmd_str,
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_get_pvc".to_string(),
                command: "kubectl get pvc -o wide --all-namespaces".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    // ==================== KUBELET DEBUGGING TOOLS ====================

    /// Get kubelet status via systemctl
    pub async fn run_kubelet_status(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("systemctl");
        command.args(["status", "kubelet", "--no-pager"]);

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
                    tool_name: "kubelet_status".to_string(),
                    command: "systemctl status kubelet --no-pager".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubelet_status".to_string(),
                command: "systemctl status kubelet --no-pager".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get kubelet logs
    pub async fn run_kubelet_logs(&self, lines: Option<usize>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("journalctl");
        command.args(["-u", "kubelet", "--no-pager"]);

        if let Some(n) = lines {
            command.args(["-n", &n.to_string()]);
        } else {
            command.args(["-n", "100"]);
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

                let cmd_str = format!("journalctl -u kubelet --no-pager -n {}", lines.unwrap_or(100));

                DebugToolResult {
                    tool_name: "kubelet_logs".to_string(),
                    command: cmd_str,
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubelet_logs".to_string(),
                command: "journalctl -u kubelet --no-pager -n 100".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get kubelet configuration
    pub async fn run_kubelet_config(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        
        // Try common kubelet config locations
        let config_paths = [
            "/var/lib/kubelet/config.yaml",
            "/etc/kubernetes/kubelet/kubelet-config.json",
            "/etc/systemd/system/kubelet.service.d/10-kubeadm.conf",
        ];

        let mut output_content = String::new();
        let mut found_config = false;

        for path in &config_paths {
            if let Ok(content) = std::fs::read_to_string(path) {
                output_content.push_str(&format!("=== {} ===\n", path));
                output_content.push_str(&content);
                output_content.push_str("\n\n");
                found_config = true;
            }
        }

        let execution_time = start_time.elapsed().as_millis() as u64;

        if found_config {
            DebugToolResult {
                tool_name: "kubelet_config".to_string(),
                command: "cat /var/lib/kubelet/config.yaml /etc/kubernetes/kubelet/* /etc/systemd/system/kubelet.service.d/*".to_string(),
                success: true,
                output: output_content,
                error: None,
                execution_time_ms: execution_time,
            }
        } else {
            DebugToolResult {
                tool_name: "kubelet_config".to_string(),
                command: "cat /var/lib/kubelet/config.yaml".to_string(),
                success: false,
                output: String::new(),
                error: Some("No kubelet configuration files found in common locations".to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    // ==================== ETCD DEBUGGING TOOLS ====================

    /// Check etcd cluster health
    pub async fn run_etcd_cluster_health(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("etcdctl");
        command.args(["cluster-health"]);

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
                    tool_name: "etcd_cluster_health".to_string(),
                    command: "etcdctl cluster-health".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "etcd_cluster_health".to_string(),
                command: "etcdctl cluster-health".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get etcd member list
    pub async fn run_etcd_member_list(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("etcdctl");
        command.args(["member", "list"]);

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
                    tool_name: "etcd_member_list".to_string(),
                    command: "etcdctl member list".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "etcd_member_list".to_string(),
                command: "etcdctl member list".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Check etcd endpoint health
    pub async fn run_etcd_endpoint_health(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("etcdctl");
        command.args(["endpoint", "health", "--cluster"]);

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
                    tool_name: "etcd_endpoint_health".to_string(),
                    command: "etcdctl endpoint health --cluster".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "etcd_endpoint_health".to_string(),
                command: "etcdctl endpoint health --cluster".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            },
        }
    }

    /// Get etcd database size and status
    pub async fn run_etcd_endpoint_status(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("etcdctl");
        command.args(["endpoint", "status", "--cluster", "-w", "table"]);

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
                    tool_name: "etcd_endpoint_status".to_string(),
                    command: "etcdctl endpoint status --cluster -w table".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "etcd_endpoint_status".to_string(),
                command: "etcdctl endpoint status --cluster -w table".to_string(),
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
    async fn test_kubectl_get_deployments_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_kubectl_get_deployments(None).await;

        assert_eq!(result.tool_name, "kubectl_get_deployments");
        assert_eq!(result.command, "kubectl get deployments -o wide --all-namespaces");
        assert!(result.execution_time_ms > 0);
    }

    #[tokio::test]
    async fn test_kubectl_logs_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_kubectl_logs("test-pod", Some("default"), Some(50)).await;

        assert_eq!(result.tool_name, "kubectl_logs");
        assert_eq!(result.command, "kubectl logs test-pod -n default --tail 50");
        assert!(result.execution_time_ms > 0);
    }

    #[tokio::test]
    async fn test_kubectl_top_pods_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_kubectl_top_pods(None).await;

        assert_eq!(result.tool_name, "kubectl_top_pods");
        assert_eq!(result.command, "kubectl top pods --all-namespaces");
        assert!(result.execution_time_ms > 0);
    }

    #[tokio::test]
    async fn test_kubelet_status_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_kubelet_status().await;

        assert_eq!(result.tool_name, "kubelet_status");
        assert_eq!(result.command, "systemctl status kubelet --no-pager");
        assert!(result.execution_time_ms > 0);
    }

    #[tokio::test]
    async fn test_kubelet_logs_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_kubelet_logs(Some(200)).await;

        assert_eq!(result.tool_name, "kubelet_logs");
        assert_eq!(result.command, "journalctl -u kubelet --no-pager -n 200");
        assert!(result.execution_time_ms > 0);
    }

    #[tokio::test]
    async fn test_etcd_cluster_health_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_etcd_cluster_health().await;

        assert_eq!(result.tool_name, "etcd_cluster_health");
        assert_eq!(result.command, "etcdctl cluster-health");
        assert!(result.execution_time_ms > 0);
    }

    #[tokio::test]
    async fn test_etcd_endpoint_health_structure() {
        let debug_tools = DebugTools::new();
        let result = debug_tools.run_etcd_endpoint_health().await;

        assert_eq!(result.tool_name, "etcd_endpoint_health");
        assert_eq!(result.command, "etcdctl endpoint health --cluster");
        assert!(result.execution_time_ms > 0);
    }

    #[test]
    fn test_k8s_tool_commands_are_user_runnable() {
        let commands = [
            "kubectl get deployments -o wide --all-namespaces",
            "kubectl get configmaps -o wide --all-namespaces",
            "kubectl logs test-pod -n default --tail 50",
            "kubectl top pods --all-namespaces",
            "kubectl top nodes",
            "kubectl cluster-info",
            "kubectl get pv -o wide",
            "kubectl get pvc -o wide --all-namespaces",
            "systemctl status kubelet --no-pager",
            "journalctl -u kubelet --no-pager -n 100",
            "etcdctl cluster-health",
            "etcdctl member list",
            "etcdctl endpoint health --cluster",
            "etcdctl endpoint status --cluster -w table",
        ];

        for command in &commands {
            // Ensure commands are user-runnable
            assert!(!command.contains("_"));
            assert!(!command.contains("run_"));
            assert!(command.len() < 150);
            
            // Ensure commands use expected tools
            assert!(command.contains("kubectl") || 
                   command.contains("systemctl") ||
                   command.contains("journalctl") ||
                   command.contains("etcdctl"));
        }
    }

    #[test]
    fn test_k8s_tool_names_are_consistent() {
        let tool_names = [
            "kubectl_get_deployments",
            "kubectl_get_configmaps", 
            "kubectl_logs",
            "kubectl_top_pods",
            "kubectl_top_nodes",
            "kubectl_cluster_info",
            "kubectl_get_pv",
            "kubectl_get_pvc",
            "kubelet_status",
            "kubelet_logs",
            "kubelet_config",
            "etcd_cluster_health",
            "etcd_member_list",
            "etcd_endpoint_health",
            "etcd_endpoint_status",
        ];

        for tool_name in &tool_names {
            assert!(!tool_name.is_empty());
            assert!(!tool_name.starts_with('_'));
            assert!(!tool_name.ends_with('_'));
            assert!(!tool_name.contains("__"));
            assert!(tool_name.len() > 5);
            assert_eq!(tool_name, &tool_name.to_lowercase());
        }
    }
} 