use crate::cli::OutputFormat;
use crate::config::RaidConfig;
use crate::output::{create_system_health_report, print_json, print_yaml};
use crate::sysinfo::SystemInfo;
use crate::ui::{print_results, print_results_with_formatter, UIFormatter};

pub fn print_output(
    system_info: &SystemInfo,
    analysis: &str,
    output_format: &OutputFormat,
    verbose: bool,
) {
    match output_format {
        OutputFormat::Text => {
            print_results(system_info, analysis, verbose);
        }
        OutputFormat::Yaml => {
            let report = create_system_health_report(system_info, analysis, verbose);
            print_yaml(&report);
        }
        OutputFormat::Json => {
            let report = create_system_health_report(system_info, analysis, verbose);
            print_json(&report);
        }
    }
}

pub fn print_output_with_config(
    system_info: &SystemInfo,
    analysis: &str,
    config: &RaidConfig,
    ui_formatter: &UIFormatter,
) {
    match config.get_output_format() {
        OutputFormat::Text => {
            print_results_with_formatter(system_info, analysis, config.output.verbose, ui_formatter);
        }
        OutputFormat::Yaml => {
            let report = create_system_health_report(system_info, analysis, config.output.verbose);
            print_yaml(&report);
        }
        OutputFormat::Json => {
            let report = create_system_health_report(system_info, analysis, config.output.verbose);
            print_json(&report);
        }
    }
}

pub fn print_system_info(info: &SystemInfo, analysis: &str, verbose: bool) {
    println!("🔍 System Information");
    println!("{}", "=".repeat(50));

    println!("\n📊 System Overview");
    println!("{}", "-".repeat(30));
    println!("🖥️  OS: {}", info.os);
    println!("⚡ CPU: {}", info.cpu);
    println!("💾 Memory: {}/{}", info.free_memory, info.total_memory);
    println!("💿 Disk: {}/{}", info.free_disk, info.total_disk);

    if info.kubernetes.is_kubernetes {
        println!("☸️  Kubernetes: Yes");
        if let Some(namespace) = &info.kubernetes.namespace {
            println!("   Namespace: {}", namespace);
        }
        if let Some(pod_name) = &info.kubernetes.pod_name {
            println!("   Pod: {}", pod_name);
        }
    } else {
        println!("☸️  Kubernetes: No");
    }

    if verbose {
        println!("\n📋 Verbose System Details");
        println!("{}", "-".repeat(30));
        // In verbose mode, show additional system details
        if !info.systemd.units.is_empty() {
            println!("System Services:");
            for unit in &info.systemd.units {
                let status_icon = if unit.status == "active" {
                    "✅"
                } else {
                    "⚠️"
                };
                println!("  {} {}: {}", status_icon, unit.name, unit.status);
            }
        }

        if !info.cgroups.controllers.is_empty() {
            println!(
                "Cgroup Controllers: {}",
                info.cgroups.controllers.join(", ")
            );
        }
    }

    println!("\n🤖 AI Analysis");
    println!("{}", "-".repeat(30));
    println!("{}", analysis);

    println!("\n{}", "=".repeat(50));
}

pub fn print_container_info(info: &SystemInfo, analysis: &str, verbose: bool) {
    println!("=== Container Status ===");
    if info.containers.is_empty() {
        println!("No containers found");
    } else {
        for container in &info.containers {
            let status_icon = if container.status.contains("Up") {
                "✅"
            } else {
                "⚠️"
            };

            // In normal mode, only show containers with issues
            // In verbose mode, show all containers
            if verbose || !container.status.contains("Up") {
                println!(
                    "  {} {} ({})",
                    status_icon, container.name, container.status
                );
                if !container.ports.is_empty() {
                    println!("    Ports: {}", container.ports.join(", "));
                }
                if verbose {
                    println!("    Image: {}", container.image);
                    println!("    ID: {}", container.id);
                }
            }
        }

        if !verbose {
            let healthy_count = info
                .containers
                .iter()
                .filter(|c| c.status.contains("Up"))
                .count();
            let unhealthy_count = info.containers.len() - healthy_count;
            if unhealthy_count == 0 {
                println!("  ✅ All {} containers are healthy", info.containers.len());
            } else {
                println!(
                    "  Summary: {}/{} containers healthy",
                    healthy_count,
                    info.containers.len()
                );
            }
        }
    }
    println!("\n=== AI Analysis ===");
    println!("{}", analysis);
}

pub fn print_kubernetes_info(info: &SystemInfo, analysis: &str, verbose: bool) {
    println!("=== Kubernetes Information ===");
    if info.kubernetes.is_kubernetes {
        println!("Running in Kubernetes: Yes");
        if let Some(namespace) = &info.kubernetes.namespace {
            println!("Namespace: {}", namespace);
        }
        if let Some(pod_name) = &info.kubernetes.pod_name {
            println!("Pod Name: {}", pod_name);
        }
        if let Some(node_name) = &info.kubernetes.node_name {
            println!("Node Name: {}", node_name);
        }
        if let Some(sa) = &info.kubernetes.service_account {
            println!("Service Account: {}", sa);
        }

        if verbose {
            println!("\nAdditional K8s Details:");
            println!("Cgroup Version: {}", info.cgroups.version);
            if let Some(memory_limit) = &info.cgroups.memory_limit {
                println!("Memory Limit: {}", memory_limit);
            }
            if let Some(cpu_limit) = &info.cgroups.cpu_limit {
                println!("CPU Limit: {}", cpu_limit);
            }
        }
    } else {
        println!("Running in Kubernetes: No");
    }
    println!("\n=== AI Analysis ===");
    println!("{}", analysis);
}

pub fn print_cgroup_info(info: &SystemInfo, analysis: &str, verbose: bool) {
    println!("=== Cgroup Information ===");
    println!("Version: {}", info.cgroups.version);
    println!("Path: {}", info.cgroups.cgroup_path);

    if verbose || !info.cgroups.controllers.is_empty() {
        println!("Controllers: {}", info.cgroups.controllers.join(", "));
    }

    if let Some(memory_limit) = &info.cgroups.memory_limit {
        println!("Memory Limit: {}", memory_limit);
    }
    if let Some(cpu_limit) = &info.cgroups.cpu_limit {
        println!("CPU Limit: {}", cpu_limit);
    }

    if verbose {
        println!("\nVerbose Cgroup Details:");
        println!("Full cgroup path: {}", info.cgroups.cgroup_path);
    }

    println!("\n=== AI Analysis ===");
    println!("{}", analysis);
}

pub fn print_systemd_info(info: &SystemInfo, analysis: &str, verbose: bool) {
    println!("=== Service Status ===");
    println!("System Status: {}", info.systemd.system_status);

    if !info.systemd.failed_units.is_empty() {
        println!("Failed Units:");
        for unit in &info.systemd.failed_units {
            println!("  ❌ {}", unit);
        }
    }

    // Show units based on verbose mode
    if verbose {
        // In verbose mode, show all units
        println!("All Monitored Units:");
        for unit in &info.systemd.units {
            let status_icon = if unit.status == "active" {
                "✅"
            } else {
                "⚠️"
            };
            println!(
                "  {} {}: {} - {}",
                status_icon, unit.name, unit.status, unit.description
            );
        }
    } else {
        // In normal mode, only show units with issues
        let mut has_issues = false;
        for unit in &info.systemd.units {
            if unit.status != "active" {
                if !has_issues {
                    println!("Units with Issues:");
                    has_issues = true;
                }
                println!("  ⚠️  {}: {}", unit.name, unit.status);
            }
        }

        if !has_issues && info.systemd.failed_units.is_empty() {
            println!("✅ All services are running normally");
        }
    }

    println!("\n=== AI Analysis ===");
    println!("{}", analysis);
}

pub fn print_journal_info(info: &SystemInfo, analysis: &str, verbose: bool) {
    println!("=== System Logs ===");

    if verbose {
        // In verbose mode, show ALL logs
        if !info.journal.recent_errors.is_empty() {
            println!("All Recent Errors ({}):", info.journal.recent_errors.len());
            for entry in &info.journal.recent_errors {
                println!(
                    "  ❌ [{}] {}: {}",
                    entry.timestamp, entry.unit, entry.message
                );
            }
        }

        if !info.journal.boot_errors.is_empty() {
            println!("All Boot Errors ({}):", info.journal.boot_errors.len());
            for entry in &info.journal.boot_errors {
                println!("  🔄 [BOOT] {}: {}", entry.unit, entry.message);
            }
        }

        if !info.journal.recent_warnings.is_empty() {
            println!("Recent Warnings ({}):", info.journal.recent_warnings.len());
            for (i, entry) in info.journal.recent_warnings.iter().enumerate() {
                if i >= 10 {
                    // Limit warnings in verbose mode to avoid spam
                    println!(
                        "  ... and {} more warnings",
                        info.journal.recent_warnings.len() - i
                    );
                    break;
                }
                println!(
                    "  ⚠️  [{}] {}: {}",
                    entry.timestamp, entry.unit, entry.message
                );
            }
        }

        if info.journal.recent_errors.is_empty()
            && info.journal.boot_errors.is_empty()
            && info.journal.recent_warnings.is_empty()
        {
            println!("✅ No errors or warnings found");
        }
    } else {
        // In normal mode, show only significant errors
        let mut significant_errors = 0;
        for entry in &info.journal.recent_errors {
            if !is_common_non_critical_error(&entry.message) {
                if significant_errors == 0 {
                    println!("Recent Errors:");
                }
                println!(
                    "  ❌ [{}] {}: {}",
                    entry.timestamp, entry.unit, entry.message
                );
                significant_errors += 1;
                if significant_errors >= 5 {
                    break;
                }
            }
        }

        // Show boot errors only if significant
        let mut boot_error_count = 0;
        for entry in &info.journal.boot_errors {
            if !is_common_non_critical_error(&entry.message) {
                if boot_error_count == 0 {
                    println!("Boot Errors:");
                }
                println!("  🔄 [BOOT] {}: {}", entry.unit, entry.message);
                boot_error_count += 1;
                if boot_error_count >= 3 {
                    break;
                }
            }
        }

        if significant_errors == 0 && boot_error_count == 0 {
            println!("✅ No significant errors found");
        }
    }

    println!("\n=== AI Analysis ===");
    println!("{}", analysis);
}

// Dry-run versions of print functions (no AI analysis)
pub fn print_results_dry_run(info: &SystemInfo) {
    println!("=== System Health Check (Dry Run) ===");

    // Always show general system information
    println!("\n--- General System Information ---");
    println!("OS: {}", info.os);
    println!("CPU: {}", info.cpu);
    println!("Total Memory: {}", info.total_memory);
    println!("Free Memory: {}", info.free_memory);
    println!("Total Disk: {}", info.total_disk);
    println!("Free Disk: {}", info.free_disk);

    // Only show system info if there are actual issues
    let has_failed_services = !info.systemd.failed_units.is_empty();
    let has_significant_errors = info
        .journal
        .recent_errors
        .iter()
        .any(|entry| !is_common_non_critical_error(&entry.message))
        || info
            .journal
            .boot_errors
            .iter()
            .any(|entry| !is_common_non_critical_error(&entry.message));
    let has_container_issues = info
        .containers
        .iter()
        .any(|container| !container.status.contains("Up"));

    // Only show Kubernetes info if we're in K8s AND there are issues
    if info.kubernetes.is_kubernetes && (has_failed_services || has_significant_errors) {
        println!("\n=== Kubernetes Environment ===");
        println!("Running in Kubernetes: Yes");
        if let Some(namespace) = &info.kubernetes.namespace {
            println!("Namespace: {}", namespace);
        }
        if let Some(pod_name) = &info.kubernetes.pod_name {
            println!("Pod Name: {}", pod_name);
        }
        if let Some(node_name) = &info.kubernetes.node_name {
            println!("Node Name: {}", node_name);
        }
        if let Some(sa) = &info.kubernetes.service_account {
            println!("Service Account: {}", sa);
        }
    }

    // Only show cgroup info if there are limits AND issues
    if (info.cgroups.memory_limit.is_some() || info.cgroups.cpu_limit.is_some())
        && (has_failed_services || has_significant_errors)
    {
        println!("\n=== Resource Limits ===");
        println!("Cgroup Version: {}", info.cgroups.version);
        if let Some(memory_limit) = &info.cgroups.memory_limit {
            println!("Memory Limit: {}", memory_limit);
        }
        if let Some(cpu_limit) = &info.cgroups.cpu_limit {
            println!("CPU Limit: {}", cpu_limit);
        }
    }

    // Only show systemd info if there are failed units
    if !info.systemd.failed_units.is_empty() {
        println!("\n=== Service Status ===");
        println!("Failed Units:");
        for unit in &info.systemd.failed_units {
            println!("  ❌ {}", unit);
        }
    }

    // Only show journal info if there are significant errors
    let mut significant_errors = 0;
    for entry in &info.journal.recent_errors {
        if !is_common_non_critical_error(&entry.message) {
            if significant_errors == 0 {
                println!("\n=== System Logs ===");
            }
            println!(
                "  ❌ [{}] {}: {}",
                entry.timestamp, entry.unit, entry.message
            );
            significant_errors += 1;
            if significant_errors >= 3 {
                break;
            }
        }
    }

    // Only show container info if there are containers with issues
    if has_container_issues {
        println!("\n=== Container Status ===");
        for container in &info.containers {
            if !container.status.contains("Up") {
                let status_icon = if container.status.contains("Up") {
                    "✅"
                } else {
                    "⚠️"
                };
                println!(
                    "  {} {} ({})",
                    status_icon, container.name, container.status
                );
                if !container.ports.is_empty() {
                    println!("    Ports: {}", container.ports.join(", "));
                }
            }
        }
    }

    // If no issues found, show a clean message
    if !has_failed_services && !has_significant_errors && !has_container_issues {
        println!("✅ System appears healthy");
    }

    println!("\n=== DRY RUN MODE ===");
    println!("AI analysis skipped. Use without --dry-run flag for AI-powered insights.");
}

pub fn is_common_non_critical_error(message: &str) -> bool {
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
    common_errors
        .iter()
        .any(|error| message_lower.contains(error))
}

pub fn print_system_info_dry_run(info: &SystemInfo) {
    println!("=== System Information ===");
    println!("OS: {}", info.os);
    println!("CPU: {}", info.cpu);
    println!("\n=== DRY RUN MODE ===");
    println!("AI analysis skipped. Use without --dry-run flag for AI-powered insights.");
}

pub fn print_container_info_dry_run(info: &SystemInfo) {
    println!("=== Container Status ===");
    if info.containers.is_empty() {
        println!("No containers found");
    } else {
        for container in &info.containers {
            let status_icon = if container.status.contains("Up") {
                "✅"
            } else {
                "⚠️"
            };
            println!(
                "  {} {} ({})",
                status_icon, container.name, container.status
            );
            if !container.ports.is_empty() {
                println!("    Ports: {}", container.ports.join(", "));
            }
        }
    }
    println!("\n=== DRY RUN MODE ===");
    println!("AI analysis skipped. Use without --dry-run flag for AI-powered insights.");
}

pub fn print_kubernetes_info_dry_run(info: &SystemInfo) {
    println!("=== Kubernetes Information ===");
    if info.kubernetes.is_kubernetes {
        println!("Running in Kubernetes: Yes");
        if let Some(namespace) = &info.kubernetes.namespace {
            println!("Namespace: {}", namespace);
        }
        if let Some(pod_name) = &info.kubernetes.pod_name {
            println!("Pod Name: {}", pod_name);
        }
        if let Some(node_name) = &info.kubernetes.node_name {
            println!("Node Name: {}", node_name);
        }
        if let Some(sa) = &info.kubernetes.service_account {
            println!("Service Account: {}", sa);
        }
    } else {
        println!("Running in Kubernetes: No");
    }
    println!("\n=== DRY RUN MODE ===");
    println!("AI analysis skipped. Use without --dry-run flag for AI-powered insights.");
}

pub fn print_cgroup_info_dry_run(info: &SystemInfo) {
    println!("=== Cgroup Information ===");
    println!("Version: {}", info.cgroups.version);
    println!("Path: {}", info.cgroups.cgroup_path);
    println!("Controllers: {}", info.cgroups.controllers.join(", "));
    if let Some(memory_limit) = &info.cgroups.memory_limit {
        println!("Memory Limit: {}", memory_limit);
    }
    if let Some(cpu_limit) = &info.cgroups.cpu_limit {
        println!("CPU Limit: {}", cpu_limit);
    }
    println!("\n=== DRY RUN MODE ===");
    println!("AI analysis skipped. Use without --dry-run flag for AI-powered insights.");
}

pub fn print_systemd_info_dry_run(info: &SystemInfo) {
    println!("=== Service Status ===");
    println!("System Status: {}", info.systemd.system_status);

    if !info.systemd.failed_units.is_empty() {
        println!("Failed Units:");
        for unit in &info.systemd.failed_units {
            println!("  ❌ {}", unit);
        }
    }

    // Only show important units if they have issues
    let mut has_issues = false;
    for unit in &info.systemd.units {
        if unit.status != "active" {
            if !has_issues {
                println!("Units with Issues:");
                has_issues = true;
            }
            println!("  ⚠️  {}: {}", unit.name, unit.status);
        }
    }

    if !has_issues && info.systemd.failed_units.is_empty() {
        println!("✅ All services are running normally");
    }

    println!("\n=== DRY RUN MODE ===");
    println!("AI analysis skipped. Use without --dry-run flag for AI-powered insights.");
}

pub fn print_journal_info_dry_run(info: &SystemInfo) {
    println!("=== System Logs ===");

    // Show only significant errors
    let mut significant_errors = 0;
    for entry in &info.journal.recent_errors {
        if !is_common_non_critical_error(&entry.message) {
            if significant_errors == 0 {
                println!("Recent Errors:");
            }
            println!(
                "  ❌ [{}] {}: {}",
                entry.timestamp, entry.unit, entry.message
            );
            significant_errors += 1;
            if significant_errors >= 5 {
                break;
            }
        }
    }

    // Show boot errors only if significant
    let mut boot_error_count = 0;
    for entry in &info.journal.boot_errors {
        if !is_common_non_critical_error(&entry.message) {
            if boot_error_count == 0 {
                println!("Boot Errors:");
            }
            println!("  🔄 [BOOT] {}: {}", entry.unit, entry.message);
            boot_error_count += 1;
            if boot_error_count >= 3 {
                break;
            }
        }
    }

    if significant_errors == 0 && boot_error_count == 0 {
        println!("✅ No significant errors found");
    }

    println!("\n=== DRY RUN MODE ===");
    println!("AI analysis skipped. Use without --dry-run flag for AI-powered insights.");
} 