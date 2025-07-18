use crate::sysinfo::SystemInfo;

pub fn print_results(info: &SystemInfo, analysis: &str, verbose: bool) {
    println!("🔍 System Health Check");
    println!("{}", "=".repeat(50));
    
    // System Overview
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
    
    if !info.containers.is_empty() {
        println!("🐳 Containers: {} running", info.containers.len());
    }

    // Determine if there are real issues
    let has_failed_services = !info.systemd.failed_units.is_empty();
    let _has_significant_errors = info.journal.recent_errors.iter()
        .any(|entry| !is_common_non_critical_error(&entry.message)) ||
        info.journal.boot_errors.iter()
        .any(|entry| !is_common_non_critical_error(&entry.message));
    let has_container_issues = info.containers.iter()
        .any(|container| !container.status.contains("Up"));

    let mut anything_printed = false;

    // Service Status
    if has_failed_services {
        println!("\n❌ Service Issues");
        println!("{}", "-".repeat(30));
        for unit in &info.systemd.failed_units {
            println!("  🔴 {}", unit);
        }
        anything_printed = true;
    } else {
        println!("\n✅ Services");
        println!("{}", "-".repeat(30));
        println!("  All systemd services are running");
    }

    // System Logs
    if verbose {
        // In verbose mode, show ALL logs
        let total_recent_errors = info.journal.recent_errors.len();
        let total_boot_errors = info.journal.boot_errors.len();
        
        if total_recent_errors > 0 || total_boot_errors > 0 {
            println!("\n📋 System Logs (Verbose Mode - All Entries)");
            println!("{}", "-".repeat(30));
            
            // Show all recent errors
            if total_recent_errors > 0 {
                println!("Recent Errors ({}):", total_recent_errors);
                for entry in &info.journal.recent_errors {
                    println!("  🔴 [{}] {}: {}", entry.timestamp, entry.unit, entry.message);
                }
            }
            
            // Show all boot errors
            if total_boot_errors > 0 {
                println!("Boot Errors ({}):", total_boot_errors);
                for entry in &info.journal.boot_errors {
                    println!("  🔄 [BOOT] {}: {}", entry.unit, entry.message);
                }
            }
            anything_printed = true;
        } else {
            println!("\n✅ System Logs");
            println!("{}", "-".repeat(30));
            println!("  No errors found");
        }
    } else {
        // In normal mode, filter logs as before
        let mut error_count = 0;
        let mut boot_error_count = 0;
        
        // Count significant errors
        for entry in &info.journal.recent_errors {
            if !is_common_non_critical_error(&entry.message) {
                error_count += 1;
            }
        }
        for entry in &info.journal.boot_errors {
            if !is_common_non_critical_error(&entry.message) {
                boot_error_count += 1;
            }
        }

        if error_count > 0 || boot_error_count > 0 {
            println!("\n⚠️  System Logs");
            println!("{}", "-".repeat(30));
            
            // Show recent errors
            for entry in &info.journal.recent_errors {
                if !is_common_non_critical_error(&entry.message) {
                    println!("  🔴 [{}] {}: {}", entry.timestamp, entry.unit, entry.message);
                }
            }
            
            // Show boot errors
            for entry in &info.journal.boot_errors {
                if !is_common_non_critical_error(&entry.message) {
                    println!("  🔄 [BOOT] {}: {}", entry.unit, entry.message);
                }
            }
            anything_printed = true;
        } else {
            println!("\n✅ System Logs");
            println!("{}", "-".repeat(30));
            println!("  No significant errors found");
        }
    }

    // Container Status
    if !info.containers.is_empty() {
        println!("\n🐳 Container Status");
        println!("{}", "-".repeat(30));
        
        let mut healthy_containers = 0;
        let mut unhealthy_containers = 0;
        
        for container in &info.containers {
            if container.status.contains("Up") {
                healthy_containers += 1;
                if verbose {
                    println!("  ✅ {} ({})", container.name, container.status);
                }
            } else {
                unhealthy_containers += 1;
                println!("  ⚠️  {} ({})", container.name, container.status);
                if !container.ports.is_empty() {
                    println!("     Ports: {}", container.ports.join(", "));
                }
            }
        }
        
        if unhealthy_containers > 0 {
            anything_printed = true;
        }
        
        println!("  Summary: {}/{} containers healthy", healthy_containers, info.containers.len());
        
        // In verbose mode, show all healthy containers too
        if verbose && healthy_containers > 0 && unhealthy_containers == 0 {
            for container in &info.containers {
                if container.status.contains("Up") {
                    println!("  ✅ {} ({})", container.name, container.status);
                }
            }
        }
    }

    // Overall System Status
    println!("\n🎯 System Status");
    println!("{}", "-".repeat(30));
    
    if !anything_printed {
        println!("✅ System appears healthy");
        println!("   All services running");
        println!("   No significant errors");
        if !info.containers.is_empty() {
            println!("   All containers healthy");
        }
    } else {
        println!("⚠️  Issues detected");
        if has_failed_services {
            println!("   • {} failed services", info.systemd.failed_units.len());
        }
        if verbose {
            let total_errors = info.journal.recent_errors.len() + info.journal.boot_errors.len();
            if total_errors > 0 {
                println!("   • {} total system errors", total_errors);
            }
        } else {
            let error_count = info.journal.recent_errors.iter()
                .filter(|entry| !is_common_non_critical_error(&entry.message))
                .count();
            let boot_error_count = info.journal.boot_errors.iter()
                .filter(|entry| !is_common_non_critical_error(&entry.message))
                .count();
            if error_count > 0 || boot_error_count > 0 {
                println!("   • {} significant system errors", error_count + boot_error_count);
            }
        }
        if has_container_issues {
            let unhealthy_count = info.containers.iter()
                .filter(|c| !c.status.contains("Up"))
                .count();
            println!("   • {} unhealthy containers", unhealthy_count);
        }
    }

    // AI Analysis
    println!("\n🤖 AI Analysis");
    println!("{}", "-".repeat(30));
    println!("{}", analysis);
    
    println!("\n{}", "=".repeat(50));
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

pub fn print_history(checks: &[(i64, String, SystemInfo, String)]) {
    println!("\n📚 Historical System Checks");
    println!("{}", "=".repeat(50));
    
    for (id, timestamp, system_info, analysis) in checks {
        println!("\n🔍 Check #{} - {}", id, timestamp);
        println!("{}", "-".repeat(40));
        println!("🖥️  OS: {}", system_info.os);
        println!("⚡ CPU: {}", system_info.cpu);
        println!("☸️  Kubernetes: {}", if system_info.kubernetes.is_kubernetes { "Yes" } else { "No" });
        
        // Determine if this check had issues
        let has_failed_services = !system_info.systemd.failed_units.is_empty();
        let has_significant_errors = system_info.journal.recent_errors.iter()
            .any(|entry| !is_common_non_critical_error(&entry.message)) ||
            system_info.journal.boot_errors.iter()
            .any(|entry| !is_common_non_critical_error(&entry.message));
        let has_container_issues = system_info.containers.iter()
            .any(|container| !container.status.contains("Up"));
        
        let status_icon = if !has_failed_services && !has_significant_errors && !has_container_issues {
            "✅"
        } else {
            "⚠️"
        };
        
        println!("{} Status: {}", status_icon, if !has_failed_services && !has_significant_errors && !has_container_issues { "Healthy" } else { "Issues Detected" });
        println!("🤖 Analysis: {}", analysis);
    }
} 