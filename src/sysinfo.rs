use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemInfo {
    pub os: String,
    pub cpu: String,
    pub total_memory: String,
    pub free_memory: String,
    pub total_disk: String,
    pub free_disk: String,
    pub kubernetes: KubernetesInfo,
    pub cgroups: CgroupInfo,
    pub systemd: SystemdInfo,
    pub journal: JournalInfo,
    pub containers: Vec<ContainerInfo>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KubernetesInfo {
    pub namespace: Option<String>,
    pub pod_name: Option<String>,
    pub node_name: Option<String>,
    pub service_account: Option<String>,
    pub is_kubernetes: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CgroupInfo {
    pub version: String,
    pub controllers: Vec<String>,
    pub memory_limit: Option<String>,
    pub cpu_limit: Option<String>,
    pub cgroup_path: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemdInfo {
    pub units: Vec<SystemdUnit>,
    pub failed_units: Vec<String>,
    pub system_status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SystemdUnit {
    pub name: String,
    pub status: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JournalInfo {
    pub recent_errors: Vec<JournalEntry>,
    pub recent_warnings: Vec<JournalEntry>,
    pub boot_errors: Vec<JournalEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JournalEntry {
    pub timestamp: String,
    pub unit: String,
    pub message: String,
    pub priority: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: Vec<String>,
}

pub fn collect_basic_system_info() -> BasicSystemInfo {
    let (total_memory, free_memory) = get_memory_info();
    let (total_disk, free_disk) = get_disk_info();

    BasicSystemInfo {
        os: get_os_info(),
        cpu: get_cpu_info(),
        total_memory,
        free_memory,
        total_disk,
        free_disk,
        is_kubernetes: is_running_in_kubernetes(),
        container_runtime_available: is_container_runtime_available(),
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BasicSystemInfo {
    pub os: String,
    pub cpu: String,
    pub total_memory: String,
    pub free_memory: String,
    pub total_disk: String,
    pub free_disk: String,
    pub is_kubernetes: bool,
    pub container_runtime_available: bool,
}

// Lightweight check for Kubernetes environment (no external commands)
fn is_running_in_kubernetes() -> bool {
    // Check for Kubernetes environment variables
    std::env::var("KUBERNETES_SERVICE_HOST").is_ok() ||
    std::env::var("KUBERNETES_SERVICE_PORT").is_ok() ||
    // Check for mounted service account token
    std::path::Path::new("/var/run/secrets/kubernetes.io/serviceaccount/token").exists()
}

// Lightweight check for container runtime availability (no external commands)
fn is_container_runtime_available() -> bool {
    // Check if docker socket exists
    std::path::Path::new("/var/run/docker.sock").exists() ||
    // Check if containerd socket exists  
    std::path::Path::new("/run/containerd/containerd.sock").exists() ||
    // Check if docker binary exists in common paths
    std::path::Path::new("/usr/bin/docker").exists() ||
    std::path::Path::new("/usr/local/bin/docker").exists()
}

pub fn collect_system_info() -> SystemInfo {
    let (total_memory, free_memory) = get_memory_info();
    let (total_disk, free_disk) = get_disk_info();
    SystemInfo {
        os: get_os_info(),
        cpu: get_cpu_info(),
        total_memory,
        free_memory,
        total_disk,
        free_disk,
        kubernetes: collect_kubernetes_info(),
        cgroups: collect_cgroup_info(),
        systemd: collect_systemd_info(),
        journal: collect_journal_info(),
        containers: collect_container_info(),
    }
}

fn get_os_info() -> String {
    // Try to read from /etc/os-release first
    if let Ok(content) = std::fs::read_to_string("/etc/os-release") {
        let mut os_info = HashMap::new();

        for line in content.lines() {
            if let Some((key, value)) = line.split_once('=') {
                // Remove quotes from value
                let clean_value = value.trim_matches('"');
                os_info.insert(key.to_string(), clean_value.to_string());
            }
        }

        // Build a comprehensive OS string
        let mut os_string = String::new();

        // Use PRETTY_NAME if available, otherwise use NAME
        if let Some(pretty_name) = os_info.get("PRETTY_NAME") {
            os_string.push_str(pretty_name);
        } else if let Some(name) = os_info.get("NAME") {
            os_string.push_str(name);
        }

        // Add version information if available
        if let Some(version) = os_info.get("VERSION") {
            if !version.is_empty() {
                os_string.push_str(&format!(" {}", version));
            }
        }

        // Add build ID if available (useful for rolling releases)
        if let Some(build_id) = os_info.get("BUILD_ID") {
            if !build_id.is_empty() && build_id != "rolling" {
                os_string.push_str(&format!(" (Build: {})", build_id));
            }
        }

        // Add kernel version
        if let Ok(kernel_version) = std::fs::read_to_string("/proc/version") {
            if let Some(kernel_info) = kernel_version.split_whitespace().nth(2) {
                os_string.push_str(&format!(" [Kernel: {}]", kernel_info));
            }
        }

        if !os_string.is_empty() {
            return os_string;
        }
    }

    // Fallback to basic OS detection
    std::env::consts::OS.to_string()
}

fn get_cpu_info() -> String {
    // Try to get CPU info from /proc/cpuinfo
    if let Ok(content) = std::fs::read_to_string("/proc/cpuinfo") {
        for line in content.lines() {
            if line.starts_with("model name") {
                if let Some(cpu_name) = line.split(':').nth(1) {
                    return cpu_name.trim().to_string();
                }
            }
        }
    }
    "Unknown CPU".to_string()
}

fn get_memory_info() -> (String, String) {
    if let Ok(output) = std::process::Command::new("free").arg("-h").output() {
        let out = String::from_utf8_lossy(&output.stdout);
        for line in out.lines() {
            if line.starts_with("Mem:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    return (parts[1].to_string(), parts[3].to_string());
                }
            }
        }
    }
    ("unknown".to_string(), "unknown".to_string())
}

fn get_disk_info() -> (String, String) {
    if let Ok(output) = std::process::Command::new("df").args(["-h", "/"]).output() {
        let out = String::from_utf8_lossy(&output.stdout);
        for line in out.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                return (parts[1].to_string(), parts[3].to_string());
            }
        }
    }
    ("unknown".to_string(), "unknown".to_string())
}

fn collect_kubernetes_info() -> KubernetesInfo {
    let mut k8s_info = KubernetesInfo {
        namespace: None,
        pod_name: None,
        node_name: None,
        service_account: None,
        is_kubernetes: false,
    };

    // Check if we're running in Kubernetes by looking for service account token
    if std::path::Path::new("/var/run/secrets/kubernetes.io/serviceaccount/token").exists() {
        k8s_info.is_kubernetes = true;

        // Try to get namespace
        if let Ok(namespace) =
            std::fs::read_to_string("/var/run/secrets/kubernetes.io/serviceaccount/namespace")
        {
            k8s_info.namespace = Some(namespace.trim().to_string());
        }

        // Try to get pod name from environment
        if let Ok(pod_name) = std::env::var("HOSTNAME") {
            k8s_info.pod_name = Some(pod_name);
        }

        // Try to get node name from environment
        if let Ok(node_name) = std::env::var("NODE_NAME") {
            k8s_info.node_name = Some(node_name);
        }

        // Try to get service account from environment
        if let Ok(sa) = std::env::var("SERVICE_ACCOUNT") {
            k8s_info.service_account = Some(sa);
        }
    }

    k8s_info
}

fn collect_cgroup_info() -> CgroupInfo {
    let mut cgroup_info = CgroupInfo {
        version: "unknown".to_string(),
        controllers: Vec::new(),
        memory_limit: None,
        cpu_limit: None,
        cgroup_path: "unknown".to_string(),
    };

    // Try to get cgroup version and path
    if let Ok(content) = std::fs::read_to_string("/proc/self/cgroup") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() >= 3 {
                cgroup_info.version = if parts[0] == "0" {
                    "v1".to_string()
                } else {
                    "v2".to_string()
                };
                cgroup_info.controllers = parts[1].split(',').map(|s| s.to_string()).collect();
                cgroup_info.cgroup_path = parts[2].to_string();
                break;
            }
        }
    }

    // Try to get memory limit
    if let Ok(content) = std::fs::read_to_string("/sys/fs/cgroup/memory/memory.limit_in_bytes") {
        cgroup_info.memory_limit = Some(content.trim().to_string());
    } else if let Ok(content) = std::fs::read_to_string("/sys/fs/cgroup/memory.max") {
        cgroup_info.memory_limit = Some(content.trim().to_string());
    }

    // Try to get CPU limit
    if let Ok(content) = std::fs::read_to_string("/sys/fs/cgroup/cpu/cpu.cfs_quota_us") {
        cgroup_info.cpu_limit = Some(content.trim().to_string());
    } else if let Ok(content) = std::fs::read_to_string("/sys/fs/cgroup/cpu.max") {
        cgroup_info.cpu_limit = Some(content.trim().to_string());
    }

    cgroup_info
}

fn collect_systemd_info() -> SystemdInfo {
    let mut systemd_info = SystemdInfo {
        units: Vec::new(),
        failed_units: Vec::new(),
        system_status: "unknown".to_string(),
    };

    // Get system status
    if let Ok(output) = Command::new("systemctl").arg("is-system-running").output() {
        systemd_info.system_status = String::from_utf8_lossy(&output.stdout).trim().to_string();
    }

    // Get failed units
    if let Ok(output) = Command::new("systemctl")
        .args(["--failed", "--no-pager", "--no-legend"])
        .output()
    {
        for line in String::from_utf8_lossy(&output.stdout).lines() {
            if !line.trim().is_empty() {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    systemd_info.failed_units.push(parts[0].to_string());
                }
            }
        }
    }

    // Get some important units
    let important_units = ["docker", "containerd", "kubelet", "kube-proxy"];
    for unit in important_units {
        if let Ok(output) = Command::new("systemctl")
            .args(["show", unit, "--property=ActiveState,Description"])
            .output()
        {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut status = "unknown".to_string();
            let mut description = "".to_string();

            for line in output_str.lines() {
                if line.starts_with("ActiveState=") {
                    status = line.split('=').nth(1).unwrap_or("unknown").to_string();
                } else if line.starts_with("Description=") {
                    description = line.split('=').nth(1).unwrap_or("").to_string();
                }
            }

            systemd_info.units.push(SystemdUnit {
                name: unit.to_string(),
                status,
                description,
            });
        }
    }

    systemd_info
}

fn collect_journal_info() -> JournalInfo {
    let mut journal_info = JournalInfo {
        recent_errors: Vec::new(),
        recent_warnings: Vec::new(),
        boot_errors: Vec::new(),
    };

    // Get recent errors (last 50 entries)
    if let Ok(output) = Command::new("journalctl")
        .args(["-p", "err", "--no-pager", "--no-hostname", "-n", "50"])
        .output()
    {
        journal_info.recent_errors = parse_journal_output(&output.stdout);
    }

    // Get recent warnings (last 50 entries)
    if let Ok(output) = Command::new("journalctl")
        .args(["-p", "warning", "--no-pager", "--no-hostname", "-n", "50"])
        .output()
    {
        journal_info.recent_warnings = parse_journal_output(&output.stdout);
    }

    // Get boot errors
    if let Ok(output) = Command::new("journalctl")
        .args(["-p", "err", "--no-pager", "--no-hostname", "-b"])
        .output()
    {
        journal_info.boot_errors = parse_journal_output(&output.stdout);
    }

    journal_info
}

fn parse_journal_output(output: &[u8]) -> Vec<JournalEntry> {
    let mut entries = Vec::new();
    let output_str = String::from_utf8_lossy(output);
    let mut current_entry: Option<JournalEntry> = None;

    for line in output_str.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Skip boot/reboot markers - they're not actual log entries
        if trimmed.starts_with("-- Boot")
            || trimmed.starts_with("-- Reboot")
            || trimmed.starts_with("--")
        {
            continue;
        }

        // Try to parse a new log entry
        let mut parts = trimmed.splitn(4, ' ');
        let month = parts.next().unwrap_or("");
        let day = parts.next().unwrap_or("");
        let time = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("");
        let timestamp = format!("{} {} {}", month, day, time);

        if !month.is_empty()
            && !day.is_empty()
            && !time.is_empty()
            && !rest.is_empty()
            && rest.contains(':')
        {
            // If we have a current entry, push it before starting a new one
            if let Some(entry) = current_entry.take() {
                // Only add entries that have actual message content
                if !entry.message.trim().is_empty() {
                    entries.push(entry);
                }
            }

            let colon_pos = rest.find(':').unwrap();
            let unit = rest[..colon_pos].trim();
            let message = rest[colon_pos + 1..].trim().to_string();

            // Only create entry if there's actual message content
            if !message.is_empty() {
                current_entry = Some(JournalEntry {
                    timestamp,
                    unit: unit.to_string(),
                    message,
                    priority: "unknown".to_string(),
                });
            }
        } else {
            // This is a continuation line, append to the current entry's message
            if let Some(ref mut entry) = current_entry {
                if !entry.message.is_empty() {
                    entry.message.push(' ');
                }
                entry.message.push_str(trimmed);
            }
        }
    }

    // Push the last entry if any and it has content
    if let Some(entry) = current_entry {
        if !entry.message.trim().is_empty() {
            entries.push(entry);
        }
    }

    entries
}

fn collect_container_info() -> Vec<ContainerInfo> {
    let mut containers = Vec::new();

    // Try to get Docker containers
    if let Ok(output) = Command::new("docker")
        .args([
            "ps",
            "--format",
            "table {{.ID}}\t{{.Names}}\t{{.Image}}\t{{.Status}}\t{{.Ports}}",
        ])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(1) {
            // Skip header
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 5 {
                containers.push(ContainerInfo {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                    image: parts[2].to_string(),
                    status: parts[3].to_string(),
                    ports: parts[4].split(',').map(|s| s.trim().to_string()).collect(),
                });
            }
        }
    }

    // Try to get containerd containers
    if let Ok(output) = Command::new("crictl")
        .args(["ps", "--output", "table"])
        .output()
    {
        let output_str = String::from_utf8_lossy(&output.stdout);
        for line in output_str.lines().skip(1) {
            // Skip header
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                containers.push(ContainerInfo {
                    id: parts[0].to_string(),
                    name: parts[1].to_string(),
                    image: parts[2].to_string(),
                    status: parts[3].to_string(),
                    ports: Vec::new(), // crictl doesn't show ports by default
                });
            }
        }
    }

    containers
}

#[cfg(test)]
mod tests {
    use super::parse_journal_output;

    #[test]
    fn test_parse_journal_output_various_cases() {
        let input = r#"
-- Boot 12345 --
Jan 01 12:00:00 systemd: Started Session 1 of user root.
Jan 01 12:01:00 kernel: Initializing cgroup subsys cpuset
Jan 01 12:01:01 kernel: 
Jan 01 12:01:02 kernel: This is a long message
    that continues on the next line
    and even more
Jan 01 12:02:00 sshd: Accepted password for user from 1.2.3.4 port 22 ssh2
-- Reboot --
Jan 01 12:03:00 systemd: Stopping User Manager for UID 1000...
Malformed line without enough parts
Jan 01 12:04:00 kernel:Another message with no space after colon

"#;
        let entries = parse_journal_output(input.as_bytes());

        // Boot markers should be filtered out, but entries with content are included
        assert_eq!(entries.len(), 6); // All entries with actual content

        // First normal entry
        assert_eq!(entries[0].unit, "systemd");
        assert_eq!(entries[0].message, "Started Session 1 of user root.");

        // Kernel entry with content
        assert_eq!(entries[1].unit, "kernel");
        assert_eq!(entries[1].message, "Initializing cgroup subsys cpuset");

        // Wrapped/continued message
        assert_eq!(entries[2].unit, "kernel");
        assert_eq!(
            entries[2].message,
            "This is a long message that continues on the next line and even more"
        );

        // SSHD entry
        assert_eq!(entries[3].unit, "sshd");
        assert_eq!(
            entries[3].message,
            "Accepted password for user from 1.2.3.4 port 22 ssh2"
        );

        // systemd stopping entry with malformed line appended
        assert_eq!(entries[4].unit, "systemd");
        assert!(
            entries[4].message.contains(
                "Stopping User Manager for UID 1000... Malformed line without enough parts"
            )
        );

        // kernel entry with no space after colon
        assert_eq!(entries[5].unit, "kernel");
        assert_eq!(
            entries[5].message,
            "Another message with no space after colon"
        );

        // Note: Empty kernel message and reboot marker are filtered out
    }
}
