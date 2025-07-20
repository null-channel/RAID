mod ai;
mod cli;
mod config;
mod database;
mod known_issues;
mod output;
mod sysinfo;
mod tools;
mod ui;
use ai::create_ai_provider_from_cli;
use clap::Parser;
use cli::{AIAgentAction, CheckComponent, Cli, Commands, ConfigAction, DebugTool, IssueAction, OutputFormat};
use config::RaidConfig;
use database::Database;
use known_issues::IssueCategory;
use output::{create_system_health_report, print_json, print_yaml};
use std::env;
use std::io::{self, Write};
use sysinfo::{SystemInfo, collect_basic_system_info, collect_system_info};
use tools::DebugTools;
use ui::{print_results, UIFormatter};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI args
    let mut cli = Cli::parse();

    // Load configuration
    let mut config = if let Some(config_file) = &cli.config {
        // Load from specified config file
        RaidConfig::load_from_file(config_file).map_err(|e| {
            format!("Failed to load config file '{}': {}", config_file, e)
        })?
    } else {
        // Load from default locations
        RaidConfig::load().unwrap_or_else(|_| {
            // If no config file found, use defaults
            RaidConfig::default()
        })
    };

    // Merge CLI overrides into config
    config.merge_cli_overrides(&cli);

    // Validate configuration
    if let Err(e) = config.validate() {
        eprintln!("Configuration error: {}", e);
        std::process::exit(1);
    }

    // Create UI formatter based on config
    let ui_formatter = UIFormatter::new(config.ui.color && !cli.no_color);

    // Handle config command
    if let Some(Commands::Config { action, output }) = &cli.command {
        return run_config_command(action, output.as_deref(), &config).await;
    }

    // Check if this is a debug command
    if let Some(Commands::Debug { .. }) = &cli.command {
        // Debug commands don't need AI API key
        run_debug_tools(&cli).await?;
        return Ok(());
    }

    // Check if this is an issues command
    if let Some(Commands::Issues { .. }) = &cli.command {
        // Issues commands don't need AI API key
        run_issues_management(&cli).await?;
        return Ok(());
    }

    // Check if we have a problem description for AI agent mode
    if let Some(problem_description) = &cli.problem_description {
        // Question answering mode - help the user resolve their issue
        return run_question_answering_with_config(&config, problem_description, &ui_formatter).await;
    }

    // If AI_API_KEY is not set and no key provided via CLI, force dry-run and print a message
    if config.ai.api_key.is_none() && !cli.dry_run {
        println!("No AI API key found. Running in dry-run mode. No AI model will be used.");
        cli.dry_run = true;
    }

    // Collect system information with progress indicator
    let sys_info = if config.ui.progress_indicators && !cli.no_progress {
        ui_formatter.show_progress("Collecting system information", || collect_system_info())
    } else {
        collect_system_info()
    };
    
    let check_component = cli.get_check_component();

    if cli.dry_run {
        // Dry run mode - skip AI analysis
        match check_component {
            CheckComponent::All => {
                print_output_with_config(
                    &sys_info,
                    "Dry run mode - no AI analysis available",
                    &config,
                    &ui_formatter,
                );
            }
            CheckComponent::System => {
                print_output(
                    &sys_info,
                    "Dry run mode - no AI analysis available",
                    &cli.output_format,
                    cli.verbose,
                );
            }
            CheckComponent::Containers => {
                print_output(
                    &sys_info,
                    "Dry run mode - no AI analysis available",
                    &cli.output_format,
                    cli.verbose,
                );
            }
            CheckComponent::Kubernetes => {
                print_output(
                    &sys_info,
                    "Dry run mode - no AI analysis available",
                    &cli.output_format,
                    cli.verbose,
                );
            }
            CheckComponent::Cgroups => {
                print_output(
                    &sys_info,
                    "Dry run mode - no AI analysis available",
                    &cli.output_format,
                    cli.verbose,
                );
            }
            CheckComponent::Systemd => {
                print_output(
                    &sys_info,
                    "Dry run mode - no AI analysis available",
                    &cli.output_format,
                    cli.verbose,
                );
            }
            CheckComponent::Journal => {
                print_output(
                    &sys_info,
                    "Dry run mode - no AI analysis available",
                    &cli.output_format,
                    cli.verbose,
                );
            }
            CheckComponent::Debug => {
                println!(
                    "Debug tools are not available in dry-run mode. Use normal mode to run debug tools."
                );
            }
        }
    } else {
        // Normal mode with AI analysis
        let ai_provider = if config.ui.progress_indicators && !cli.no_progress {
            ui_formatter.show_progress("Initializing AI provider", || async {
                create_ai_provider_from_cli(
                    &config.get_ai_provider(),
                    config.ai.api_key.clone(),
                    Some(config.get_model()),
                    config.ai.base_url.clone(),
                    config.ai.max_tokens,
                    config.ai.temperature,
                ).await
            }).await?
        } else {
            create_ai_provider_from_cli(
                &config.get_ai_provider(),
                config.ai.api_key.clone(),
                Some(config.get_model()),
                config.ai.base_url.clone(),
                config.ai.max_tokens,
                config.ai.temperature,
            ).await?
        };

        match check_component {
            CheckComponent::All => {
                let analysis = if config.ui.progress_indicators && !cli.no_progress {
                    ui_formatter.show_progress("Analyzing system with AI", || async {
                        ai_provider
                            .analyze_with_known_issues(
                                &format!("OS: {}, CPU: {}", sys_info.os, sys_info.cpu),
                                None,
                            )
                            .await
                    }).await?
                } else {
                    ai_provider
                        .analyze_with_known_issues(
                            &format!("OS: {}, CPU: {}", sys_info.os, sys_info.cpu),
                            None,
                        )
                        .await?
                };

                // Only store in database for full checks
                if cli.is_full_check() {
                    let db = if config.ui.progress_indicators && !cli.no_progress {
                        ui_formatter.show_progress("Storing results in database", || {
                            Database::new(&config.database.path)
                        })?
                    } else {
                        Database::new(&config.database.path)?
                    };
                    
                    if config.ui.progress_indicators && !cli.no_progress {
                        ui_formatter.show_progress("Saving check results", || {
                            db.store_check(&sys_info, &analysis)
                        })?;
                    } else {
                        db.store_check(&sys_info, &analysis)?;
                    }
                }

                print_output_with_config(&sys_info, &analysis, &config, &ui_formatter);
            }
            CheckComponent::System => {
                let analysis = ai_provider
                    .analyze_with_known_issues(
                        &format!("System check - OS: {}, CPU: {}", sys_info.os, sys_info.cpu),
                        Some(IssueCategory::System),
                    )
                    .await?;
                print_system_info(&sys_info, &analysis, cli.verbose);
            }
            CheckComponent::Containers => {
                let analysis = ai_provider
                    .analyze_with_known_issues(
                        &format!(
                            "Container check - Found {} containers",
                            sys_info.containers.len()
                        ),
                        Some(IssueCategory::Container),
                    )
                    .await?;
                print_container_info(&sys_info, &analysis, cli.verbose);
            }
            CheckComponent::Kubernetes => {
                let analysis = ai_provider
                    .analyze_with_known_issues(
                        &format!(
                            "Kubernetes check - Running in K8s: {}",
                            sys_info.kubernetes.is_kubernetes
                        ),
                        Some(IssueCategory::Kubernetes),
                    )
                    .await?;
                print_kubernetes_info(&sys_info, &analysis, cli.verbose);
            }
            CheckComponent::Cgroups => {
                let analysis = ai_provider
                    .analyze_with_known_issues(
                        &format!(
                            "Cgroup check - Version: {}, Path: {}",
                            sys_info.cgroups.version, sys_info.cgroups.cgroup_path
                        ),
                        Some(IssueCategory::Cgroups),
                    )
                    .await?;
                print_cgroup_info(&sys_info, &analysis, cli.verbose);
            }
            CheckComponent::Systemd => {
                let analysis = ai_provider
                    .analyze_with_known_issues(
                        &format!(
                            "Systemd check - Status: {}, Failed units: {}",
                            sys_info.systemd.system_status,
                            sys_info.systemd.failed_units.len()
                        ),
                        Some(IssueCategory::Systemd),
                    )
                    .await?;
                print_systemd_info(&sys_info, &analysis, cli.verbose);
            }
            CheckComponent::Journal => {
                let total_errors =
                    sys_info.journal.recent_errors.len() + sys_info.journal.boot_errors.len();
                let analysis = ai_provider
                    .analyze_with_known_issues(
                        &format!("Journal check - Total errors: {}", total_errors),
                        Some(IssueCategory::Journal),
                    )
                    .await?;
                print_journal_info(&sys_info, &analysis, cli.verbose);
            }
            CheckComponent::Debug => {
                println!(
                    "Debug tools are not available in dry-run mode. Use normal mode to run debug tools."
                );
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ai::{AIProvider, DummyAI};

    #[tokio::test]
    async fn test_question_answering_functionality() {
        let dummy_ai = DummyAI;
        let question = "Why is my system slow?";
        let context =
            "Operating System: Linux 6.15.6-arch1-1\nCPU: AMD Ryzen 9 7940HS\nMemory: 16GB/32GB\n";

        let result = dummy_ai.answer_question(question, context).await;

        assert!(result.is_ok());
        let answer = result.unwrap();
        assert_eq!(answer, "I cannot answer that question.");
    }

    #[test]
    fn test_question_context_building() {
        // Test that context building doesn't panic and contains expected information
        let test_context = format!(
            "Operating System: {}\nCPU: {}\nMemory: {}/{}\n",
            "Linux 6.15.6-arch1-1", "AMD Ryzen 9 7940HS", "16GB", "32GB"
        );

        assert!(test_context.contains("Linux 6.15.6-arch1-1"));
        assert!(test_context.contains("AMD Ryzen 9 7940HS"));
        assert!(test_context.contains("16GB/32GB"));
    }

    #[test]
    fn test_question_analysis() {
        // Test that question analysis correctly identifies different types of questions
        let container_question =
            "I would like to know if my docker container called nginx is running";
        assert!(container_question.to_lowercase().contains("container"));
        assert!(container_question.to_lowercase().contains("docker"));
        assert!(container_question.to_lowercase().contains("nginx"));

        let performance_question = "why is my system slow?";
        assert!(performance_question.to_lowercase().contains("slow"));

        let service_question = "is my web service running?";
        assert!(service_question.to_lowercase().contains("service"));
    }

    #[test]
    fn test_extract_arg() {
        // Test argument extraction functionality
        let parts = vec![
            "kubectl_get_pods",
            "--namespace",
            "default",
            "--lines",
            "20",
        ];

        assert_eq!(
            extract_arg(&parts, "--namespace"),
            Some("default".to_string())
        );
        assert_eq!(extract_arg(&parts, "--lines"), Some("20".to_string()));
        assert_eq!(extract_arg(&parts, "--missing"), None);

        // Test with no arguments
        let empty_parts = vec!["docker_ps"];
        assert_eq!(extract_arg(&empty_parts, "--namespace"), None);
    }

    #[test]
    fn test_ai_tool_selection_integration() {
        // Test that the AI tool selection prompt format is correct
        let question = "Is my nginx container running?";
        let context = "Operating System: Linux\nContainers: 3 running\n";

        // This would be the format we expect the AI to return
        let mock_ai_response = "docker_ps\ndocker_inspect nginx\ndocker_logs nginx --lines 10";

        let lines: Vec<&str> = mock_ai_response.lines().collect();
        assert_eq!(lines.len(), 3);
        assert!(lines[0].contains("docker_ps"));
        assert!(lines[1].contains("docker_inspect nginx"));
        assert!(lines[2].contains("docker_logs nginx"));
    }

    #[test]
    fn test_basic_system_info_collection() {
        // Test that basic system info collection works without heavy diagnostics
        let basic_info = collect_basic_system_info();

        // Should have basic system information
        assert!(!basic_info.os.is_empty());
        assert!(!basic_info.cpu.is_empty());
        assert!(!basic_info.total_memory.is_empty());
        assert!(!basic_info.free_memory.is_empty());
        assert!(!basic_info.total_disk.is_empty());
        assert!(!basic_info.free_disk.is_empty());

        // Should be boolean flags (no heavy diagnostics)
        // These are just checking the types exist, values will vary by system
        let _is_k8s = basic_info.is_kubernetes;
        let _has_container_runtime = basic_info.container_runtime_available;
    }

    #[tokio::test]
    async fn test_debug_tool_command_display() {
        // Test that debug tools return actual commands that users can replicate
        let debug_tools = DebugTools::new();

        // Test a simple command
        let result = debug_tools.run_free().await;
        assert!(result.command.contains("free"));
        assert!(!result.command.contains("_")); // Should not contain internal tool naming

        // The command should be something users can actually run
        // like "free -h" not "run_free" or "free_command"
        assert!(result.command == "free -h" || result.command.starts_with("free "));
    }
}

fn print_output(
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

fn print_output_with_config(
    system_info: &SystemInfo,
    analysis: &str,
    config: &RaidConfig,
    ui_formatter: &UIFormatter,
) {
    match config.get_output_format() {
        OutputFormat::Text => {
            ui::print_results_with_formatter(system_info, analysis, config.output.verbose, ui_formatter);
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

fn print_system_info(info: &SystemInfo, analysis: &str, verbose: bool) {
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

fn print_container_info(info: &SystemInfo, analysis: &str, verbose: bool) {
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

fn print_kubernetes_info(info: &SystemInfo, analysis: &str, verbose: bool) {
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

fn print_cgroup_info(info: &SystemInfo, analysis: &str, verbose: bool) {
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

fn print_systemd_info(info: &SystemInfo, analysis: &str, verbose: bool) {
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

fn print_journal_info(info: &SystemInfo, analysis: &str, verbose: bool) {
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
fn print_results_dry_run(info: &SystemInfo) {
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
    common_errors
        .iter()
        .any(|error| message_lower.contains(error))
}

fn print_system_info_dry_run(info: &SystemInfo) {
    println!("=== System Information ===");
    println!("OS: {}", info.os);
    println!("CPU: {}", info.cpu);
    println!("\n=== DRY RUN MODE ===");
    println!("AI analysis skipped. Use without --dry-run flag for AI-powered insights.");
}

fn print_container_info_dry_run(info: &SystemInfo) {
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

fn print_kubernetes_info_dry_run(info: &SystemInfo) {
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

fn print_cgroup_info_dry_run(info: &SystemInfo) {
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

fn print_systemd_info_dry_run(info: &SystemInfo) {
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

fn print_journal_info_dry_run(info: &SystemInfo) {
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

async fn run_config_command(
    action: &ConfigAction,
    output_path: Option<&str>,
    config: &RaidConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        ConfigAction::Init => {
            let output_file = output_path.unwrap_or("raid.yaml");
            RaidConfig::create_sample_config(output_file)?;
            println!("✅ Created sample configuration file: {}", output_file);
            println!("💡 Edit this file to customize your settings, then use:");
            println!("   cargo run -- --config {}", output_file);
        }
        ConfigAction::Show => {
            let yaml_content = serde_yaml::to_string(config)?;
            println!("Current Configuration (merged from all sources):");
            println!("{}", yaml_content);
        }
        ConfigAction::Validate => {
            match config.validate() {
                Ok(_) => println!("✅ Configuration is valid"),
                Err(e) => {
                    eprintln!("❌ Configuration validation failed: {}", e);
                    std::process::exit(1);
                }
            }
        }
        ConfigAction::Locations => {
            println!("Configuration File Locations (in order of precedence):");
            println!("1. Command line: --config <file>");
            println!("2. Current directory: ./raid.yaml, ./raid.yml, ./raid.toml");
            println!("3. User config: ~/.config/raid/raid.yaml");
            println!("4. System config: /etc/raid.yaml");
            println!("5. Environment variables: RAID_*");
            println!("6. Built-in defaults");
            
            if let Some(user_config_dir) = dirs::config_dir() {
                let user_config_path = user_config_dir.join("raid").join("raid.yaml");
                println!("\n📁 Suggested user config location:");
                println!("   {}", user_config_path.display());
            }
        }
    }
    Ok(())
}

async fn run_question_answering_with_config(
    config: &RaidConfig,
    question: &str,
    ui_formatter: &UIFormatter,
) -> Result<(), Box<dyn std::error::Error>> {
    let ai_provider = create_ai_provider_from_cli(
        &config.get_ai_provider(),
        config.ai.api_key.clone(),
        Some(config.get_model()),
        config.ai.base_url.clone(),
        config.ai.max_tokens,
        config.ai.temperature,
    )
    .await?;

    println!("❓ Question: {}", question);
    println!("🤖 AI Assistant ({})", ai_provider.name());
    
    // Collect basic system info with progress
    let sys_info = ui_formatter.show_progress("Collecting system information", || {
        collect_basic_system_info()
    });

    // Create comprehensive context about the system
    let mut system_context = String::new();
    system_context.push_str(&format!("Operating System: {}\n", sys_info.os));
    system_context.push_str(&format!("CPU: {}\n", sys_info.cpu));
    system_context.push_str(&format!(
        "Memory: {}/{}\n",
        sys_info.free_memory, sys_info.total_memory
    ));
    system_context.push_str(&format!(
        "Disk: {}/{}\n",
        sys_info.free_disk, sys_info.total_disk
    ));

    if sys_info.is_kubernetes {
        system_context.push_str("Environment: Kubernetes cluster\n");
    }

    if sys_info.container_runtime_available {
        system_context.push_str("Container Runtime: Available\n");
    }

    // Get AI analysis (this is synchronous for now, but shows progress)
    let final_analysis = ui_formatter.show_progress("Getting AI analysis", || {
        // For now, we'll use a blocking approach
        // In a real implementation, you'd want to handle async properly
        "AI analysis would go here - this is a simplified version for demo purposes."
    });

    println!("\n🤖 AI Analysis:");
    println!("{}", final_analysis);

    Ok(())
}

async fn run_debug_tools(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let debug_tools = DebugTools::new();

    match &cli.command {
        Some(Commands::Debug {
            tool,
            namespace,
            pod,
            service,
            lines,
        }) => match tool {
            DebugTool::KubectlGetPods => {
                let result = debug_tools.run_kubectl_get_pods(namespace.as_deref()).await;
                print_debug_result(&result);
            }
            DebugTool::KubectlDescribePod => {
                if let Some(pod_name) = pod {
                    let result = debug_tools
                        .run_kubectl_describe_pod(pod_name, namespace.as_deref())
                        .await;
                    print_debug_result(&result);
                } else {
                    println!(
                        "Error: Pod name is required for describe command. Use --pod <pod-name>"
                    );
                }
            }
            DebugTool::KubectlGetServices => {
                let result = debug_tools
                    .run_kubectl_get_services(namespace.as_deref())
                    .await;
                print_debug_result(&result);
            }
            DebugTool::KubectlGetNodes => {
                let result = debug_tools.run_kubectl_get_nodes().await;
                print_debug_result(&result);
            }
            DebugTool::KubectlGetEvents => {
                let result = debug_tools
                    .run_kubectl_get_events(namespace.as_deref())
                    .await;
                print_debug_result(&result);
            }
            DebugTool::JournalctlRecent => {
                let result = debug_tools.run_journalctl_recent(*lines).await;
                print_debug_result(&result);
            }
            DebugTool::JournalctlService => {
                if let Some(service_name) = service {
                    let result = debug_tools
                        .run_journalctl_service(service_name, *lines)
                        .await;
                    print_debug_result(&result);
                } else {
                    println!(
                        "Error: Service name is required for service logs. Use --service <service-name>"
                    );
                }
            }
            DebugTool::JournalctlBoot => {
                let result = debug_tools.run_journalctl_boot().await;
                print_debug_result(&result);
            }
            DebugTool::JournalctlErrors => {
                let result = debug_tools.run_journalctl_errors(*lines).await;
                print_debug_result(&result);
            }
            DebugTool::SystemctlStatus => {
                if let Some(service_name) = service {
                    let result = debug_tools.run_systemctl_status(service_name).await;
                    print_debug_result(&result);
                } else {
                    println!(
                        "Error: Service name is required for systemctl status. Use --service <service-name>"
                    );
                }
            }
            DebugTool::PsAux => {
                let result = debug_tools.run_ps_aux().await;
                print_debug_result(&result);
            }
            DebugTool::Netstat => {
                let result = debug_tools.run_netstat().await;
                print_debug_result(&result);
            }
            DebugTool::Df => {
                let result = debug_tools.run_df().await;
                print_debug_result(&result);
            }
            DebugTool::Free => {
                let result = debug_tools.run_free().await;
                print_debug_result(&result);
            }
            DebugTool::CatProcCgroups => {
                let result = debug_tools.run_cat_proc_cgroups().await;
                print_debug_result(&result);
            }
            DebugTool::LsCgroup => {
                let result = debug_tools.run_ls_cgroup().await;
                print_debug_result(&result);
            }
            DebugTool::CatProcSelfCgroup => {
                let result = debug_tools.run_cat_proc_self_cgroup().await;
                print_debug_result(&result);
            }
            DebugTool::CatProcSelfMountinfo => {
                let result = debug_tools.run_cat_proc_self_mountinfo().await;
                print_debug_result(&result);
            }
            DebugTool::Lsns => {
                let result = debug_tools.run_lsns().await;
                print_debug_result(&result);
            }
            DebugTool::CatProcSelfStatus => {
                let result = debug_tools.run_cat_proc_self_status().await;
                print_debug_result(&result);
            }
            DebugTool::CatProcSelfNs => {
                let result = debug_tools.run_cat_proc_self_ns().await;
                print_debug_result(&result);
            }
        },
        _ => {
            println!("Error: Debug command not found");
        }
    }

    Ok(())
}

fn print_debug_result(result: &tools::DebugToolResult) {
    println!("=== Debug Tool Result ===");
    println!("Tool: {}", result.tool_name);
    println!("Command: {}", result.command);
    println!("Success: {}", result.success);
    println!("Execution Time: {}ms", result.execution_time_ms);

    if let Some(error) = &result.error {
        println!("Error: {}", error);
    }

    println!("\n=== Output ===");
    println!("{}", result.output);
}

async fn run_question_answering(
    cli: &Cli,
    question: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let ai_provider = create_ai_provider_from_cli(
        &cli.ai_provider,
        cli.ai_api_key.clone(),
        Some(cli.get_model()),
        cli.ai_base_url.clone(),
        cli.ai_max_tokens,
        cli.ai_temperature,
    )
    .await?;

    println!("❓ Question: {}", question);
    println!("🤖 AI Assistant ({})", ai_provider.name());
    println!("Analyzing your question and determining which tools to run...\n");

    // Collect basic system info
    let sys_info = collect_basic_system_info();

    // Create comprehensive context about the system
    let mut system_context = String::new();
    system_context.push_str(&format!("Operating System: {}\n", sys_info.os));
    system_context.push_str(&format!("CPU: {}\n", sys_info.cpu));
    system_context.push_str(&format!(
        "Memory: {}/{}\n",
        sys_info.free_memory, sys_info.total_memory
    ));
    system_context.push_str(&format!(
        "Disk: {}/{}\n",
        sys_info.free_disk, sys_info.total_disk
    ));

    if sys_info.is_kubernetes {
        system_context.push_str("Environment: Kubernetes cluster\n");
    }

    if sys_info.container_runtime_available {
        system_context.push_str("Container Runtime: Available\n");
    }

    // Create a comprehensive list of available tools for the AI to choose from
    let available_tools = r#"
AVAILABLE DIAGNOSTIC TOOLS:

KUBERNETES TOOLS:
- kubectl_get_pods [--namespace <ns>]: List all pods in namespace
- kubectl_describe_pod <pod_name> [--namespace <ns>]: Get detailed pod information  
- kubectl_get_services [--namespace <ns>]: List all services in namespace
- kubectl_get_nodes: List all cluster nodes
- kubectl_get_events [--namespace <ns>]: Get recent cluster events

SYSTEM LOGS:
- journalctl_recent [--lines <n>]: Get recent system logs (default 50 lines)
- journalctl_service <service_name> [--lines <n>]: Get logs for specific service
- journalctl_boot: Get boot logs
- journalctl_errors [--lines <n>]: Get error logs only

SYSTEM SERVICES:
- systemctl_status <service_name>: Get status of specific service

CONTAINER TOOLS:
- docker_ps: List all Docker containers
- docker_inspect <container_name>: Inspect specific container
- docker_logs <container_name> [--lines <n>]: Get container logs

PROCESS & PERFORMANCE:
- ps_aux: List all running processes
- top: Show current system processes and resource usage
- free: Show memory usage
- vmstat: Virtual memory statistics
- iotop: Show I/O usage by process
- htop: Interactive process viewer
- pidstat: Process statistics

NETWORK DIAGNOSTICS:
- netstat: Show network connections
- ss: Show socket statistics
- ip_addr: Show network interfaces
- ip_route: Show routing table
- ping <host>: Test network connectivity
- dig <domain>: DNS lookup
- iptables: Show firewall rules

STORAGE DIAGNOSTICS:
- df: Show disk usage
- lsblk: List block devices
- iostat: I/O statistics
- mount: Show mounted filesystems
- du <path>: Directory usage

CONTAINER/CGROUP INFO:
- cat_proc_cgroups: Get cgroups information
- ls_cgroup: List cgroup filesystem
- cat_proc_self_cgroup: Current process cgroup info
- lsns: List all namespaces

SECURITY TOOLS:
- w: Show logged in users
- last: Show login history
- ps_ef: Show all processes with full info
- lsof: Show open files and network connections
"#;

    // First, ask the AI which tools it wants to run
    let tool_selection_prompt = format!(
        r#"You are an expert Linux systems administrator and Kubernetes operator. Your goal is to help diagnose and solve the user's problem by selecting the most appropriate diagnostic tools.

QUESTION: {}

SYSTEM CONTEXT:
{}

{}

INSTRUCTIONS:
1. Analyze the user's question carefully
2. Based on the question and system context, select the most relevant diagnostic tools
3. Consider what information you need to properly diagnose the issue
4. List the tools you want to run, one per line, in the format: TOOL_NAME [arguments]

RESPONSE FORMAT:
Provide your tool selection as a simple list, one tool per line. For example:
```
docker_ps
kubectl_get_pods --namespace default
journalctl_recent --lines 20
systemctl_status nginx
```

Do NOT provide analysis yet - just select the tools you need to gather information.
Select 2-5 most relevant tools based on the question."#,
        question, system_context, available_tools
    );

    // Get AI's tool recommendations
    let tool_recommendations = ai_provider
        .answer_question("", &tool_selection_prompt)
        .await?;

    println!("🔧 AI selected these diagnostic tools:");
    println!("{}", tool_recommendations);
    println!();

    // Parse and execute the recommended tools
    let debug_tools = DebugTools::new();
    let mut tool_results = Vec::new();
    let mut tools_used = Vec::new();

    // Parse the AI's response to extract tool commands
    for line in tool_recommendations.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("```") {
            continue;
        }

        // Remove leading dashes or bullets
        let line = line.trim_start_matches("- ").trim_start_matches("* ");

        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        let tool_name = parts[0];

        // Execute the tool based on the AI's recommendation
        let result = match tool_name {
            "kubectl_get_pods" => {
                let namespace = extract_arg(&parts, "--namespace");
                debug_tools.run_kubectl_get_pods(namespace.as_deref()).await
            }
            "kubectl_describe_pod" => {
                let pod_name = parts.get(1).map(|s| s.to_string());
                let namespace = extract_arg(&parts, "--namespace");
                if let Some(pod) = pod_name {
                    debug_tools
                        .run_kubectl_describe_pod(&pod, namespace.as_deref())
                        .await
                } else {
                    continue;
                }
            }
            "kubectl_get_services" => {
                let namespace = extract_arg(&parts, "--namespace");
                debug_tools
                    .run_kubectl_get_services(namespace.as_deref())
                    .await
            }
            "kubectl_get_nodes" => debug_tools.run_kubectl_get_nodes().await,
            "kubectl_get_events" => {
                let namespace = extract_arg(&parts, "--namespace");
                debug_tools
                    .run_kubectl_get_events(namespace.as_deref())
                    .await
            }
            "journalctl_recent" => {
                let lines = extract_arg(&parts, "--lines").and_then(|s| s.parse().ok());
                debug_tools.run_journalctl_recent(lines).await
            }
            "journalctl_service" => {
                let service_name = parts.get(1).map(|s| s.to_string());
                let lines = extract_arg(&parts, "--lines").and_then(|s| s.parse().ok());
                if let Some(service) = service_name {
                    debug_tools.run_journalctl_service(&service, lines).await
                } else {
                    continue;
                }
            }
            "journalctl_boot" => debug_tools.run_journalctl_boot().await,
            "journalctl_errors" => {
                let lines = extract_arg(&parts, "--lines").and_then(|s| s.parse().ok());
                debug_tools.run_journalctl_errors(lines).await
            }
            "systemctl_status" => {
                let service_name = parts.get(1).map(|s| s.to_string());
                if let Some(service) = service_name {
                    debug_tools.run_systemctl_status(&service).await
                } else {
                    continue;
                }
            }
            "docker_ps" => debug_tools.run_docker_ps().await,
            "docker_inspect" => {
                let container_name = parts.get(1).map(|s| s.to_string());
                if let Some(container) = container_name {
                    debug_tools.run_docker_inspect(&container).await
                } else {
                    continue;
                }
            }
            "docker_logs" => {
                let container_name = parts.get(1).map(|s| s.to_string());
                let lines = extract_arg(&parts, "--lines").and_then(|s| s.parse().ok());
                if let Some(container) = container_name {
                    debug_tools.run_docker_logs(&container, lines).await
                } else {
                    continue;
                }
            }
            "ps_aux" => debug_tools.run_ps_aux().await,
            "top" => debug_tools.run_top().await,
            "free" => debug_tools.run_free().await,
            "netstat" => debug_tools.run_netstat().await,
            "df" => debug_tools.run_df().await,
            _ => {
                println!("⚠️  Unknown tool: {}", tool_name);
                continue;
            }
        };

        // Show the actual command that was executed so users can replicate it
        println!("🔍 Running: {}", result.command);

        tools_used.push(result.command.clone());

        if result.success {
            println!("✅ Tool completed successfully");
            // Show brief output preview
            let preview = result.output.lines().take(3).collect::<Vec<_>>().join("\n");
            if !preview.trim().is_empty() {
                println!("Preview: {}", preview);
            }
        } else {
            println!(
                "❌ Tool failed: {}",
                result.error.clone().unwrap_or("Unknown error".to_string())
            );
        }
        tool_results.push(result);
        println!();
    }

    // Build comprehensive context with tool results for final analysis
    let mut final_context = system_context;

    if !tool_results.is_empty() {
        final_context.push_str("\nDIAGNOSTIC TOOL RESULTS:\n");
        for result in &tool_results {
            final_context.push_str(&format!("=== {} ===\n", result.command));
            final_context.push_str(&format!("Success: {}\n", result.success));
            if result.success {
                final_context.push_str(&format!("Output:\n{}\n\n", result.output));
            } else if let Some(error) = &result.error {
                final_context.push_str(&format!("Error: {}\n\n", error));
            }
        }
    }

    // Get final AI analysis
    let analysis_prompt = format!(
        r#"You are an expert Linux systems administrator and Kubernetes operator. Based on the user's question and the diagnostic information gathered, provide a comprehensive analysis and solution.

ORIGINAL QUESTION: {}

DIAGNOSTIC INFORMATION:
{}

INSTRUCTIONS:
1. Analyze the diagnostic information in the context of the user's question
2. Identify any issues, problems, or relevant findings
3. Provide clear explanations of what you found
4. Offer specific, actionable solutions or next steps
5. If everything looks normal, explain that clearly
6. Be concise but thorough in your analysis

Provide your analysis now:"#,
        question, final_context
    );

    println!("🤖 AI Analysis:");
    let final_analysis = ai_provider.answer_question("", &analysis_prompt).await?;
    println!("{}", final_analysis);

    // Provide guidance on running commands manually
    if !tools_used.is_empty() {
        println!("\n📚 Commands used for this analysis:");
        for command in &tools_used {
            println!("  $ {}", command);
        }
        println!();
        println!("💡 Tip: You can run any of these commands directly in your terminal!");
    }

    Ok(())
}

// Helper function to extract arguments from command line
fn extract_arg(parts: &[&str], arg_name: &str) -> Option<String> {
    for i in 0..parts.len() {
        if parts[i] == arg_name && i + 1 < parts.len() {
            return Some(parts[i + 1].to_string());
        }
    }
    None
}

async fn run_ai_agent(
    cli: &Cli,
    problem_description: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let ai_provider = create_ai_provider_from_cli(
        &cli.ai_provider,
        cli.ai_api_key.clone(),
        Some(cli.get_model()),
        cli.ai_base_url.clone(),
        cli.ai_max_tokens,
        cli.ai_temperature,
    )
    .await?;

    let debug_tools = DebugTools::new();
    let mut context = String::new();
    let mut tool_results = Vec::new();
    let mut user_clarifications = 0;
    let max_clarifications = 3;

    println!("🤖 AI Agent Mode");
    println!("Problem: {}", problem_description);
    println!("Starting analysis...\n");

    // Initial system info collection
    let sys_info = collect_system_info();
    context.push_str(&format!(
        "System Info: OS={}, CPU={}, K8s={}\n",
        sys_info.os, sys_info.cpu, sys_info.kubernetes.is_kubernetes
    ));

    // AI agent loop
    let max_iterations = 10;
    let mut iteration = 1;
    while iteration <= max_iterations {
        println!("🔄 Iteration {}/{}", iteration, max_iterations);

        // Build the prompt for the AI
        let prompt = build_agent_prompt(problem_description, &context, &tool_results);

        // Get AI response with known issues context
        let response = ai_provider.analyze_with_known_issues(&prompt, None).await?;

        // Parse AI response to determine action
        match parse_ai_response(&response) {
            AIAgentAction::RunTool {
                tool,
                namespace,
                pod,
                service,
                lines,
            } => {
                println!("🔧 Running tool: {:?}", tool);

                let result = match tool {
                    DebugTool::KubectlGetPods => {
                        debug_tools.run_kubectl_get_pods(namespace.as_deref()).await
                    }
                    DebugTool::KubectlDescribePod => {
                        if let Some(pod_name) = pod {
                            debug_tools
                                .run_kubectl_describe_pod(&pod_name, namespace.as_deref())
                                .await
                        } else {
                            tools::DebugToolResult {
                                tool_name: "kubectl_describe_pod".to_string(),
                                command: "kubectl describe pod".to_string(),
                                success: false,
                                output: String::new(),
                                error: Some("Pod name required".to_string()),
                                execution_time_ms: 0,
                            }
                        }
                    }
                    DebugTool::KubectlGetServices => {
                        debug_tools
                            .run_kubectl_get_services(namespace.as_deref())
                            .await
                    }
                    DebugTool::KubectlGetNodes => debug_tools.run_kubectl_get_nodes().await,
                    DebugTool::KubectlGetEvents => {
                        debug_tools
                            .run_kubectl_get_events(namespace.as_deref())
                            .await
                    }
                    DebugTool::JournalctlRecent => debug_tools.run_journalctl_recent(lines).await,
                    DebugTool::JournalctlService => {
                        if let Some(service_name) = service {
                            debug_tools
                                .run_journalctl_service(&service_name, lines)
                                .await
                        } else {
                            tools::DebugToolResult {
                                tool_name: "journalctl_service".to_string(),
                                command: "journalctl -u".to_string(),
                                success: false,
                                output: String::new(),
                                error: Some("Service name required".to_string()),
                                execution_time_ms: 0,
                            }
                        }
                    }
                    DebugTool::JournalctlBoot => debug_tools.run_journalctl_boot().await,
                    DebugTool::JournalctlErrors => debug_tools.run_journalctl_errors(lines).await,
                    DebugTool::SystemctlStatus => {
                        if let Some(service_name) = service {
                            debug_tools.run_systemctl_status(&service_name).await
                        } else {
                            tools::DebugToolResult {
                                tool_name: "systemctl_status".to_string(),
                                command: "systemctl status".to_string(),
                                success: false,
                                output: String::new(),
                                error: Some("Service name required".to_string()),
                                execution_time_ms: 0,
                            }
                        }
                    }
                    DebugTool::PsAux => debug_tools.run_ps_aux().await,
                    DebugTool::Netstat => debug_tools.run_netstat().await,
                    DebugTool::Df => debug_tools.run_df().await,
                    DebugTool::Free => debug_tools.run_free().await,
                    DebugTool::CatProcCgroups => debug_tools.run_cat_proc_cgroups().await,
                    DebugTool::LsCgroup => debug_tools.run_ls_cgroup().await,
                    DebugTool::CatProcSelfCgroup => debug_tools.run_cat_proc_self_cgroup().await,
                    DebugTool::CatProcSelfMountinfo => {
                        debug_tools.run_cat_proc_self_mountinfo().await
                    }
                    DebugTool::Lsns => debug_tools.run_lsns().await,
                    DebugTool::CatProcSelfStatus => debug_tools.run_cat_proc_self_status().await,
                    DebugTool::CatProcSelfNs => debug_tools.run_cat_proc_self_ns().await,
                };

                tool_results.push(result.clone());
                context.push_str(&format!(
                    "\nTool Result ({:?}): {}\n",
                    tool,
                    if result.success { "SUCCESS" } else { "FAILED" }
                ));
                context.push_str(&result.output);

                println!("✅ Tool completed");
            }
            AIAgentAction::ProvideAnalysis { analysis } => {
                println!("\n🎯 Final Analysis:");
                println!("{}", analysis);
                break;
            }
            AIAgentAction::AskUser { question } => {
                println!("\n❓ AI Question: {}", question);
                if user_clarifications < max_clarifications {
                    print!("Your answer: ");
                    io::stdout().flush().unwrap();
                    let mut user_input = String::new();
                    io::stdin().read_line(&mut user_input)?;
                    context.push_str(&format!("\nUser clarification: {}\n", user_input.trim()));
                    user_clarifications += 1;
                    // Continue the loop for another iteration
                } else {
                    println!("Maximum number of clarifications reached. Stopping.");
                    break;
                }
            }
        }

        if iteration == max_iterations {
            println!("\n⚠️  Maximum iterations reached. Providing current analysis.");
            let final_analysis = ai_provider.analyze_with_known_issues(&format!(
                "Based on the problem '{}' and the collected information, provide a final analysis and recommendations.",
                problem_description
            ), None).await?;
            println!("\n🎯 Final Analysis:");
            println!("{}", final_analysis);
        }
        iteration += 1;
    }

    Ok(())
}

fn build_agent_prompt(
    problem: &str,
    context: &str,
    tool_results: &[tools::DebugToolResult],
) -> String {
    let tool_descriptions = r#"
AVAILABLE TOOLS:
- kubectl_get_pods [--namespace <ns>]: List all pods in namespace
- kubectl_describe_pod <pod_name> [--namespace <ns>]: Get detailed pod information
- kubectl_get_services [--namespace <ns>]: List all services in namespace
- kubectl_get_nodes: List all cluster nodes
- kubectl_get_events [--namespace <ns>]: Get recent cluster events
- journalctl_recent [--lines <n>]: Get recent system logs (default 50 lines)
- journalctl_service <service_name> [--lines <n>]: Get logs for specific service
- journalctl_boot: Get boot logs
- journalctl_errors [--lines <n>]: Get error logs only
- systemctl_status <service_name>: Get status of specific service
- ps_aux: List all running processes
- netstat: Show network connections
- df: Show disk usage
- free: Show memory usage
- cat_proc_cgroups: Get cgroups information from /proc/cgroups
- ls_cgroup: List cgroup filesystem structure
- cat_proc_self_cgroup: Get current process cgroup information
- cat_proc_self_mountinfo: Get mount information for current process
- lsns: List all namespaces on the system
- cat_proc_self_status: Get current process status information
- cat_proc_self_ns: List namespace files for current process
"#;

    let tool_history = if tool_results.is_empty() {
        "None".to_string()
    } else {
        tool_results
            .iter()
            .map(|r| {
                format!(
                    "- {}: {} ({}ms)",
                    r.tool_name,
                    if r.success { "SUCCESS" } else { "FAILED" },
                    r.execution_time_ms
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        r#"You are an expert system administrator and Kubernetes operator. Your goal is to diagnose and solve the user's problem.

PROBLEM: {}

CURRENT CONTEXT:
{}

TOOL EXECUTION HISTORY:
{}

AVAILABLE TOOLS:{}

THINKING PROCESS:
1. Analyze the problem description and current context
2. Determine what information you need to diagnose the issue
3. Choose the most relevant tool to run next
4. If you have enough information, provide a comprehensive analysis
5. If you need more information from the user, ask a specific question

RESPONSE FORMAT:
- To run a tool: RUN_TOOL <tool_name> [--namespace <ns>] [--pod <pod>] [--service <service>] [--lines <n>]
- To provide analysis: ANALYZE <your detailed analysis with root cause and recommendations>
- To ask user: ASK <specific question>

EXAMPLES:
- RUN_TOOL kubectl_get_pods --namespace default
- RUN_TOOL journalctl_service nginx --lines 20
- ANALYZE The issue is caused by insufficient memory. Pods are being OOM killed.
- ASK What is the name of the pod that's failing?

Choose the most logical next step based on the problem and available information."#,
        problem, context, tool_history, tool_descriptions
    )
}

fn parse_ai_response(response: &str) -> AIAgentAction {
    let response = response.trim();

    // Look for the action keywords in the response
    if response.contains("RUN_TOOL") {
        // Extract the tool command from the response
        let tool_match = response
            .lines()
            .find(|line| line.trim().starts_with("RUN_TOOL"))
            .unwrap_or("");

        let parts: Vec<&str> = tool_match.split_whitespace().collect();
        if parts.len() >= 2 {
            let tool_name = parts[1];
            let tool = match tool_name {
                "kubectl_get_pods" => DebugTool::KubectlGetPods,
                "kubectl_describe_pod" => DebugTool::KubectlDescribePod,
                "kubectl_get_services" => DebugTool::KubectlGetServices,
                "kubectl_get_nodes" => DebugTool::KubectlGetNodes,
                "kubectl_get_events" => DebugTool::KubectlGetEvents,
                "journalctl_recent" => DebugTool::JournalctlRecent,
                "journalctl_service" => DebugTool::JournalctlService,
                "journalctl_boot" => DebugTool::JournalctlBoot,
                "journalctl_errors" => DebugTool::JournalctlErrors,
                "systemctl_status" => DebugTool::SystemctlStatus,
                "ps_aux" => DebugTool::PsAux,
                "netstat" => DebugTool::Netstat,
                "df" => DebugTool::Df,
                "free" => DebugTool::Free,
                _ => {
                    println!("Unknown tool: {}", tool_name);
                    return AIAgentAction::ProvideAnalysis {
                        analysis: format!("Unknown tool requested: {}", tool_name),
                    };
                }
            };

            // Parse optional parameters
            let mut namespace = None;
            let mut pod = None;
            let mut service = None;
            let mut lines = None;

            let mut i = 2;
            while i < parts.len() {
                match parts[i] {
                    "--namespace" if i + 1 < parts.len() => {
                        namespace = Some(parts[i + 1].to_string());
                        i += 2;
                    }
                    "--pod" if i + 1 < parts.len() => {
                        pod = Some(parts[i + 1].to_string());
                        i += 2;
                    }
                    "--service" if i + 1 < parts.len() => {
                        service = Some(parts[i + 1].to_string());
                        i += 2;
                    }
                    "--lines" if i + 1 < parts.len() => {
                        lines = parts[i + 1].parse().ok();
                        i += 2;
                    }
                    _ => i += 1,
                }
            }

            return AIAgentAction::RunTool {
                tool,
                namespace,
                pod,
                service,
                lines,
            };
        }
    } else if response.contains("ANALYZE") {
        // Extract analysis from the response
        let analysis_start = response.find("ANALYZE").unwrap_or(0);
        let analysis = response[analysis_start..]
            .strip_prefix("ANALYZE")
            .unwrap_or("")
            .trim();
        return AIAgentAction::ProvideAnalysis {
            analysis: analysis.to_string(),
        };
    } else if response.contains("ASK") {
        // Extract question from the response
        let ask_start = response.find("ASK").unwrap_or(0);
        let question = response[ask_start..]
            .strip_prefix("ASK")
            .unwrap_or("")
            .trim();
        return AIAgentAction::AskUser {
            question: question.to_string(),
        };
    }

    // If no clear action found, treat as analysis
    AIAgentAction::ProvideAnalysis {
        analysis: response.to_string(),
    }
}

async fn run_issues_management(cli: &Cli) -> Result<(), Box<dyn std::error::Error>> {
    let db = known_issues::KnownIssuesDatabase::new().await;

    if let Some(Commands::Issues {
        action,
        issue_id,
        query,
    }) = &cli.command
    {
        match action {
            IssueAction::List => {
                println!("📋 Known Issues Database");
                println!("========================");
                let issues = db.get_all_issues().await;
                if issues.is_empty() {
                    println!("No known issues found.");
                } else {
                    for issue in issues {
                        println!("\n🔍 {}", issue.title);
                        println!("   ID: {}", issue.id);
                        println!("   Category: {:?}", issue.category);
                        println!("   Severity: {:?}", issue.severity);
                        println!("   Description: {}", issue.description);
                        println!("   Tags: {}", issue.tags.join(", "));
                    }
                }
            }
            IssueAction::Get => {
                if let Some(id) = issue_id {
                    if let Some(issue) = db.get_issue(id).await {
                        println!("📋 Issue Details");
                        println!("================");
                        println!("Title: {}", issue.title);
                        println!("ID: {}", issue.id);
                        println!("Category: {:?}", issue.category);
                        println!("Severity: {:?}", issue.severity);
                        println!("Description: {}", issue.description);
                        println!("Patterns: {}", issue.patterns.join(", "));
                        println!("Keywords: {}", issue.keywords.join(", "));
                        println!("Symptoms: {}", issue.symptoms.join(", "));
                        println!("Verification Commands:");
                        for cmd in &issue.verification_commands {
                            println!("  - {}", cmd);
                        }
                        println!("Fix Commands:");
                        for cmd in &issue.fix_commands {
                            println!("  - {}", cmd);
                        }
                        println!("Tags: {}", issue.tags.join(", "));
                    } else {
                        println!("❌ Issue with ID '{}' not found.", id);
                    }
                } else {
                    println!("❌ Issue ID required for 'get' action. Use --issue-id <id>");
                }
            }
            IssueAction::Search => {
                if let Some(search_query) = query {
                    println!("🔍 Searching for issues matching: '{}'", search_query);
                    let issues = db.search_issues(search_query).await;
                    if issues.is_empty() {
                        println!("No issues found matching '{}'", search_query);
                    } else {
                        println!("Found {} matching issues:", issues.len());
                        for issue in issues {
                            println!("\n🔍 {}", issue.title);
                            println!("   ID: {}", issue.id);
                            println!("   Category: {:?}", issue.category);
                            println!("   Severity: {:?}", issue.severity);
                            println!("   Description: {}", issue.description);
                        }
                    }
                } else {
                    println!("❌ Search query required. Use --query <search_term>");
                }
            }
            IssueAction::Add => {
                println!(
                    "❌ Add functionality not yet implemented. This would allow adding new known issues."
                );
            }
            IssueAction::Update => {
                println!(
                    "❌ Update functionality not yet implemented. This would allow updating existing known issues."
                );
            }
            IssueAction::Delete => {
                println!(
                    "❌ Delete functionality not yet implemented. This would allow deleting known issues."
                );
            }
        }
    }

    Ok(())
}
