use super::{DebugToolResult, DebugTools};
use std::process::Command;

impl DebugTools {
    pub async fn run_kubectl_get_pods(&self, namespace: Option<&str>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.arg("get").arg("pods").arg("--output=wide");
        
        if let Some(ns) = namespace {
            command.args(["-n", ns]);
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
                    tool_name: "kubectl_get_pods".to_string(),
                    command: format!("kubectl get pods --output=wide {}", 
                        namespace.map(|ns| format!("-n {}", ns)).unwrap_or_default()),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_get_pods".to_string(),
                command: "kubectl get pods --output=wide".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_kubectl_describe_pod(&self, pod_name: &str, namespace: Option<&str>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.arg("describe").arg("pod").arg(pod_name);
        
        if let Some(ns) = namespace {
            command.args(["-n", ns]);
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
                    tool_name: "kubectl_describe_pod".to_string(),
                    command: format!("kubectl describe pod {} {}", 
                        pod_name,
                        namespace.map(|ns| format!("-n {}", ns)).unwrap_or_default()),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_describe_pod".to_string(),
                command: format!("kubectl describe pod {}", pod_name),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_kubectl_get_services(&self, namespace: Option<&str>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.arg("get").arg("services").arg("--output=wide");
        
        if let Some(ns) = namespace {
            command.args(["-n", ns]);
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
                    tool_name: "kubectl_get_services".to_string(),
                    command: format!("kubectl get services --output=wide {}", 
                        namespace.map(|ns| format!("-n {}", ns)).unwrap_or_default()),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_get_services".to_string(),
                command: "kubectl get services --output=wide".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_kubectl_get_nodes(&self) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.arg("get").arg("nodes").arg("--output=wide");

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
                    tool_name: "kubectl_get_nodes".to_string(),
                    command: "kubectl get nodes --output=wide".to_string(),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_get_nodes".to_string(),
                command: "kubectl get nodes --output=wide".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }

    pub async fn run_kubectl_get_events(&self, namespace: Option<&str>) -> DebugToolResult {
        let start_time = std::time::Instant::now();
        let mut command = Command::new("kubectl");
        command.arg("get").arg("events");
        
        if let Some(ns) = namespace {
            command.args(["-n", ns]);
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
                    tool_name: "kubectl_get_events".to_string(),
                    command: format!("kubectl get events {}", 
                        namespace.map(|ns| format!("-n {}", ns)).unwrap_or_default()),
                    success,
                    output: output_str,
                    error: error_str,
                    execution_time_ms: execution_time,
                }
            }
            Err(e) => DebugToolResult {
                tool_name: "kubectl_get_events".to_string(),
                command: "kubectl get events".to_string(),
                success: false,
                output: String::new(),
                error: Some(e.to_string()),
                execution_time_ms: execution_time,
            }
        }
    }
} 