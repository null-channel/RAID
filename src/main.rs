mod ai;
mod cli;
mod commands;
mod config;
mod database;
mod known_issues;
mod output;
mod sysinfo;
mod tools;
mod ui;

use clap::Parser;
use cli::{Cli, Commands};
use commands::{ai::AnalysisType, config::run_config_command};
use config::RaidConfig;
use sysinfo::{collect_basic_system_info, collect_system_info};
use ui::UIFormatter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Parse CLI args
    let cli = Cli::parse();

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

    // Create UI formatter
    let ui_formatter = UIFormatter::new(config.output.color);

    // Handle different commands
    match &cli.command {
        Some(Commands::Config { action, output }) => {
            run_config_command(action, output.as_deref(), &config).await?;
        }
        Some(Commands::Issues { .. }) => {
            println!("Issues management functionality is being refactored.");
            println!("The original complex functionality is preserved in main_old.rs");
        }
        Some(Commands::Debug { .. }) => {
            println!("Debug functionality is being refactored.");
            println!("The original complex functionality is preserved in main_old.rs");
        }
        Some(Commands::Check { .. }) => {
            println!("Check functionality is being refactored.");
            println!("The original complex functionality is preserved in main_old.rs");
        }
        None => {
            // Default: system health check
            if cli.dry_run {
                let info = collect_basic_system_info();
                println!("🔍 System Health Check (Dry Run)");
                println!("OS: {}", info.os);
                println!("CPU: {}", info.cpu);
                println!("Memory: {}/{}", info.free_memory, info.total_memory);
                println!("Disk: {}/{}", info.free_disk, info.total_disk);
                println!("\n=== DRY RUN MODE ===");
                println!("AI analysis skipped. Use without --dry-run flag for AI-powered insights.");
            } else {
                // Collect full system info
                let _info = ui_formatter.show_progress("Collecting system information", || {
                    collect_system_info()
                });
                
                // Run AI analysis for comprehensive system check
                commands::ai::run_unified_ai_analysis(
                    &config,
                    "Analyze this system's health and provide insights on any issues or optimizations.",
                    &ui_formatter,
                    AnalysisType::SystemCheck,
                ).await?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_loading() {
        // Test basic config loading
        let config = RaidConfig::default();
        assert!(config.validate().is_ok());
    }
} 