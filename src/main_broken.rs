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
use cli::{Cli, Commands, ConfigAction};
use commands::{ai::AnalysisType, config::run_config_command, debug::run_debug_tools};
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
            println!("Please check the original implementation in main_old.rs");
        }
        Some(Commands::Ask { question }) => {
            commands::ai::run_question_answering_with_config(&config, question, &ui_formatter).await?;
        }
        Some(Commands::Agent { problem, max_tool_calls }) => {
            commands::ai::run_ai_agent_mode(&config, problem, &ui_formatter, *max_tool_calls).await?;
        }
        Some(Commands::Debug { .. }) => {
            run_debug_tools(&cli).await?;
        }
        None => {
            // Default: system health check
            if cli.dry_run {
                let info = collect_basic_system_info();
                output::printers::print_results_dry_run(&info);
            } else {
                // Collect full system info
                let info = ui_formatter.show_progress("Collecting system information", || {
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

async fn run_issues_management(
    cli: &Cli,
    action: &IssueAction,
) -> Result<(), Box<dyn std::error::Error>> {
    let db = Database::new("system_checks.db")?;

    match action {
        IssueAction::List { category } => {
            println!("📋 Known Issues Database");
            println!("=".repeat(40));
            
            let issues = if let Some(cat) = category {
                db.get_issues_by_category(cat)?
            } else {
                db.get_all_issues()?
            };

            if issues.is_empty() {
                println!("No issues found in the database.");
            } else {
                for issue in issues {
                    println!("\n🔍 Issue: {}", issue.description);
                    println!("   Category: {:?}", issue.category);
                    println!("   Severity: {:?}", issue.severity);
                    if !issue.patterns.is_empty() {
                        println!("   Patterns: {}", issue.patterns.join(", "));
                    }
                    if let Some(solution) = &issue.solution {
                        println!("   Solution: {}", solution);
                    }
                }
            }
        }
        IssueAction::Add { description, category, severity, patterns, solution } => {
            let category = category.clone().unwrap_or(IssueCategory::System);
            let severity = severity.clone().unwrap_or(known_issues::Severity::Medium);
            let patterns = patterns.clone().unwrap_or_default();
            
            db.add_issue(description, category, severity, patterns, solution.clone())?;
            println!("✅ Issue added to database");
        }
        IssueAction::Search { query } => {
            println!("🔍 Searching for: {}", query);
            println!("=".repeat(40));
            
            let issues = db.search_issues(query)?;
            
            if issues.is_empty() {
                println!("No matching issues found.");
            } else {
                for issue in issues {
                    println!("\n📌 {}", issue.description);
                    println!("   Category: {:?}", issue.category);
                    if let Some(solution) = &issue.solution {
                        println!("   💡 Solution: {}", solution);
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_question_answering_functionality() {
        // Test that the question answering flow works
        let config = RaidConfig::default();
        let ui_formatter = UIFormatter::new(false);
        
        // This test should not fail due to missing API keys in test environment
        let result = commands::ai::run_question_answering_with_config(
            &config,
            "test question",
            &ui_formatter,
        ).await;
        
        // In test environment, we expect this to fail due to missing API key
        // but the function should exist and be callable
        assert!(result.is_err());
    }

    #[test]
    fn test_config_loading() {
        // Test basic config loading
        let config = RaidConfig::default();
        assert!(config.validate().is_ok());
    }
} 