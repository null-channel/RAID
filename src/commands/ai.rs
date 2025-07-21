use crate::ai::{create_ai_provider_from_cli, AIAgent, AIAgentConfig, AIAgentResult};
use crate::config::RaidConfig;
use crate::sysinfo::collect_basic_system_info;
use crate::ui::UIFormatter;
use std::io::{self, Write};

/// Types of AI analysis to determine prompting strategy
#[derive(Debug, Clone)]
pub enum AnalysisType {
    /// User asked a specific question - focus on answering it
    Question,
    /// System health check - comprehensive analysis
    SystemCheck,
}

pub async fn run_question_answering_with_config(
    config: &RaidConfig,
    question: &str,
    ui_formatter: &UIFormatter,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if AI API key is available
    if config.ai.api_key.is_none() {
        println!("❌ No AI API key found. Question answering requires an AI provider.");
        println!("Please set your AI_API_KEY environment variable or use --ai-api-key flag.");
        println!("Supported providers: OpenAI, Anthropic, Local (Ollama)");
        println!("\nFor a basic system check without AI, use: cargo run -- --dry-run");
        return Ok(());
    }

    // Test AI provider connection before proceeding
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

    // Quick test to verify the AI provider is working
    match ai_provider.analyze("test").await {
        Ok(_) => {
            // Provider is working, proceed with question answering
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

    run_unified_ai_analysis(config, question, ui_formatter, AnalysisType::Question).await
}

pub async fn run_ai_agent_mode(
    config: &RaidConfig,
    problem_description: &str,
    ui_formatter: &UIFormatter,
    max_tool_calls: usize,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if AI API key is available
    if config.ai.api_key.is_none() {
        println!("❌ No AI API key found. AI Agent mode requires an AI provider.");
        println!("Please set your AI_API_KEY environment variable or use --ai-api-key flag.");
        println!("Supported providers: OpenAI, Anthropic, Local (Ollama)");
        println!("\nFor a basic system check without AI, use: cargo run -- --dry-run");
        return Ok(());
    }

    let ai_provider = create_ai_provider_from_cli(
        &config.get_ai_provider(),
        config.ai.api_key.clone(),
        Some(config.get_model()),
        config.ai.base_url.clone(),
        config.ai.max_tokens,
        config.ai.temperature,
    )
    .await?;

    println!("🤖 AI Agent Mode - Iterative Problem Solving");
    println!("Problem: {}", problem_description);
    println!("Max tool calls: {}", max_tool_calls);
    println!("Starting analysis...\n");

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

    // Create AI agent configuration
    let agent_config = AIAgentConfig {
        max_tool_calls,
        pause_on_limit: true,
        allow_user_continuation: true,
        verbose_logging: config.output.verbose,
    };

    // Create and run the AI agent
    let mut agent = ui_formatter.show_progress("Initializing AI agent", || async {
        AIAgent::new(ai_provider, agent_config).await
    }).await;

    // Run the agent
    let mut result = ui_formatter.show_progress("Running AI agent analysis", || async {
        agent.run(problem_description, &system_context).await
    }).await?;

    // Handle the result and potential continuation
    loop {
        match result {
            AIAgentResult::Success { final_analysis, tool_calls_used } => {
                println!("\n🎯 Final Analysis (after {} tool calls):", tool_calls_used);
                println!("{}", final_analysis);
                
                // Show conversation summary if verbose
                if config.output.verbose {
                    println!("\n📊 Conversation Summary:");
                    println!("{}", agent.get_conversation_summary());
                }
                break;
            }
            AIAgentResult::PausedForUserInput { reason, tool_calls_used } => {
                println!("\n⏸️  AI Agent paused after {} tool calls", tool_calls_used);
                println!("Reason: {}", reason);
                
                print!("\nYour response (or 'quit' to exit): ");
                io::stdout().flush()?;
                
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input)?;
                let user_input = user_input.trim();
                
                if user_input.to_lowercase() == "quit" {
                    println!("Exiting AI agent mode.");
                    break;
                }
                
                // Continue with user input
                result = agent.continue_with_input(user_input).await?;
            }
            AIAgentResult::LimitReached { partial_analysis, tool_calls_used } => {
                println!("\n⚠️  Tool call limit reached after {} calls", tool_calls_used);
                println!("{}", partial_analysis);
                
                print!("\nContinue with {} more tool calls? (y/n): ", max_tool_calls);
                io::stdout().flush()?;
                
                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input)?;
                
                if user_input.trim().to_lowercase().starts_with('y') {
                    println!("Continuing with {} more tool calls...", max_tool_calls);
                    result = agent.continue_after_limit().await?;
                } else {
                    println!("Analysis stopped by user.");
                    break;
                }
            }
            AIAgentResult::Error { error, tool_calls_used } => {
                println!("\n❌ AI Agent encountered an error after {} tool calls:", tool_calls_used);
                println!("Error: {}", error);
                break;
            }
        }
    }

    Ok(())
}

/// Unified AI interaction system that always uses tools but with different prompting strategies
pub async fn run_unified_ai_analysis(
    config: &RaidConfig,
    user_input: &str,
    ui_formatter: &UIFormatter,
    analysis_type: AnalysisType,
) -> Result<(), Box<dyn std::error::Error>> {
    // Check if AI API key is available
    if config.ai.api_key.is_none() {
        println!("❌ No AI API key found. AI analysis requires an AI provider.");
        println!("Please set your AI_API_KEY environment variable or use --ai-api-key flag.");
        println!("Supported providers: OpenAI, Anthropic, Local (Ollama)");
        println!("\nFor a basic system check without AI, use: cargo run -- --dry-run");
        return Ok(());
    }

    // Test AI provider connection before proceeding
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

    // Quick test to verify the AI provider is working
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

    // Different headers based on analysis type
    match analysis_type {
        AnalysisType::Question => {
            println!("❓ Question: {}", user_input);
            println!("🤖 AI Assistant ({})", ai_provider.name());
            println!("Analyzing your question and determining which tools to run...\n");
        }
        AnalysisType::SystemCheck => {
            println!("🔍 System Health Check");
            println!("🤖 AI Assistant ({})", ai_provider.name());
            println!("Starting comprehensive system analysis...\n");
        }
    }

    // Always use the agent-based approach for consistency
    let agent_config = AIAgentConfig {
        max_tool_calls: match analysis_type {
            AnalysisType::Question => 5, // Focused questions need fewer tools
            AnalysisType::SystemCheck => 10, // System checks might need more comprehensive analysis
        },
        pause_on_limit: false,
        allow_user_continuation: false,
        verbose_logging: config.output.verbose,
    };

    // Collect basic system info
    let sys_info = ui_formatter.show_progress("Collecting system information", || {
        collect_basic_system_info()
    });

    // Create system context
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

    // Create and run agent
    let mut agent = ui_formatter.show_progress("Initializing AI agent", || async {
        AIAgent::new(ai_provider, agent_config).await
    }).await;

    let result = ui_formatter.show_progress("Running AI analysis", || async {
        agent.run(user_input, &system_context).await
    }).await?;

    // Display results
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
        AIAgentResult::PausedForUserInput { .. } => {
            // This shouldn't happen with our config, but handle it anyway
            println!("\n⚠️  Unexpected pause in non-interactive mode");
        }
    }

    Ok(())
} 