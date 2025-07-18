use serde::{Deserialize, Serialize};
use crate::sysinfo::SystemInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemHealthReport {
    pub timestamp: String,
    pub system_info: SystemInfo,
    pub analysis: String,
    pub status: SystemStatus,
    pub issues: Vec<Issue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub overall: String, // "healthy", "warning", "critical"
    pub services: ServiceStatus,
    pub logs: LogStatus,
    pub containers: ContainerStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub status: String, // "healthy", "warning", "critical"
    pub failed_units: Vec<String>,
    pub total_units: usize,
    pub failed_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogStatus {
    pub status: String, // "healthy", "warning", "critical"
    pub recent_errors: Vec<LogEntry>,
    pub boot_errors: Vec<LogEntry>,
    pub total_errors: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub unit: String,
    pub message: String,
    pub priority: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerStatus {
    pub status: String, // "healthy", "warning", "critical"
    pub containers: Vec<ContainerInfo>,
    pub healthy_count: usize,
    pub unhealthy_count: usize,
    pub total_count: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerInfo {
    pub name: String,
    pub status: String,
    pub ports: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Issue {
    pub category: String, // "service", "log", "container", "system"
    pub severity: String, // "low", "medium", "high", "critical"
    pub message: String,
    pub details: Option<String>,
}

pub fn create_system_health_report(
    system_info: &SystemInfo,
    analysis: &str,
    verbose: bool,
) -> SystemHealthReport {
    let timestamp = chrono::Utc::now().to_rfc3339();
    
    // Analyze system status
    let has_failed_services = !system_info.systemd.failed_units.is_empty();
    let has_significant_errors = system_info.journal.recent_errors.iter()
        .any(|entry| !is_common_non_critical_error(&entry.message)) ||
        system_info.journal.boot_errors.iter()
        .any(|entry| !is_common_non_critical_error(&entry.message));
    let has_container_issues = system_info.containers.iter()
        .any(|container| !container.status.contains("Up"));

    // Determine overall status
    let overall_status = if !has_failed_services && !has_significant_errors && !has_container_issues {
        "healthy".to_string()
    } else if has_failed_services {
        "critical".to_string()
    } else {
        "warning".to_string()
    };

    // Create service status
    let service_status = ServiceStatus {
        status: if has_failed_services { "critical".to_string() } else { "healthy".to_string() },
        failed_units: system_info.systemd.failed_units.clone(),
        total_units: system_info.systemd.units.len(),
        failed_count: system_info.systemd.failed_units.len(),
    };

    // Create log status based on verbose mode
    let (recent_errors, boot_errors) = if verbose {
        // In verbose mode, include ALL logs
        let all_recent_errors: Vec<LogEntry> = system_info.journal.recent_errors.iter()
            .map(|entry| LogEntry {
                timestamp: entry.timestamp.clone(),
                unit: entry.unit.clone(),
                message: entry.message.clone(),
                priority: entry.priority.clone(),
            })
            .collect();

        let all_boot_errors: Vec<LogEntry> = system_info.journal.boot_errors.iter()
            .map(|entry| LogEntry {
                timestamp: entry.timestamp.clone(),
                unit: entry.unit.clone(),
                message: entry.message.clone(),
                priority: entry.priority.clone(),
            })
            .collect();

        (all_recent_errors, all_boot_errors)
    } else {
        // In normal mode, only include significant errors
        let significant_recent_errors: Vec<LogEntry> = system_info.journal.recent_errors.iter()
            .filter(|entry| !is_common_non_critical_error(&entry.message))
            .map(|entry| LogEntry {
                timestamp: entry.timestamp.clone(),
                unit: entry.unit.clone(),
                message: entry.message.clone(),
                priority: entry.priority.clone(),
            })
            .collect();

        let significant_boot_errors: Vec<LogEntry> = system_info.journal.boot_errors.iter()
            .filter(|entry| !is_common_non_critical_error(&entry.message))
            .map(|entry| LogEntry {
                timestamp: entry.timestamp.clone(),
                unit: entry.unit.clone(),
                message: entry.message.clone(),
                priority: entry.priority.clone(),
            })
            .collect();

        (significant_recent_errors, significant_boot_errors)
    };

    let log_status = LogStatus {
        status: if !recent_errors.is_empty() || !boot_errors.is_empty() {
            "warning".to_string()
        } else {
            "healthy".to_string()
        },
        recent_errors: recent_errors.clone(),
        boot_errors: boot_errors.clone(),
        total_errors: recent_errors.len() + boot_errors.len(),
    };

    // Create container status
    let healthy_containers: Vec<ContainerInfo> = system_info.containers.iter()
        .filter(|c| c.status.contains("Up"))
        .map(|c| ContainerInfo {
            name: c.name.clone(),
            status: c.status.clone(),
            ports: c.ports.clone(),
        })
        .collect();

    let unhealthy_containers: Vec<ContainerInfo> = system_info.containers.iter()
        .filter(|c| !c.status.contains("Up"))
        .map(|c| ContainerInfo {
            name: c.name.clone(),
            status: c.status.clone(),
            ports: c.ports.clone(),
        })
        .collect();

    let container_status = ContainerStatus {
        status: if !unhealthy_containers.is_empty() {
            "warning".to_string()
        } else if !system_info.containers.is_empty() {
            "healthy".to_string()
        } else {
            "unknown".to_string()
        },
        containers: system_info.containers.iter().map(|c| ContainerInfo {
            name: c.name.clone(),
            status: c.status.clone(),
            ports: c.ports.clone(),
        }).collect(),
        healthy_count: healthy_containers.len(),
        unhealthy_count: unhealthy_containers.len(),
        total_count: system_info.containers.len(),
    };

    // Create issues list
    let mut issues = Vec::new();
    
    if has_failed_services {
        issues.push(Issue {
            category: "service".to_string(),
            severity: "critical".to_string(),
            message: format!("{} failed systemd units", system_info.systemd.failed_units.len()),
            details: Some(format!("Failed units: {}", system_info.systemd.failed_units.join(", "))),
        });
    }

    if !recent_errors.is_empty() || !boot_errors.is_empty() {
        issues.push(Issue {
            category: "log".to_string(),
            severity: "warning".to_string(),
            message: format!("{} significant system errors", log_status.total_errors),
            details: None,
        });
    }

    if !unhealthy_containers.is_empty() {
        issues.push(Issue {
            category: "container".to_string(),
            severity: "warning".to_string(),
            message: format!("{} unhealthy containers", unhealthy_containers.len()),
            details: Some(format!("Unhealthy containers: {}", 
                unhealthy_containers.iter().map(|c| c.name.as_str()).collect::<Vec<_>>().join(", "))),
        });
    }

    SystemHealthReport {
        timestamp,
        system_info: system_info.clone(),
        analysis: analysis.to_string(),
        status: SystemStatus {
            overall: overall_status,
            services: service_status,
            logs: log_status,
            containers: container_status,
        },
        issues,
    }
}

pub fn print_yaml(report: &SystemHealthReport) {
    let yaml = serde_yaml::to_string(report).unwrap();
    println!("{}", yaml);
}

pub fn print_json(report: &SystemHealthReport) {
    let json = serde_json::to_string_pretty(report).unwrap();
    println!("{}", json);
}

fn is_common_non_critical_error(message: &str) -> bool {
    let common_errors = [
        "dmidecode",
        "environment.d",
        "invalid variable name",
        "gkr-pam",
        "daemon control file",
        "ACPI BIOS Error",
        "ACPI Error",
        "hub config failed",
        "Unknown group",
        "plugdev",
        "udev rules",
        "dbus-broker-launch",
        "nm_dispatcher",
        "watchdog did not stop",
        "could not resolve symbol",
        "ae_not_found",
        "hub doesn't have any ports",
        "bluetooth: hci0: no support for _prr acpi method",
        "cannot get freq at ep",
        "gdm: failed to list cached users",
        "gdbus.error:org.freedesktop.dbus.error.serviceunknown",
        "davincipanel.rules",
    ];
    
    let message_lower = message.to_lowercase();
    common_errors.iter().any(|error| message_lower.contains(error))
} 