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

use ai::{create_ai_provider_from_cli, AIAgent, AIAgentConfig, AIAgentResult};
use clap::Parser;
use cli::{CheckComponent, Cli, Commands, IssueAction};
use commands::{config::run_config_command, debug::run_debug_tools};
use config::RaidConfig;

use sysinfo::collect_basic_system_info;
use tools::DebugTools;
use ui::UIFormatter;

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

    // Create UI formatter
    let ui_formatter = UIFormatter::new(config.output.color && !cli.no_color);

    // Initialize debug tools with availability checking at startup
    println!("🔧 Checking available system tools...");
    let debug_tools = DebugTools::initialize_with_availability_check();
    let available_categories = debug_tools.get_available_categories();
    if config.output.verbose || cli.verbose {
        println!("📋 Available tool categories: {:?}", available_categories);
        for category in &available_categories {
            let tools = debug_tools.get_category_tools(category);
            println!("   {:?}: {} tools available", category, tools.len());
        }
    }

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

    // If AI_API_KEY is not set and no key provided via CLI, force dry-run and print a message
    if config.ai.api_key.is_none() && !cli.dry_run {
        println!("No AI API key found. Running in dry-run mode. No AI model will be used.");
        cli.dry_run = true;
    }

    // Handle dry-run mode (no AI analysis)
    if cli.dry_run {
        let info = collect_basic_system_info();
        println!("🔍 System Health Check (Dry Run)");
        println!("OS: {}", info.os);
        println!("CPU: {}", info.cpu);
        println!("Memory: {}/{}", info.free_memory, info.total_memory);
        println!("Disk: {}/{}", info.free_disk, info.total_disk);
        println!("\n=== DRY RUN MODE ===");
        println!("AI analysis skipped. Use without --dry-run flag for AI-powered insights.");
        return Ok(());
    }

    // For all AI-powered operations, use the unified AIAgent system
    run_unified_ai_system(&config, &ui_formatter, &cli).await
}

/// Unified AI system that always uses AIAgent with full tool access
async fn run_unified_ai_system(
    config: &RaidConfig,
    ui_formatter: &UIFormatter,
    cli: &Cli,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if AI API key is available
    if config.ai.api_key.is_none() {
        println!("❌ No AI API key found. AI analysis requires an AI provider.");
        println!("Please set your AI_API_KEY environment variable or use --ai-api-key flag.");
        println!("Supported providers: OpenAI, Anthropic, Local (Ollama)");
        println!("\nFor a basic system check without AI, use: cargo run -- --dry-run");
        return Ok(());
    }

    // Create AI provider
    let ai_provider = match create_ai_provider_from_cli(
        &config.get_ai_provider(),
        config.ai.api_key.clone(),
        Some(config.get_model()),
        config.ai.base_url.clone(),
        config.ai.max_tokens,
        config.ai.temperature,
    ).await {
        Ok(provider) => provider,
        Err(e) => {
            println!("❌ Failed to initialize AI provider: {}", e);
            println!("This usually means:");
            println!("  • Invalid API key");
            println!("  • Network connectivity issues");
            println!("  • Service temporarily unavailable");
            println!("\nPlease check your API key and try again.");
            println!("\nFor a basic system check without AI, use: cargo run -- --dry-run");
            return Ok(());
        }
    };

    // Test AI provider connection before proceeding
    match ai_provider.analyze("test").await {
        Ok(_) => {
            // Provider is working, proceed with analysis
        },
        Err(e) => {
            println!("❌ AI provider test failed: {}", e);
            println!("This usually indicates:");
            println!("  • Invalid or expired API key");
            println!("  • Insufficient API credits/quota");
            println!("  • Network connectivity issues");
            println!("\nPlease verify your API key and try again.");
            println!("\nFor a basic system check without AI, use: cargo run -- --dry-run");
            return Ok(());
        }
    }

    // Collect basic system info
    let sys_info = ui_formatter.show_progress("Collecting system information", || {
        collect_basic_system_info()
    });

    // Create comprehensive system context
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

    // Determine the analysis type and create appropriate prompt
    let (analysis_prompt, max_tool_calls) = match (&cli.command, &cli.problem_description) {
        // Specific component check
        (Some(Commands::Check { component }), _) => {
            let component_focus = match component {
                CheckComponent::All => "comprehensive system health check",
                CheckComponent::System => "system information and performance analysis",
                CheckComponent::Containers => "container and Docker analysis",
                CheckComponent::Kubernetes => "Kubernetes cluster analysis",
                CheckComponent::Cgroups => "cgroups and resource management analysis",
                CheckComponent::Systemd => "systemd services and system management analysis",
                CheckComponent::Journal => "system logs and journal analysis",
                CheckComponent::Debug => "debug tools analysis",
            };
            (format!("Perform a focused {} for this system. Analyze the component thoroughly and provide insights on any issues or optimizations.", component_focus), 10)
        },
        // User provided a specific problem description
        (_, Some(problem)) => {
            if cli.ai_agent_mode {
                // Iterative AI agent mode - more tool calls allowed
                (format!("The user has described this problem: '{}'. Help them diagnose and solve this issue by using appropriate diagnostic tools and providing step-by-step guidance.", problem), cli.ai_max_tool_calls)
            } else {
                // Question answering mode - focused analysis
                (format!("The user has a question about their system: '{}'. Analyze their system to provide a helpful answer to their question.", problem), 5)
            }
        },
        // Default comprehensive system check
        _ => {
            ("Analyze this system's health and provide insights on any issues or optimizations. Perform a comprehensive system check.".to_string(), 10)
        }
    };

    // Display appropriate header based on the analysis type
    match (&cli.command, &cli.problem_description) {
        (Some(Commands::Check { component }), _) => {
            println!("🔍 Component Check: {:?}", component);
            println!("🤖 AI Assistant ({})", ai_provider.name());
            println!("Analyzing {} component...\n", component.as_str());
        },
        (_, Some(problem)) => {
            if cli.ai_agent_mode {
                println!("🤖 AI Agent Mode - Iterative Problem Solving");
                println!("Problem: {}", problem);
                println!("Max tool calls: {}", cli.ai_max_tool_calls);
                println!("Starting analysis...\n");
            } else {
                println!("❓ Question: {}", problem);
                println!("🤖 AI Assistant ({})", ai_provider.name());
                println!("Analyzing your question and determining which tools to run...\n");
            }
        },
        _ => {
            println!("🔍 System Health Check");
            println!("🤖 AI Assistant ({})", ai_provider.name());
            println!("Starting comprehensive system analysis...\n");
        }
    }

    // Create AI agent configuration
    let agent_config = AIAgentConfig {
        max_tool_calls,
        pause_on_limit: cli.ai_agent_mode, // Only pause in interactive agent mode
        allow_user_continuation: cli.ai_agent_mode,
        verbose_logging: config.output.verbose || cli.verbose,
    };

    // Create and run the AI agent (always with full tool access)
    let mut agent = ui_formatter.show_progress("Initializing AI agent with tool access", || async {
        AIAgent::new(ai_provider, agent_config).await
    }).await;

    let result = ui_formatter.show_progress("Running AI analysis", || async {
        agent.run(&analysis_prompt, &system_context).await
    }).await?;

    // Handle the result and potential continuation (for interactive agent mode)
    if cli.ai_agent_mode {
        handle_ai_agent_result(result, &mut agent, ui_formatter, config).await?;
    } else {
        // For non-interactive mode, just display the result
        match result {
            AIAgentResult::Success { final_analysis, tool_calls_used } => {
                println!("\n🎯 Analysis Result (used {} tools):", tool_calls_used);
                println!("{}", final_analysis);
                
                if config.output.verbose {
                    println!("\n📊 Tool Usage Summary:");
                    println!("{}", agent.get_conversation_summary());
                }
            }
            AIAgentResult::LimitReached { partial_analysis, tool_calls_used } => {
                println!("\n⚠️  Analysis stopped at tool limit ({} tools used):", tool_calls_used);
                println!("{}", partial_analysis);
            }
            AIAgentResult::Error { error, tool_calls_used } => {
                println!("\n❌ Analysis failed after {} tool calls:", tool_calls_used);
                println!("Error: {}", error);
            }
            AIAgentResult::PausedForUserInput { reason, .. } => {
                // In non-interactive mode, treat pause as completion
                println!("\n🎯 Analysis Result:");
                println!("{}", reason);
            }
        }
    }

    Ok(())
}

/// Handle AI agent results with potential user interaction (for agent mode)
async fn handle_ai_agent_result(
    mut result: AIAgentResult,
    agent: &mut AIAgent,
    ui_formatter: &UIFormatter,
    config: &RaidConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{self, Write};

    loop {
        match result {
            AIAgentResult::Success { final_analysis, tool_calls_used } => {
                println!("\n🎯 Final Analysis (used {} tools):", tool_calls_used);
                println!("{}", final_analysis);
                
                if config.output.verbose {
                    println!("\n📊 Tool Usage Summary:");
                    println!("{}", agent.get_conversation_summary());
                }
                break;
            }
            AIAgentResult::LimitReached { partial_analysis, tool_calls_used } => {
                println!("\n⚠️  Analysis paused at tool limit ({} tools used):", tool_calls_used);
                println!("{}", partial_analysis);
                
                // Ask if user wants to continue
                print!("\nWould you like to continue with more tool calls? (y/n): ");
                io::stdout().flush()?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                
                if input.trim().to_lowercase().starts_with('y') {
                    println!("Continuing analysis...");
                    result = ui_formatter.show_progress("Continuing AI analysis", || async {
                        agent.continue_after_limit().await
                    }).await?;
                } else {
                    println!("Analysis stopped by user.");
                    break;
                }
            }
            AIAgentResult::PausedForUserInput { reason, tool_calls_used } => {
                println!("\n🤖 AI Agent needs more information ({} tools used so far):", tool_calls_used);
                println!("{}", reason);
                
                print!("\nYour response: ");
                io::stdout().flush()?;
                
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input)?;
                
                if !user_input.trim().is_empty() {
                    result = ui_formatter.show_progress("Continuing AI analysis", || async {
                        agent.continue_with_input(user_input.trim()).await
                    }).await?;
                } else {
                    println!("No input provided. Ending analysis.");
                    break;
                }
            }
            AIAgentResult::Error { error, tool_calls_used } => {
                println!("\n❌ Analysis failed after {} tool calls:", tool_calls_used);
                println!("Error: {}", error);
                break;
            }
        }
    }

    Ok(())
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