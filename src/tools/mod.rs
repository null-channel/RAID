use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Re-export all tool modules
pub mod container_info;
pub mod journalctl;
pub mod kubectl;
pub mod network_debug;
pub mod performance_debug;
pub mod process_debug;
pub mod security_debug;
pub mod storage_debug;
pub mod system_info;
pub mod systemctl;

// Common structures used across tools
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DebugToolResult {
    pub tool_name: String,
    pub command: String,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time_ms: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KubernetesDebugInfo {
    pub pods: Vec<PodInfo>,
    pub services: Vec<ServiceInfo>,
    pub nodes: Vec<NodeInfo>,
    pub events: Vec<EventInfo>,
    pub namespace_info: Option<NamespaceInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PodInfo {
    pub name: String,
    pub namespace: String,
    pub status: String,
    pub ready: String,
    pub restarts: String,
    pub age: String,
    pub ip: Option<String>,
    pub node: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub namespace: String,
    pub type_: String,
    pub cluster_ip: String,
    pub external_ip: Option<String>,
    pub ports: String,
    pub age: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NodeInfo {
    pub name: String,
    pub status: String,
    pub roles: String,
    pub age: String,
    pub version: String,
    pub internal_ip: Option<String>,
    pub external_ip: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EventInfo {
    pub last_seen: String,
    pub type_: String,
    pub reason: String,
    pub object: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NamespaceInfo {
    pub name: String,
    pub status: String,
    pub age: String,
    pub labels: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalDebugInfo {
    pub recent_logs: Vec<JournalLogEntry>,
    pub boot_logs: Vec<JournalLogEntry>,
    pub service_logs: Vec<JournalLogEntry>,
    pub error_logs: Vec<JournalLogEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JournalLogEntry {
    pub timestamp: String,
    pub unit: String,
    pub priority: String,
    pub message: String,
    pub pid: Option<String>,
}

pub struct DebugTools {
    pub kubernetes_enabled: bool,
    pub kubectl_path: Option<String>,
}

impl DebugTools {
    pub fn new() -> Self {
        let kubectl_path = Self::find_kubectl();
        let kubernetes_enabled = kubectl_path.is_some();

        Self {
            kubernetes_enabled,
            kubectl_path,
        }
    }

    fn find_kubectl() -> Option<String> {
        // Check if kubectl is available in PATH
        if let Ok(output) = std::process::Command::new("which").arg("kubectl").output() {
            if output.status.success() {
                return Some(String::from_utf8_lossy(&output.stdout).trim().to_string());
            }
        }
        None
    }
}
