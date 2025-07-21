use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

// Re-export all tool modules
pub mod arch_debug;
pub mod container_info;
pub mod ebpf_debug;
pub mod journalctl;
pub mod kubectl;
pub mod kubernetes_debug;
pub mod network_debug;
pub mod performance_debug;
pub mod process_debug;
pub mod security_debug;
pub mod storage_debug;
pub mod system_info;
pub mod systemctl;

// Trait for checking tool availability
pub trait ToolAvailability {
    fn check_tool_availability(&self, tool_name: &str) -> bool {
        // Default implementation: check if command exists in PATH
        Command::new("which")
            .arg(tool_name)
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    fn check_file_exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }

    fn get_available_tools(&self) -> Vec<String>;
}

// Tool category enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ToolCategory {
    SystemInfo,
    NetworkDebug,
    ProcessDebug,
    StorageDebug,
    PerformanceDebug,
    SecurityDebug,
    ContainerInfo,
    Kubernetes,
    ArchLinux,
    EbpfDebug,
    Journalctl,
    Systemctl,
}

// Available tool information
#[derive(Debug, Clone)]
pub struct AvailableToolInfo {
    pub category: ToolCategory,
    pub tool_names: Vec<String>,
    pub is_available: bool,
    pub missing_dependencies: Vec<String>,
}

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
    pub available_tools: HashMap<ToolCategory, AvailableToolInfo>,
}

impl DebugTools {
    pub fn new() -> Self {
        let kubectl_path = Self::find_kubectl();
        let kubernetes_enabled = kubectl_path.is_some();

        Self {
            kubernetes_enabled,
            kubectl_path,
            available_tools: HashMap::new(),
        }
    }

    /// Initialize and check availability of all tools
    pub fn initialize_with_availability_check() -> Self {
        let mut debug_tools = Self::new();
        debug_tools.check_all_tool_availability();
        debug_tools
    }

    /// Check availability of all tool categories
    pub fn check_all_tool_availability(&mut self) {
        let categories = [
            ToolCategory::SystemInfo,
            ToolCategory::NetworkDebug,
            ToolCategory::ProcessDebug,
            ToolCategory::StorageDebug,
            ToolCategory::PerformanceDebug,
            ToolCategory::SecurityDebug,
            ToolCategory::ContainerInfo,
            ToolCategory::Kubernetes,
            ToolCategory::ArchLinux,
            ToolCategory::EbpfDebug,
            ToolCategory::Journalctl,
            ToolCategory::Systemctl,
        ];

        for category in &categories {
            let available_info = self.check_category_availability(category.clone());
            self.available_tools.insert(category.clone(), available_info);
        }
    }

    /// Check availability for a specific tool category
    fn check_category_availability(&self, category: ToolCategory) -> AvailableToolInfo {
        match category {
            ToolCategory::SystemInfo => self.check_system_info_tools(),
            ToolCategory::NetworkDebug => self.check_network_debug_tools(),
            ToolCategory::ProcessDebug => self.check_process_debug_tools(),
            ToolCategory::StorageDebug => self.check_storage_debug_tools(),
            ToolCategory::PerformanceDebug => self.check_performance_debug_tools(),
            ToolCategory::SecurityDebug => self.check_security_debug_tools(),
            ToolCategory::ContainerInfo => self.check_container_info_tools(),
            ToolCategory::Kubernetes => self.check_kubernetes_tools(),
            ToolCategory::ArchLinux => self.check_arch_debug_tools(),
            ToolCategory::EbpfDebug => self.check_ebpf_debug_tools(),
            ToolCategory::Journalctl => self.check_journalctl_tools(),
            ToolCategory::Systemctl => self.check_systemctl_tools(),
        }
    }

    /// Get list of available tool categories
    pub fn get_available_categories(&self) -> Vec<ToolCategory> {
        self.available_tools
            .iter()
            .filter(|(_, info)| info.is_available)
            .map(|(category, _)| category.clone())
            .collect()
    }

    /// Check if a specific tool category is available
    pub fn is_category_available(&self, category: &ToolCategory) -> bool {
        self.available_tools
            .get(category)
            .map(|info| info.is_available)
            .unwrap_or(false)
    }

    /// Get available tools for a category
    pub fn get_category_tools(&self, category: &ToolCategory) -> Vec<String> {
        self.available_tools
            .get(category)
            .map(|info| info.tool_names.clone())
            .unwrap_or_default()
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

    // Tool availability checking methods for each category
    fn check_system_info_tools(&self) -> AvailableToolInfo {
        let tools = ["ps", "netstat", "df", "free"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        AvailableToolInfo {
            category: ToolCategory::SystemInfo,
            tool_names: available_tools.clone(),
            is_available: !available_tools.is_empty(),
            missing_dependencies: missing_tools,
        }
    }

    fn check_network_debug_tools(&self) -> AvailableToolInfo {
        let tools = ["ip", "ss", "ping", "traceroute", "dig", "iptables", "ethtool", "arp", "tcpdump", "nft", "ufw", "systemctl"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        // Check for optional network tools
        let optional_tools = ["iwconfig", "netstat", "iperf3"];
        for tool in &optional_tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            }
            // Don't add optional tools to missing_tools since they're not required
        }

        AvailableToolInfo {
            category: ToolCategory::NetworkDebug,
            tool_names: available_tools.clone(),
            is_available: !available_tools.is_empty(),
            missing_dependencies: missing_tools,
        }
    }

    fn check_process_debug_tools(&self) -> AvailableToolInfo {
        let tools = ["lsof", "strace", "pmap", "pidstat", "pgrep", "pkill"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        AvailableToolInfo {
            category: ToolCategory::ProcessDebug,
            tool_names: available_tools.clone(),
            is_available: !available_tools.is_empty(),
            missing_dependencies: missing_tools,
        }
    }

    fn check_storage_debug_tools(&self) -> AvailableToolInfo {
        let tools = ["iostat", "smartctl", "fdisk", "lsblk", "mount", "du", "hdparm", "blkid"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        AvailableToolInfo {
            category: ToolCategory::StorageDebug,
            tool_names: available_tools.clone(),
            is_available: !available_tools.is_empty(),
            missing_dependencies: missing_tools,
        }
    }

    fn check_performance_debug_tools(&self) -> AvailableToolInfo {
        let tools = ["top", "vmstat", "sar", "mpstat", "iotop", "htop", "nethogs", "perf", "sysbench"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        AvailableToolInfo {
            category: ToolCategory::PerformanceDebug,
            tool_names: available_tools.clone(),
            is_available: !available_tools.is_empty(),
            missing_dependencies: missing_tools,
        }
    }

    fn check_security_debug_tools(&self) -> AvailableToolInfo {
        let tools = ["auditctl", "ausearch", "sestatus", "getenforce", "semodule", "w", "last", "fail2ban-client", "clamscan"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        AvailableToolInfo {
            category: ToolCategory::SecurityDebug,
            tool_names: available_tools.clone(),
            is_available: !available_tools.is_empty(),
            missing_dependencies: missing_tools,
        }
    }

    fn check_container_info_tools(&self) -> AvailableToolInfo {
        let tools = ["docker", "lsns"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        // Also check for required files
        let required_files = ["/proc/cgroups", "/sys/fs/cgroup"];
        for file in &required_files {
            if !self.check_file_exists(file) {
                missing_tools.push(format!("missing file: {}", file));
            }
        }

        AvailableToolInfo {
            category: ToolCategory::ContainerInfo,
            tool_names: available_tools.clone(),
            is_available: !available_tools.is_empty(),
            missing_dependencies: missing_tools,
        }
    }

    fn check_kubernetes_tools(&self) -> AvailableToolInfo {
        let tools = ["kubectl", "etcdctl"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        AvailableToolInfo {
            category: ToolCategory::Kubernetes,
            tool_names: available_tools.clone(),
            is_available: self.check_tool_availability("kubectl"), // kubectl is minimum requirement
            missing_dependencies: missing_tools,
        }
    }

    fn check_arch_debug_tools(&self) -> AvailableToolInfo {
        let tools = ["pacman", "systemd-analyze", "journalctl", "lsmod", "systemctl"];
        let arch_specific_tools = ["checkupdates", "paccache", "yay", "paru"];
        
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        // Check core tools
        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        // Check Arch-specific tools
        for tool in &arch_specific_tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        // Check for Arch Linux specific files
        let arch_files = ["/etc/pacman.conf", "/etc/pacman.d/mirrorlist"];
        let is_arch = arch_files.iter().all(|file| self.check_file_exists(file));

        AvailableToolInfo {
            category: ToolCategory::ArchLinux,
            tool_names: available_tools.clone(),
            is_available: is_arch && self.check_tool_availability("pacman"),
            missing_dependencies: missing_tools,
        }
    }

    fn check_ebpf_debug_tools(&self) -> AvailableToolInfo {
        let tools = ["bpftool", "bpftrace"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        // Check for BPF filesystem
        let bpf_mounted = self.check_file_exists("/sys/fs/bpf");
        if !bpf_mounted {
            missing_tools.push("BPF filesystem not mounted".to_string());
        }

        AvailableToolInfo {
            category: ToolCategory::EbpfDebug,
            tool_names: available_tools.clone(),
            is_available: !available_tools.is_empty() && bpf_mounted,
            missing_dependencies: missing_tools,
        }
    }

    fn check_journalctl_tools(&self) -> AvailableToolInfo {
        let tools = ["journalctl"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        AvailableToolInfo {
            category: ToolCategory::Journalctl,
            tool_names: available_tools.clone(),
            is_available: self.check_tool_availability("journalctl"),
            missing_dependencies: missing_tools,
        }
    }

    fn check_systemctl_tools(&self) -> AvailableToolInfo {
        let tools = ["systemctl"];
        let mut available_tools = Vec::new();
        let mut missing_tools = Vec::new();

        for tool in &tools {
            if self.check_tool_availability(tool) {
                available_tools.push(tool.to_string());
            } else {
                missing_tools.push(tool.to_string());
            }
        }

        AvailableToolInfo {
            category: ToolCategory::Systemctl,
            tool_names: available_tools.clone(),
            is_available: self.check_tool_availability("systemctl"),
            missing_dependencies: missing_tools,
        }
    }
}

impl ToolAvailability for DebugTools {
    fn get_available_tools(&self) -> Vec<String> {
        self.available_tools
            .values()
            .flat_map(|info| info.tool_names.clone())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_availability_trait() {
        let debug_tools = DebugTools::new();
        
        // Test that check_tool_availability works
        assert!(debug_tools.check_tool_availability("echo")); // echo should always be available
        assert!(!debug_tools.check_tool_availability("nonexistent_command_12345"));
        
        // Test file exists checking
        assert!(debug_tools.check_file_exists("/proc/version")); // Should exist on Linux
        assert!(!debug_tools.check_file_exists("/nonexistent/path/12345"));
    }

    #[test]
    fn test_tool_category_enum() {
        // Test that all enum variants are distinct
        let categories = vec![
            ToolCategory::SystemInfo,
            ToolCategory::NetworkDebug,
            ToolCategory::ProcessDebug,
            ToolCategory::StorageDebug,
            ToolCategory::PerformanceDebug,
            ToolCategory::SecurityDebug,
            ToolCategory::ContainerInfo,
            ToolCategory::Kubernetes,
            ToolCategory::ArchLinux,
            ToolCategory::EbpfDebug,
            ToolCategory::Journalctl,
            ToolCategory::Systemctl,
        ];

        // Should have 12 distinct categories
        assert_eq!(categories.len(), 12);
        
        // Test Debug trait works
        let category = ToolCategory::SystemInfo;
        let debug_output = format!("{:?}", category);
        assert_eq!(debug_output, "SystemInfo");
    }

    #[test]
    fn test_available_tool_info_structure() {
        let available_info = AvailableToolInfo {
            category: ToolCategory::SystemInfo,
            tool_names: vec!["ps".to_string(), "netstat".to_string()],
            is_available: true,
            missing_dependencies: vec![],
        };

        assert_eq!(available_info.category, ToolCategory::SystemInfo);
        assert_eq!(available_info.tool_names.len(), 2);
        assert!(available_info.is_available);
        assert!(available_info.missing_dependencies.is_empty());
    }

    #[test]
    fn test_debug_tools_initialization() {
        let debug_tools = DebugTools::new();
        
        // Should start with empty available_tools
        assert!(debug_tools.available_tools.is_empty());
        
        // Test initialization with availability check
        let mut debug_tools_with_check = DebugTools::initialize_with_availability_check();
        
        // Should have populated available_tools after initialization
        assert!(!debug_tools_with_check.available_tools.is_empty());
        
        // Should have at least some categories checked
        let available_categories = debug_tools_with_check.get_available_categories();
        // We expect at least SystemInfo to be available (ps, netstat, df, free are common)
        assert!(available_categories.len() > 0);
    }

    #[test]
    fn test_system_info_tools_availability() {
        let debug_tools = DebugTools::new();
        let system_info = debug_tools.check_system_info_tools();
        
        assert_eq!(system_info.category, ToolCategory::SystemInfo);
        assert!(!system_info.tool_names.is_empty() || !system_info.missing_dependencies.is_empty());
        
        // At least some common tools should be available on most systems
        if system_info.is_available {
            assert!(!system_info.tool_names.is_empty());
        }
    }

    #[test]
    fn test_network_debug_tools_availability() {
        let debug_tools = DebugTools::new();
        let network_info = debug_tools.check_network_debug_tools();
        
        assert_eq!(network_info.category, ToolCategory::NetworkDebug);
        
        // Should have checked for network tools including new ones
        let total_tools = network_info.tool_names.len() + network_info.missing_dependencies.len();
        assert!(total_tools >= 10); // Should check at least 10+ tools now
        
        // Basic tools like 'ip' and 'ping' should be available on most systems
        if network_info.is_available {
            assert!(
                network_info.tool_names.contains(&"ip".to_string()) ||
                network_info.tool_names.contains(&"ping".to_string())
            );
        }
        
        // Should check for UFW and systemctl
        let all_checked_tools: Vec<String> = network_info.tool_names.iter()
            .chain(network_info.missing_dependencies.iter())
            .cloned()
            .collect();
        
        // These tools should be in either available or missing list
        assert!(
            all_checked_tools.contains(&"ufw".to_string()) ||
            network_info.tool_names.contains(&"ufw".to_string())
        );
        assert!(
            all_checked_tools.contains(&"systemctl".to_string()) ||
            network_info.tool_names.contains(&"systemctl".to_string())
        );
    }

    #[test]
    fn test_kubernetes_tools_availability() {
        let debug_tools = DebugTools::new();
        let k8s_info = debug_tools.check_kubernetes_tools();
        
        assert_eq!(k8s_info.category, ToolCategory::Kubernetes);
        
        // Kubernetes availability depends on kubectl being available
        if debug_tools.check_tool_availability("kubectl") {
            assert!(k8s_info.is_available);
            assert!(k8s_info.tool_names.contains(&"kubectl".to_string()));
        } else {
            assert!(!k8s_info.is_available);
            assert!(k8s_info.missing_dependencies.contains(&"kubectl".to_string()));
        }
    }

    #[test]
    fn test_arch_linux_tools_availability() {
        let debug_tools = DebugTools::new();
        let arch_info = debug_tools.check_arch_debug_tools();
        
        assert_eq!(arch_info.category, ToolCategory::ArchLinux);
        
        // Arch availability depends on pacman and arch-specific files
        let has_pacman = debug_tools.check_tool_availability("pacman");
        let has_pacman_conf = debug_tools.check_file_exists("/etc/pacman.conf");
        
        if has_pacman && has_pacman_conf {
            assert!(arch_info.is_available);
            assert!(arch_info.tool_names.contains(&"pacman".to_string()));
        } else {
            assert!(!arch_info.is_available);
        }
    }

    #[test]
    fn test_ebpf_tools_availability() {
        let debug_tools = DebugTools::new();
        let ebpf_info = debug_tools.check_ebpf_debug_tools();
        
        assert_eq!(ebpf_info.category, ToolCategory::EbpfDebug);
        
        // eBPF availability depends on bpftool/bpftrace and BPF filesystem
        let has_bpf_fs = debug_tools.check_file_exists("/sys/fs/bpf");
        let has_bpftool = debug_tools.check_tool_availability("bpftool");
        
        if has_bpf_fs && has_bpftool {
            assert!(ebpf_info.is_available);
        } else {
            assert!(!ebpf_info.is_available);
        }
    }

    #[test]
    fn test_container_info_tools_availability() {
        let debug_tools = DebugTools::new();
        let container_info = debug_tools.check_container_info_tools();
        
        assert_eq!(container_info.category, ToolCategory::ContainerInfo);
        
        // Container info depends on basic container tools and cgroup files
        let has_cgroups = debug_tools.check_file_exists("/proc/cgroups");
        let has_cgroup_fs = debug_tools.check_file_exists("/sys/fs/cgroup");
        
        if !has_cgroups || !has_cgroup_fs {
            // Should note missing files in dependencies
            assert!(container_info.missing_dependencies.iter().any(|dep| dep.contains("missing file:")));
        }
    }

    #[test] 
    fn test_journalctl_and_systemctl_availability() {
        let debug_tools = DebugTools::new();
        
        let journalctl_info = debug_tools.check_journalctl_tools();
        assert_eq!(journalctl_info.category, ToolCategory::Journalctl);
        
        let systemctl_info = debug_tools.check_systemctl_tools();
        assert_eq!(systemctl_info.category, ToolCategory::Systemctl);
        
        // These should be available on systemd systems
        let has_journalctl = debug_tools.check_tool_availability("journalctl");
        let has_systemctl = debug_tools.check_tool_availability("systemctl");
        
        assert_eq!(journalctl_info.is_available, has_journalctl);
        assert_eq!(systemctl_info.is_available, has_systemctl);
    }

    #[test]
    fn test_category_filtering_methods() {
        let mut debug_tools = DebugTools::initialize_with_availability_check();
        
        // Test get_available_categories
        let available_categories = debug_tools.get_available_categories();
        for category in &available_categories {
            assert!(debug_tools.is_category_available(category));
        }
        
        // Test get_category_tools
        for category in &available_categories {
            let tools = debug_tools.get_category_tools(category);
            assert!(!tools.is_empty(), "Available category should have tools: {:?}", category);
        }
        
        // Test is_category_available for non-existent or unavailable tools
        // (This test might vary based on system, but structure should work)
        
        // Test get_available_tools
        let all_tools = debug_tools.get_available_tools();
        assert!(!all_tools.is_empty());
    }

    #[test]
    fn test_check_all_tool_availability() {
        let mut debug_tools = DebugTools::new();
        assert!(debug_tools.available_tools.is_empty());
        
        debug_tools.check_all_tool_availability();
        
        // Should have checked all categories
        assert_eq!(debug_tools.available_tools.len(), 12); // All categories should be checked
        
        // Each category should have some result
        for (category, info) in &debug_tools.available_tools {
            assert_eq!(info.category, *category);
            // Either should have available tools or missing dependencies tracked
            assert!(
                !info.tool_names.is_empty() || !info.missing_dependencies.is_empty(),
                "Category {:?} should have either tools or missing deps tracked", category
            );
        }
    }
}
