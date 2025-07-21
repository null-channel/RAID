use crate::sysinfo::SystemInfo;
use serde::{Deserialize, Serialize};

pub mod printers;

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
    let has_significant_errors = system_info
        .journal
        .recent_errors
        .iter()
        .any(|entry| !printers::is_common_non_critical_error(&entry.message))
        || system_info
            .journal
            .boot_errors
            .iter()
            .any(|entry| !printers::is_common_non_critical_error(&entry.message));
    let has_container_issues = system_info
        .containers
        .iter()
        .any(|container| !container.status.contains("Up"));

    // Determine overall status
    let overall_status = if !has_failed_services && !has_significant_errors && !has_container_issues
    {
        "healthy".to_string()
    } else if has_failed_services {
        "critical".to_string()
    } else {
        "warning".to_string()
    };

    // Build service status
    let service_status = ServiceStatus {
        status: if has_failed_services {
            "critical".to_string()
        } else {
            "healthy".to_string()
        },
        failed_units: system_info.systemd.failed_units.clone(),
        total_units: system_info.systemd.units.len(),
        failed_count: system_info.systemd.failed_units.len(),
    };

    // Build log status
    let significant_errors: Vec<LogEntry> = system_info
        .journal
        .recent_errors
        .iter()
        .filter(|entry| !printers::is_common_non_critical_error(&entry.message))
        .map(|entry| LogEntry {
            timestamp: entry.timestamp.clone(),
            unit: entry.unit.clone(),
            message: entry.message.clone(),
            priority: entry.priority.clone(),
        })
        .collect();

    let significant_boot_errors: Vec<LogEntry> = system_info
        .journal
        .boot_errors
        .iter()
        .filter(|entry| !printers::is_common_non_critical_error(&entry.message))
        .map(|entry| LogEntry {
            timestamp: entry.timestamp.clone(),
            unit: entry.unit.clone(),
            message: entry.message.clone(),
            priority: entry.priority.clone(),
        })
        .collect();

    let log_status = LogStatus {
        status: if has_significant_errors {
            "warning".to_string()
        } else {
            "healthy".to_string()
        },
        recent_errors: significant_errors.clone(),
        boot_errors: significant_boot_errors.clone(),
        total_errors: significant_errors.len() + significant_boot_errors.len(),
    };

    // Build container status
    let healthy_containers = system_info
        .containers
        .iter()
        .filter(|c| c.status.contains("Up"))
        .count();
    let container_status = ContainerStatus {
        status: if has_container_issues {
            "warning".to_string()
        } else {
            "healthy".to_string()
        },
        containers: system_info
            .containers
            .iter()
            .map(|c| ContainerInfo {
                name: c.name.clone(),
                status: c.status.clone(),
                ports: c.ports.clone(),
            })
            .collect(),
        healthy_count: healthy_containers,
        unhealthy_count: system_info.containers.len() - healthy_containers,
        total_count: system_info.containers.len(),
    };

    let status = SystemStatus {
        overall: overall_status,
        services: service_status,
        logs: log_status,
        containers: container_status,
    };

    // Build issues list
    let mut issues = Vec::new();

    // Add service issues
    for failed_unit in &system_info.systemd.failed_units {
        issues.push(Issue {
            category: "service".to_string(),
            severity: "high".to_string(),
            message: format!("Service '{}' has failed", failed_unit),
            details: None,
        });
    }

    // Add log issues
    for entry in &significant_errors {
        issues.push(Issue {
            category: "log".to_string(),
            severity: "medium".to_string(),
            message: format!("Error in {}: {}", entry.unit, entry.message),
            details: Some(entry.timestamp.clone()),
        });
    }

    // Add container issues
    for container in &system_info.containers {
        if !container.status.contains("Up") {
            issues.push(Issue {
                category: "container".to_string(),
                severity: "medium".to_string(),
                message: format!("Container '{}' is not running: {}", container.name, container.status),
                details: None,
            });
        }
    }

    SystemHealthReport {
        timestamp,
        system_info: system_info.clone(),
        analysis: analysis.to_string(),
        status,
        issues,
    }
}

pub fn print_json(report: &SystemHealthReport) {
    let json = serde_json::to_string_pretty(report).unwrap_or_else(|e| {
        format!("Error serializing to JSON: {}", e)
    });
    println!("{}", json);
}

pub fn print_yaml(report: &SystemHealthReport) {
    let yaml = serde_yaml::to_string(report).unwrap_or_else(|e| {
        format!("Error serializing to YAML: {}", e)
    });
    println!("{}", yaml);
} 