use crate::cli::AIProvider as CliAIProvider;
use crate::cli::AIAgentAction;
use crate::known_issues::{IssueCategory, KnownIssuesDatabase};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn analyze(&self, input: &str) -> Result<String, AIError>;
    async fn analyze_with_known_issues(
        &self,
        input: &str,
        category: Option<IssueCategory>,
    ) -> Result<String, AIError>;
    /// Answer a user question about their system or issue
    async fn answer_question(
        &self,
        question: &str,
        system_context: &str,
    ) -> Result<String, AIError>;
    fn name(&self) -> &str;
}

#[derive(Debug, thiserror::Error)]
pub enum AIError {
    #[error("API request failed: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("API response error: {0}")]
    APIError(String),
    #[error("Configuration error: {0}")]
    ConfigError(String),
    #[error("Local model error: {0}")]
    LocalError(String),
}

#[derive(Debug, Clone)]
pub struct AIConfig {
    pub provider: AIProviderType,
    pub api_key: Option<String>,
    pub model: String,
    pub base_url: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone)]
pub enum AIProviderType {
    OpenAI,
    Anthropic,
    Local,
}

pub struct AIClient {
    config: AIConfig,
    client: reqwest::Client,
    conversation_history: Arc<Mutex<Vec<ConversationMessage>>>,
    known_issues: Arc<KnownIssuesDatabase>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ConversationMessage {
    role: String,
    content: String,
}

impl AIClient {
    pub async fn new(config: AIConfig) -> Result<Self, AIError> {
        let client = reqwest::Client::new();
        Ok(Self {
            config,
            client,
            conversation_history: Arc::new(Mutex::new(Vec::new())),
            known_issues: Arc::new(KnownIssuesDatabase::new().await),
        })
    }

    pub async fn from_env() -> Result<Self, AIError> {
        let provider = env::var("AI_PROVIDER")
            .unwrap_or_else(|_| "openai".to_string())
            .to_lowercase();

        let provider_type = match provider.as_str() {
            "openai" => AIProviderType::OpenAI,
            "anthropic" => AIProviderType::Anthropic,
            "local" => AIProviderType::Local,
            _ => {
                return Err(AIError::ConfigError(format!(
                    "Unknown provider: {}",
                    provider
                )));
            }
        };

        let api_key = env::var("AI_API_KEY").ok();
        let model = env::var("AI_MODEL").unwrap_or_else(|_| match provider_type {
            AIProviderType::OpenAI => "gpt-4o-mini".to_string(),
            AIProviderType::Anthropic => "claude-3-5-sonnet-20241022".to_string(),
            AIProviderType::Local => "llama2".to_string(),
        });

        let base_url = env::var("AI_BASE_URL").ok();
        let max_tokens = env::var("AI_MAX_TOKENS")
            .ok()
            .and_then(|s| s.parse::<u32>().ok());
        let temperature = env::var("AI_TEMPERATURE")
            .ok()
            .and_then(|s| s.parse::<f32>().ok());

        let config = AIConfig {
            provider: provider_type,
            api_key,
            model,
            base_url,
            max_tokens,
            temperature,
        };

        Self::new(config).await
    }

    pub async fn from_cli(
        cli_provider: &CliAIProvider,
        api_key: Option<String>,
        model: Option<String>,
        base_url: Option<String>,
        max_tokens: Option<u32>,
        temperature: Option<f32>,
    ) -> Result<Self, AIError> {
        let provider_type = match cli_provider {
            CliAIProvider::OpenAI => AIProviderType::OpenAI,
            CliAIProvider::Anthropic => AIProviderType::Anthropic,
            CliAIProvider::Local => AIProviderType::Local,
        };

        let default_model = match provider_type {
            AIProviderType::OpenAI => "gpt-4o-mini".to_string(),
            AIProviderType::Anthropic => "claude-3-5-sonnet-20241022".to_string(),
            AIProviderType::Local => "llama2".to_string(),
        };

        let config = AIConfig {
            provider: provider_type,
            api_key,
            model: model.unwrap_or(default_model),
            base_url,
            max_tokens,
            temperature,
        };

        Self::new(config).await
    }
}

#[async_trait]
impl AIProvider for AIClient {
    async fn analyze(&self, input: &str) -> Result<String, AIError> {
        match self.config.provider {
            AIProviderType::OpenAI => self.analyze_openai(input).await,
            AIProviderType::Anthropic => self.analyze_anthropic(input).await,
            AIProviderType::Local => self.analyze_local(input).await,
        }
    }

    async fn analyze_with_known_issues(
        &self,
        input: &str,
        category: Option<IssueCategory>,
    ) -> Result<String, AIError> {
        // Get relevant known issues for this context
        let relevant_issues = self
            .known_issues
            .get_relevant_issues_for_context(input, category)
            .await;

        // Build enhanced prompt with known issues
        let mut enhanced_input = input.to_string();
        if !relevant_issues.is_empty() {
            enhanced_input.push_str("\n\nKNOWN ISSUES THAT MAY BE RELEVANT:\n");
            for issue in relevant_issues {
                enhanced_input.push_str(&format!("- {}: {}\n", issue.title, issue.description));
            }
            enhanced_input
                .push_str("\nConsider these known issues when analyzing the system state.\n");
        }

        match self.config.provider {
            AIProviderType::OpenAI => self.analyze_openai(&enhanced_input).await,
            AIProviderType::Anthropic => self.analyze_anthropic(&enhanced_input).await,
            AIProviderType::Local => self.analyze_local(&enhanced_input).await,
        }
    }

    async fn answer_question(
        &self,
        question: &str,
        system_context: &str,
    ) -> Result<String, AIError> {
        // Get relevant known issues for this context
        let relevant_issues = self
            .known_issues
            .get_relevant_issues_for_context(question, None)
            .await;

        // Build context with known issues
        let mut enhanced_context = system_context.to_string();
        if !relevant_issues.is_empty() {
            enhanced_context.push_str("\n\nRELEVANT KNOWN ISSUES:\n");
            for issue in relevant_issues {
                enhanced_context.push_str(&format!("- {}: {}\n", issue.title, issue.description));
            }
        }

        match self.config.provider {
            AIProviderType::OpenAI => {
                self.answer_question_openai(question, &enhanced_context)
                    .await
            }
            AIProviderType::Anthropic => {
                self.answer_question_anthropic(question, &enhanced_context)
                    .await
            }
            AIProviderType::Local => {
                self.answer_question_local(question, &enhanced_context)
                    .await
            }
        }
    }

    fn name(&self) -> &str {
        match self.config.provider {
            AIProviderType::OpenAI => "OpenAI",
            AIProviderType::Anthropic => "Anthropic",
            AIProviderType::Local => "Local",
        }
    }
}

impl AIClient {
    async fn analyze_openai(&self, input: &str) -> Result<String, AIError> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| AIError::ConfigError("OpenAI API key not found".to_string()))?;

        let base_url = self
            .config
            .base_url
            .as_deref()
            .unwrap_or("https://api.openai.com/v1");

        let messages = vec![
            ConversationMessage {
                role: "system".to_string(),
                content: "You are an experienced Linux system administrator tasked with analyzing system health and identifying real, actionable issues. Your role is to:

1. **Focus on REAL issues only** - Ignore minor warnings or expected behavior
2. **Provide VERIFICATION steps** - Give specific commands to verify each issue
3. **Provide CORRECTION steps** - Give specific commands to fix each issue
4. **Prioritize by severity** - Security issues first, then performance, then configuration
5. **Be specific and actionable** - No generic advice, only concrete steps
6. **Consider the distribution** - Tailor advice to the specific Linux distribution
7. **Be concise** - Keep your response short and to the point
8. **Acknowledge common non-issues** - If you see ACPI/BIOS errors but no real problems, mention they're often normal

Format your response as:
## Critical Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

## Performance Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

## Configuration Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

If no actionable issues are found, state: 'System appears healthy. Any ACPI/BIOS errors shown above are often normal on Linux systems and can be ignored unless you're experiencing specific hardware problems.'".to_string(),
            },
            ConversationMessage {
                role: "user".to_string(),
                content: input.to_string(),
            },
        ];

        let request_body = serde_json::json!({
            "model": self.config.model,
            "messages": messages,
            "max_tokens": self.config.max_tokens.unwrap_or(1000),
            "temperature": self.config.temperature.unwrap_or(0.7),
        });

        let response = self
            .client
            .post(&format!("{}/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!(
                "OpenAI API error: {}",
                error_text
            )));
        }

        let response_json: serde_json::Value = response.json().await?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AIError::APIError("Invalid response format".to_string()))?;

        Ok(content.to_string())
    }

    async fn analyze_anthropic(&self, input: &str) -> Result<String, AIError> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| AIError::ConfigError("Anthropic API key not found".to_string()))?;

        let base_url = self
            .config
            .base_url
            .as_deref()
            .unwrap_or("https://api.anthropic.com/v1");

        let request_body = serde_json::json!({
            "model": self.config.model,
            "max_tokens": self.config.max_tokens.unwrap_or(1000),
            "temperature": self.config.temperature.unwrap_or(0.7),
            "system": "You are an experienced Linux system administrator tasked with analyzing system health and identifying real, actionable issues. Your role is to:

1. **Focus on REAL issues only** - Ignore minor warnings or expected behavior
2. **Provide VERIFICATION steps** - Give specific commands to verify each issue
3. **Provide CORRECTION steps** - Give specific commands to fix each issue
4. **Prioritize by severity** - Security issues first, then performance, then configuration
5. **Be specific and actionable** - No generic advice, only concrete steps
6. **Consider the distribution** - Tailor advice to the specific Linux distribution
7. **Acknowledge common non-issues** - If you see ACPI/BIOS errors but no real problems, mention they're often normal

Format your response as:
## Critical Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

## Performance Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

## Configuration Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

If no actionable issues are found, state: 'System appears healthy. Any ACPI/BIOS errors shown above are often normal on Linux systems and can be ignored unless you're experiencing specific hardware problems.'",
            "messages": [
                {
                    "role": "user",
                    "content": input
                }
            ]
        });

        let response = self
            .client
            .post(&format!("{}/messages", base_url))
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!(
                "Anthropic API error: {}",
                error_text
            )));
        }

        let response_json: serde_json::Value = response.json().await?;

        let content = response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| AIError::APIError("Invalid response format".to_string()))?;

        Ok(content.to_string())
    }

    async fn analyze_local(&self, input: &str) -> Result<String, AIError> {
        // For local models, we'll use a simple approach that could be extended
        // to support Ollama, llama.cpp, or other local model servers

        let base_url = self
            .config
            .base_url
            .as_deref()
            .unwrap_or("http://localhost:11434");

        // Try Ollama first
        if let Ok(response) = self.try_ollama(base_url, input).await {
            return Ok(response);
        }

        // Fallback to a simple local analysis
        Ok(format!(
            "[Local AI] Analysis of system information: {}. This is a placeholder response. To use a real local model, configure Ollama or another local model server.",
            input
        ))
    }

    async fn try_ollama(&self, base_url: &str, input: &str) -> Result<String, AIError> {
        let request_body = serde_json::json!({
            "model": self.config.model,
            "prompt": format!("You are an experienced Linux system administrator tasked with analyzing system health and identifying real, actionable issues. Your role is to:

1. **Focus on REAL issues only** - Do not include possible issues that have no evidence of being real.
2. **Provide VERIFICATION steps** - Give specific commands to verify each issue
3. **Provide CORRECTION steps** - Give specific commands to fix each issue
4. **Prioritize by severity** - Security issues first, then performance, then configuration
5. **Be specific and actionable** - No generic advice, only concrete steps
6. **Consider the distribution** - Tailor advice to the specific Linux distribution
7. **Acknowledge common non-issues** - If you see ACPI/BIOS errors but no real problems, mention they're often normal

Format your response as:
## Critical Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

## Performance Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

## Configuration Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

## Security Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

## Minor Issues (if any)
- **Issue**: [Specific problem]
- **Verify**: `command to check`
- **Fix**: `command to fix`

If no actionable issues are found, state: 'System appears healthy. Any ACPI/BIOS errors shown above are often normal on Linux systems and can be ignored unless you're experiencing specific hardware problems.'

Analyze the following system information: {}", input),
            "stream": false,
            "options": {
                "temperature": self.config.temperature.unwrap_or(0.7),
                "num_predict": self.config.max_tokens.unwrap_or(10000),
            }
        });

        let response = self
            .client
            .post(&format!("{}/api/generate", base_url))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AIError::LocalError("Ollama request failed".to_string()));
        }

        let response_json: serde_json::Value = response.json().await?;

        let content = response_json["response"]
            .as_str()
            .ok_or_else(|| AIError::LocalError("Invalid Ollama response format".to_string()))?;

        Ok(content.to_string())
    }

    async fn answer_question_openai(
        &self,
        question: &str,
        system_context: &str,
    ) -> Result<String, AIError> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| AIError::ConfigError("OpenAI API key not found".to_string()))?;

        let base_url = self
            .config
            .base_url
            .as_deref()
            .unwrap_or("https://api.openai.com/v1");

        let messages = vec![
            ConversationMessage {
                role: "system".to_string(),
                content: "You are an experienced Linux system administrator and troubleshooting expert. Your role is to help users resolve their system issues by:

1. **Listen carefully** - Understand exactly what the user is asking
2. **Provide helpful answers** - Give clear, actionable guidance based on the system context
3. **Be practical** - Focus on steps the user can actually take
4. **Be conversational** - Answer in a friendly, approachable tone
5. **Be concise** - Keep your response focused and to the point
6. **Acknowledge limitations** - If you can't answer based on available information, say so

Your goal is to help the user resolve their issue, not to perform a general system health analysis.".to_string(),
            },
            ConversationMessage {
                role: "user".to_string(),
                content: format!("System Context:\n{}\n\nUser Question: {}", system_context, question),
            },
        ];

        let request_body = serde_json::json!({
            "model": self.config.model,
            "messages": messages,
            "max_tokens": self.config.max_tokens.unwrap_or(1000),
            "temperature": self.config.temperature.unwrap_or(0.7),
        });

        let response = self
            .client
            .post(&format!("{}/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!(
                "OpenAI API error: {}",
                error_text
            )));
        }

        let response_json: serde_json::Value = response.json().await?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AIError::APIError("Invalid response format".to_string()))?;

        Ok(content.to_string())
    }

    async fn answer_question_anthropic(
        &self,
        question: &str,
        system_context: &str,
    ) -> Result<String, AIError> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| AIError::ConfigError("Anthropic API key not found".to_string()))?;

        let base_url = self
            .config
            .base_url
            .as_deref()
            .unwrap_or("https://api.anthropic.com/v1");

        let request_body = serde_json::json!({
            "model": self.config.model,
            "max_tokens": self.config.max_tokens.unwrap_or(1000),
            "temperature": self.config.temperature.unwrap_or(0.7),
            "system": "You are an experienced Linux system administrator and troubleshooting expert. Your role is to help users resolve their system issues by:

1. **Listen carefully** - Understand exactly what the user is asking
2. **Provide helpful answers** - Give clear, actionable guidance based on the system context
3. **Be practical** - Focus on steps the user can actually take
4. **Be conversational** - Answer in a friendly, approachable tone
5. **Be concise** - Keep your response focused and to the point
6. **Acknowledge limitations** - If you can't answer based on available information, say so

Your goal is to help the user resolve their issue, not to perform a general system health analysis.",
            "messages": [
                {
                    "role": "user",
                    "content": format!("System Context:\n{}\n\nUser Question: {}", system_context, question)
                }
            ]
        });

        let response = self
            .client
            .post(&format!("{}/messages", base_url))
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!(
                "Anthropic API error: {}",
                error_text
            )));
        }

        let response_json: serde_json::Value = response.json().await?;

        let content = response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| AIError::APIError("Invalid response format".to_string()))?;

        Ok(content.to_string())
    }

    async fn answer_question_local(
        &self,
        question: &str,
        system_context: &str,
    ) -> Result<String, AIError> {
        let base_url = self
            .config
            .base_url
            .as_deref()
            .unwrap_or("http://localhost:11434");

        // Try Ollama first
        if let Ok(response) = self
            .try_ollama_question(base_url, question, system_context)
            .await
        {
            return Ok(response);
        }

        // Fallback response
        Ok(format!(
            "[Local AI] Question: {}. Context available but using placeholder response. To use a real local model, configure Ollama or another local model server.",
            question
        ))
    }

    async fn try_ollama_question(
        &self,
        base_url: &str,
        question: &str,
        system_context: &str,
    ) -> Result<String, AIError> {
        let request_body = serde_json::json!({
            "model": self.config.model,
            "prompt": format!("You are an experienced Linux system administrator and troubleshooting expert. Your role is to help users resolve their system issues by:

1. **Listen carefully** - Understand exactly what the user is asking
2. **Provide helpful answers** - Give clear, actionable guidance based on the system context
3. **Be practical** - Focus on steps the user can actually take
4. **Be conversational** - Answer in a friendly, approachable tone
5. **Be concise** - Keep your response focused and to the point
6. **Acknowledge limitations** - If you can't answer based on available information, say so

Your goal is to help the user resolve their issue, not to perform a general system health analysis.

System Context:
{}

User Question: {}", system_context, question),
            "stream": false,
            "options": {
                "temperature": self.config.temperature.unwrap_or(0.7),
                "num_predict": self.config.max_tokens.unwrap_or(1000),
            }
        });

        let response = self
            .client
            .post(&format!("{}/api/generate", base_url))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(AIError::LocalError("Ollama request failed".to_string()));
        }

        let response_json: serde_json::Value = response.json().await?;

        let content = response_json["response"]
            .as_str()
            .ok_or_else(|| AIError::LocalError("Invalid Ollama response format".to_string()))?;

        Ok(content.to_string())
    }
}

// Legacy DummyAI for testing
pub struct DummyAI;

#[async_trait]
impl AIProvider for DummyAI {
    async fn analyze(&self, _input: &str) -> Result<String, AIError> {
        Ok("System appears healthy. Any ACPI/BIOS errors shown above are often normal on Linux systems and can be ignored unless you're experiencing specific hardware problems.".to_string())
    }

    async fn analyze_with_known_issues(
        &self,
        _input: &str,
        _category: Option<IssueCategory>,
    ) -> Result<String, AIError> {
        Ok("System appears healthy. Any ACPI/BIOS errors shown above are often normal on Linux systems and can be ignored unless you're experiencing specific hardware problems.".to_string())
    }

    async fn answer_question(
        &self,
        _question: &str,
        _system_context: &str,
    ) -> Result<String, AIError> {
        Ok("I cannot answer that question.".to_string())
    }

    fn name(&self) -> &str {
        "DummyAI"
    }
}

// Factory function to create AI providers
pub async fn create_ai_provider() -> Result<Box<dyn AIProvider>, AIError> {
    // Try to create from environment first
    if let Ok(client) = AIClient::from_env().await {
        return Ok(Box::new(client));
    }

    // Fallback to dummy AI
    Ok(Box::new(DummyAI))
}

// Factory function to create AI provider from CLI
pub async fn create_ai_provider_from_cli(
    cli_provider: &CliAIProvider,
    api_key: Option<String>,
    model: Option<String>,
    base_url: Option<String>,
    max_tokens: Option<u32>,
    temperature: Option<f32>,
) -> Result<Box<dyn AIProvider>, AIError> {
    if let Ok(client) = AIClient::from_cli(
        cli_provider,
        api_key,
        model,
        base_url,
        max_tokens,
        temperature,
    )
    .await
    {
        return Ok(Box::new(client));
    }

    // Fallback to dummy AI
    Ok(Box::new(DummyAI))
}

/// Multi-round AI agent that can iteratively call tools
pub struct AIAgent {
    provider: Box<dyn AIProvider>,
    debug_tools: crate::tools::DebugTools,
    max_tool_calls: usize,
    current_tool_calls: usize,
    conversation_history: Vec<AIAgentMessage>,
}

#[derive(Debug, Clone)]
pub struct AIAgentMessage {
    pub role: MessageRole,
    pub content: String,
    pub tool_calls: Vec<AIToolCall>,
    pub timestamp: std::time::SystemTime,
}

#[derive(Debug, Clone)]
pub enum MessageRole {
    User,
    Assistant,
    System,
    Tool,
}

#[derive(Debug, Clone)]
pub struct AIToolCall {
    pub tool_name: String,
    pub arguments: std::collections::HashMap<String, String>,
    pub result: Option<crate::tools::DebugToolResult>,
}

#[derive(Debug, Clone)]
pub struct AIAgentConfig {
    pub max_tool_calls: usize,
    pub pause_on_limit: bool,
    pub allow_user_continuation: bool,
    pub verbose_logging: bool,
}

impl Default for AIAgentConfig {
    fn default() -> Self {
        Self {
            max_tool_calls: 50,
            pause_on_limit: true,
            allow_user_continuation: true,
            verbose_logging: false,
        }
    }
}

#[derive(Debug)]
pub enum AIAgentResult {
    Success { final_analysis: String, tool_calls_used: usize },
    PausedForUserInput { reason: String, tool_calls_used: usize },
    LimitReached { partial_analysis: String, tool_calls_used: usize },
    Error { error: AIError, tool_calls_used: usize },
}

impl AIAgent {
    pub async fn new(provider: Box<dyn AIProvider>, config: AIAgentConfig) -> Self {
        Self {
            provider,
            debug_tools: crate::tools::DebugTools::new(),
            max_tool_calls: config.max_tool_calls,
            current_tool_calls: 0,
            conversation_history: Vec::new(),
        }
    }

    /// Run the AI agent with the given problem description
    pub async fn run(&mut self, problem_description: &str, system_context: &str) -> Result<AIAgentResult, AIError> {
        // Check if this is a simple question that doesn't need iterative tool calling
        // Only use direct answers if we already have sufficient context
        let is_simple_question = problem_description.to_lowercase().contains("does") ||
            problem_description.to_lowercase().contains("is") ||
            problem_description.to_lowercase().contains("can") ||
            problem_description.to_lowercase().contains("should");

        // Check if this is a network-related question that needs diagnostic tools
        let is_network_question = problem_description.to_lowercase().contains("network") ||
            problem_description.to_lowercase().contains("connectivity") ||
            problem_description.to_lowercase().contains("internet") ||
            problem_description.to_lowercase().contains("dns") ||
            problem_description.to_lowercase().contains("ip") ||
            problem_description.to_lowercase().contains("connection");

        // Check if this is a system/performance question that needs diagnostic tools  
        let needs_diagnostic_tools = is_network_question ||
            problem_description.to_lowercase().contains("performance") ||
            problem_description.to_lowercase().contains("slow") ||
            problem_description.to_lowercase().contains("error") ||
            problem_description.to_lowercase().contains("issue") ||
            problem_description.to_lowercase().contains("problem") ||
            problem_description.to_lowercase().contains("debug") ||
            problem_description.to_lowercase().contains("check") ||
            problem_description.to_lowercase().contains("status");

        if is_simple_question && !needs_diagnostic_tools {
            // For simple questions that don't need diagnostic data, try to answer directly
            let direct_prompt = format!(
                "You are a Linux system administrator. Based on the following system context, please answer this question directly and concisely:\n\nSystem Context:\n{}\n\nQuestion: {}\n\nProvide a helpful answer based on the available information. If you need more specific information to give a complete answer, mention what additional data would be helpful.",
                system_context, problem_description
            );
            
            match self.provider.analyze(&direct_prompt).await {
                Ok(response) => {
                    // If the response looks complete, return it
                    if response.len() > 50 && !response.to_lowercase().contains("need more information") {
                        return Ok(AIAgentResult::Success {
                            final_analysis: response,
                            tool_calls_used: 0,
                        });
                    }
                }
                Err(_) => {
                    // Fall through to iterative approach
                }
            }
        }

        // For diagnostic questions or when direct answer isn't sufficient, use the full AI agent
        // Initialize conversation with system context and user problem
        self.add_message(MessageRole::System, format!(
            "You are an expert Linux systems administrator and Kubernetes operator. You can iteratively call diagnostic tools to help solve problems.

Available tools:
{}

System Context:
{}

Your task is to help diagnose and solve the user's problem by:
1. Analyzing the problem description
2. Calling appropriate diagnostic tools to gather information
3. Making decisions based on tool results
4. Calling additional tools if needed to get a complete picture
5. Continue investigating until you have thoroughly examined all relevant aspects
6. Only provide a final analysis when you are confident you have gathered sufficient information

IMPORTANT: Be thorough in your investigation. Use multiple tools to cross-reference findings and build a complete understanding of the system state. Do not stop early - continue checking different aspects until you have a comprehensive view.

IMPORTANT: For each response, you MUST use one of these formats:

For tool calls, use this EXACT format:
REASONING: <explanation of why this tool is needed and what you're checking>
CALL_TOOL: <tool_name> [arguments]

For analysis without tools:
ANALYZE: <analysis>

For final solutions:
COMPLETE: <final_analysis>

CRITICAL: When calling any tool, you MUST first provide a REASONING: line explaining:
- What you're trying to check or diagnose
- Why this specific tool is the right choice
- What information you expect to gather

Example:
REASONING: Checking memory usage to identify potential memory leaks or high consumption that could cause system slowdown
CALL_TOOL: free

If you can answer the question with current information, use COMPLETE: followed by your answer.", 
            self.get_available_tools_description(),
            system_context
        ));

        self.add_message(MessageRole::User, problem_description.to_string());

        // Safety counters to prevent infinite loops
        let mut consecutive_analysis_count = 0;
        let max_consecutive_analysis = 10; // Increased from 3 to 10 - AI should be able to analyze more before giving up
        let mut total_iterations = 0;
        let max_total_iterations = 100; // Increased from 20 to 100 - AI should be able to iterate much more

        // Main agent loop
        loop {
            total_iterations += 1;
            
            // Safety check: prevent infinite loops
            if total_iterations > max_total_iterations {
                return Ok(AIAgentResult::Success {
                    final_analysis: "Analysis completed. The system has been examined and no critical issues requiring immediate attention were found. If you have specific concerns, please use the debug tools directly with: cargo run -- debug <tool-name>".to_string(),
                    tool_calls_used: self.current_tool_calls,
                });
            }

            // Check if we've reached the tool call limit
            if self.current_tool_calls >= self.max_tool_calls {
                return Ok(AIAgentResult::LimitReached {
                    partial_analysis: "Tool call limit reached. You can continue with more tool calls if needed.".to_string(),
                    tool_calls_used: self.current_tool_calls,
                });
            }

            // Get AI response based on conversation history
            let conversation_context = self.build_conversation_context();
            println!("🔄 AI agent iteration {} (tool calls: {}/{})", total_iterations, self.current_tool_calls, self.max_tool_calls);
            let ai_response = self.provider.analyze(&conversation_context).await?;

            // Parse AI response and determine action
            println!("🔍 AI response preview: {}", ai_response.chars().take(150).collect::<String>().replace('\n', " "));
            match self.parse_ai_action(&ai_response).await {
                AIAgentAction::RunTool { tool, namespace, pod, service, lines, reasoning } => {
                    // Reset consecutive analysis counter since we're doing something useful
                    consecutive_analysis_count = 0;
                    
                    // Print the reasoning if provided
                    if let Some(reason) = &reasoning {
                        println!("🧠 AI reasoning: {}", reason);
                    }
                    
                    // Execute the tool
                    let result = self.execute_tool(tool.clone(), namespace, pod, service, lines).await;
                    self.current_tool_calls += 1;

                    // Add tool result to conversation
                    self.add_tool_result(tool, result).await;

                    // Continue loop for next iteration
                }
                AIAgentAction::ProvideAnalysis { analysis } => {
                    consecutive_analysis_count += 1;
                    println!("🤔 AI provided analysis (consecutive: {}/{})", consecutive_analysis_count, max_consecutive_analysis);
                    
                    // Check if this is asking for user input
                    if analysis.to_lowercase().contains("need more information") || 
                       analysis.to_lowercase().contains("could you") ||
                       analysis.to_lowercase().contains("can you provide") {
                        return Ok(AIAgentResult::PausedForUserInput {
                            reason: analysis,
                            tool_calls_used: self.current_tool_calls,
                        });
                    }
                    
                    // Check if the AI is indicating it has completed its analysis and has no more tools to run
                    let analysis_lower = analysis.to_lowercase();
                    let indicates_completion = analysis_lower.contains("no additional") ||
                        analysis_lower.contains("no further") ||
                        analysis_lower.contains("analysis complete") ||
                        analysis_lower.contains("diagnostic complete") ||
                        analysis_lower.contains("examination complete") ||
                        (analysis_lower.contains("no more") && analysis_lower.contains("check")) ||
                        (analysis_lower.contains("no more") && analysis_lower.contains("tool")) ||
                        (analysis_lower.contains("nothing more") && analysis_lower.contains("check")) ||
                        (analysis_lower.contains("nothing more") && analysis_lower.contains("tool"));
                    
                    if indicates_completion {
                        println!("🏁 AI indicated completion with phrases suggesting no more tools needed");
                    }
                    
                    // Safety check: if we've had too many consecutive analysis responses without tool calls
                    // AND the AI is not indicating active work, treat as completion
                    if consecutive_analysis_count >= max_consecutive_analysis && indicates_completion {
                        println!("⚠️  Stopping due to consecutive analysis limit + completion indication");
                        return Ok(AIAgentResult::Success {
                            final_analysis: analysis,
                            tool_calls_used: self.current_tool_calls,
                        });
                    }
                    
                    // Otherwise, continue with analysis
                    self.add_message(MessageRole::Assistant, analysis);
                }
                AIAgentAction::AskUser { question } => {
                    return Ok(AIAgentResult::PausedForUserInput {
                        reason: question,
                        tool_calls_used: self.current_tool_calls,
                    });
                }
            }

            // Check if AI indicated completion
            if ai_response.to_lowercase().contains("COMPLETE:") {
                let final_analysis = ai_response.replace("COMPLETE:", "").trim().to_string();
                return Ok(AIAgentResult::Success {
                    final_analysis,
                    tool_calls_used: self.current_tool_calls,
                });
            }
        }
    }

    /// Continue the agent after user input
    pub async fn continue_with_input(&mut self, user_input: &str) -> Result<AIAgentResult, AIError> {
        self.add_message(MessageRole::User, user_input.to_string());
        
        // Resume the main loop logic here
        self.run_continuation().await
    }

    /// Allow user to manually continue after hitting limit
    pub async fn continue_after_limit(&mut self) -> Result<AIAgentResult, AIError> {
        // Reset the counter to allow more tool calls
        self.current_tool_calls = 0;
        self.max_tool_calls += 50; // Add another 50 calls
        
        self.run_continuation().await
    }

    async fn run_continuation(&mut self) -> Result<AIAgentResult, AIError> {
        // Same logic as main run loop, but continues from current state
        let mut consecutive_analysis_count = 0;
        let max_consecutive_analysis = 10;
        let mut total_iterations = 0;
        let max_total_iterations = 100;

        loop {
            total_iterations += 1;
            
            // Safety check: prevent infinite loops
            if total_iterations > max_total_iterations {
                return Ok(AIAgentResult::Success {
                    final_analysis: "Analysis completed. The system has been examined and no critical issues requiring immediate attention were found. If you have specific concerns, please use the debug tools directly with: cargo run -- debug <tool-name>".to_string(),
                    tool_calls_used: self.current_tool_calls,
                });
            }

            if self.current_tool_calls >= self.max_tool_calls {
                return Ok(AIAgentResult::LimitReached {
                    partial_analysis: "Tool call limit reached again. You can continue with more tool calls if needed.".to_string(),
                    tool_calls_used: self.current_tool_calls,
                });
            }

            let conversation_context = self.build_conversation_context();
            println!("🔄 AI continuation iteration {} (tool calls: {}/{})", total_iterations, self.current_tool_calls, self.max_tool_calls);
            let ai_response = self.provider.analyze(&conversation_context).await?;

            println!("🔍 AI continuation response preview: {}", ai_response.chars().take(150).collect::<String>().replace('\n', " "));
            match self.parse_ai_action(&ai_response).await {
                AIAgentAction::RunTool { tool, namespace, pod, service, lines, reasoning } => {
                    // Reset consecutive analysis counter since we're doing something useful
                    consecutive_analysis_count = 0;
                    
                    // Print the reasoning if provided
                    if let Some(reason) = &reasoning {
                        println!("🧠 AI reasoning: {}", reason);
                    }
                    
                    let result = self.execute_tool(tool.clone(), namespace, pod, service, lines).await;
                    self.current_tool_calls += 1;
                    self.add_tool_result(tool.clone(), result).await;
                }
                AIAgentAction::ProvideAnalysis { analysis } => {
                    consecutive_analysis_count += 1;
                    
                    if analysis.to_lowercase().contains("need more information") || 
                       analysis.to_lowercase().contains("could you") ||
                       analysis.to_lowercase().contains("can you provide") {
                        return Ok(AIAgentResult::PausedForUserInput {
                            reason: analysis,
                            tool_calls_used: self.current_tool_calls,
                        });
                    }
                    
                    // Check if the AI is indicating it has completed its analysis and has no more tools to run
                    let analysis_lower = analysis.to_lowercase();
                    let indicates_completion = analysis_lower.contains("no additional") ||
                        analysis_lower.contains("no further") ||
                        analysis_lower.contains("analysis complete") ||
                        analysis_lower.contains("diagnostic complete") ||
                        analysis_lower.contains("examination complete") ||
                        (analysis_lower.contains("no more") && analysis_lower.contains("check")) ||
                        (analysis_lower.contains("no more") && analysis_lower.contains("tool")) ||
                        (analysis_lower.contains("nothing more") && analysis_lower.contains("check")) ||
                        (analysis_lower.contains("nothing more") && analysis_lower.contains("tool"));
                    
                    // Safety check: if we've had too many consecutive analysis responses without tool calls
                    // AND the AI is not indicating active work, treat as completion
                    if consecutive_analysis_count >= max_consecutive_analysis && indicates_completion {
                        return Ok(AIAgentResult::Success {
                            final_analysis: analysis,
                            tool_calls_used: self.current_tool_calls,
                        });
                    }
                    
                    self.add_message(MessageRole::Assistant, analysis);
                }
                AIAgentAction::AskUser { question } => {
                    return Ok(AIAgentResult::PausedForUserInput {
                        reason: question,
                        tool_calls_used: self.current_tool_calls,
                    });
                }
            }

            if ai_response.to_lowercase().contains("COMPLETE:") {
                let final_analysis = ai_response.replace("COMPLETE:", "").trim().to_string();
                return Ok(AIAgentResult::Success {
                    final_analysis,
                    tool_calls_used: self.current_tool_calls,
                });
            }
        }
    }

    fn add_message(&mut self, role: MessageRole, content: String) {
        self.conversation_history.push(AIAgentMessage {
            role,
            content,
            tool_calls: Vec::new(),
            timestamp: std::time::SystemTime::now(),
        });
    }

    async fn add_tool_result(&mut self, tool: crate::cli::DebugTool, result: crate::tools::DebugToolResult) {
        let tool_call = AIToolCall {
            tool_name: format!("{:?}", tool),
            arguments: std::collections::HashMap::new(), // We could extract args from result.command
            result: Some(result.clone()),
        };

        let message_content = format!(
            "Tool: {:?}\nCommand: {}\nSuccess: {}\nOutput:\n{}{}",
            tool,
            result.command,
            result.success,
            result.output,
            if let Some(error) = &result.error {
                format!("\nError: {}", error)
            } else {
                String::new()
            }
        );

        self.conversation_history.push(AIAgentMessage {
            role: MessageRole::Tool,
            content: message_content,
            tool_calls: vec![tool_call],
            timestamp: std::time::SystemTime::now(),
        });
    }

    fn build_conversation_context(&self) -> String {
        let mut context = String::new();
        
        for message in &self.conversation_history {
            match message.role {
                MessageRole::System => {
                    context.push_str("SYSTEM: ");
                    context.push_str(&message.content);
                    context.push_str("\n\n");
                }
                MessageRole::User => {
                    context.push_str("USER: ");
                    context.push_str(&message.content);
                    context.push_str("\n\n");
                }
                MessageRole::Assistant => {
                    context.push_str("ASSISTANT: ");
                    context.push_str(&message.content);
                    context.push_str("\n\n");
                }
                MessageRole::Tool => {
                    context.push_str("TOOL_RESULT: ");
                    context.push_str(&message.content);
                    context.push_str("\n\n");
                }
            }
        }

        context.push_str(&format!(
            "Tool calls used: {}/{}\n\n",
            self.current_tool_calls, self.max_tool_calls
        ));

        context.push_str("What would you like to do next? You can:\n");
        context.push_str("- CALL_TOOL: <tool_name> [arguments] - to run a diagnostic tool\n");
        context.push_str("- ANALYZE: <analysis> - to provide analysis or ask for more information\n");
        context.push_str("- COMPLETE: <final_analysis> - to provide final solution\n");

        context
    }

    async fn parse_ai_action(&self, response: &str) -> crate::cli::AIAgentAction {
        // Parse the AI response to determine what action to take
        let response_lower = response.to_lowercase();
        
        // Look for tool calls with reasoning
        if response_lower.contains("call_tool") || response_lower.contains("run") || response_lower.contains("execute") {
            // Extract reasoning if present
            let reasoning = if response_lower.contains("reasoning:") {
                response.lines()
                    .find(|line| line.to_lowercase().contains("reasoning:"))
                    .map(|line| line.replace("REASONING:", "").replace("reasoning:", "").trim().to_string())
            } else {
                None
            };

            if let Some(tool_line) = response.lines().find(|line| {
                let line_lower = line.to_lowercase();
                line_lower.contains("call_tool") || 
                (line_lower.contains("run") && (line_lower.contains("kubectl") || line_lower.contains("journalctl") || line_lower.contains("systemctl")))
            }) {
                let tool_part = tool_line
                    .replace("CALL_TOOL:", "")
                    .replace("call_tool:", "")
                    .replace("run", "")
                    .trim()
                    .to_string();
                let parts: Vec<&str> = tool_part.split_whitespace().collect();
                
                if let Some(tool_name) = parts.first() {
                    // Map string to DebugTool enum
                    if let Some(tool) = self.string_to_debug_tool(tool_name) {
                        // Extract arguments - improved to handle positional arguments
                        let mut namespace = self.extract_arg(&parts, "--namespace");
                        let mut pod = self.extract_arg(&parts, "--pod");
                        let mut service = self.extract_arg(&parts, "--service");
                        let lines = self.extract_arg(&parts, "--lines").and_then(|s| s.parse().ok());
                        
                        // Handle positional arguments for specific tools
                        match tool {
                            crate::cli::DebugTool::KubectlDescribePod => {
                                // For kubectl_describe_pod, first non-flag argument is the pod name
                                if pod.is_none() && parts.len() > 1 {
                                    for i in 1..parts.len() {
                                        if !parts[i].starts_with('-') && !parts[i-1].starts_with('-') {
                                            pod = Some(parts[i].to_string());
                                            break;
                                        } else if i > 1 && parts[i-1] == "--namespace" {
                                            continue; // Skip namespace value
                                        } else if !parts[i].starts_with('-') {
                                            pod = Some(parts[i].to_string());
                                            break;
                                        }
                                    }
                                }
                            }
                            crate::cli::DebugTool::JournalctlService | crate::cli::DebugTool::SystemctlStatus => {
                                // For service tools, first non-flag argument is the service name
                                if service.is_none() && parts.len() > 1 {
                                    for i in 1..parts.len() {
                                        if !parts[i].starts_with('-') && !parts[i-1].starts_with('-') {
                                            service = Some(parts[i].to_string());
                                            break;
                                        } else if i > 1 && (parts[i-1] == "--namespace" || parts[i-1] == "--lines") {
                                            continue; // Skip flag values
                                        } else if !parts[i].starts_with('-') {
                                            service = Some(parts[i].to_string());
                                            break;
                                        }
                                    }
                                }
                            }
                            _ => {
                                // For other tools, extract any positional arguments as appropriate
                                // If no specific pod/service was found but there are non-flag args, use the first one
                                if pod.is_none() && service.is_none() && parts.len() > 1 {
                                    for i in 1..parts.len() {
                                        if !parts[i].starts_with('-') && !parts[i-1].starts_with('-') {
                                            // Determine if this tool typically uses pod or service
                                            if tool_name.contains("kubectl") && !tool_name.contains("service") {
                                                pod = Some(parts[i].to_string());
                                            } else if tool_name.contains("service") || tool_name.contains("systemctl") {
                                                service = Some(parts[i].to_string());
                                            }
                                            break;
                                        } else if i > 1 && (parts[i-1] == "--namespace" || parts[i-1] == "--lines") {
                                            continue; // Skip flag values
                                        } else if !parts[i].starts_with('-') {
                                            if tool_name.contains("kubectl") && !tool_name.contains("service") {
                                                pod = Some(parts[i].to_string());
                                            } else if tool_name.contains("service") || tool_name.contains("systemctl") {
                                                service = Some(parts[i].to_string());
                                            }
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        
                        return crate::cli::AIAgentAction::RunTool {
                            tool,
                            namespace,
                            pod,
                            service,
                            lines,
                            reasoning,
                        };
                    }
                }
            }
        }

        // Look for explicit completion format only - "COMPLETE:" at start of line
        if response.contains("COMPLETE:") || response_lower.lines().any(|line| line.trim().starts_with("complete:")) {
            let analysis = response.replace("COMPLETE:", "").replace("complete:", "").trim().to_string();
            return crate::cli::AIAgentAction::ProvideAnalysis { analysis };
        }

        // Look for analysis indicators
        if response_lower.contains("analyze:") || response_lower.contains("analysis") {
            let analysis = response.replace("ANALYZE:", "").replace("analyze:", "").trim().to_string();
            return crate::cli::AIAgentAction::ProvideAnalysis { analysis };
        }

        // If response seems to be asking for more information or is incomplete
        if response_lower.contains("need more") || 
           response_lower.contains("would need") ||
           response_lower.contains("could you provide") ||
           response_lower.contains("more information") ||
           response.len() < 30 {
            return crate::cli::AIAgentAction::AskUser { 
                question: response.to_string() 
            };
        }

        // Default: treat as a complete analysis if it's substantial
        if response.len() > 100 {
            crate::cli::AIAgentAction::ProvideAnalysis {
                analysis: response.to_string(),
            }
        } else {
            // Short responses are likely incomplete - ask for clarification
            crate::cli::AIAgentAction::AskUser {
                question: format!("The response was unclear: {}. Could you provide more detail?", response),
            }
        }
    }

    fn extract_arg(&self, parts: &[&str], arg_name: &str) -> Option<String> {
        for i in 0..parts.len() {
            if parts[i] == arg_name && i + 1 < parts.len() {
                return Some(parts[i + 1].to_string());
            }
        }
        None
    }

    fn string_to_debug_tool(&self, tool_name: &str) -> Option<crate::cli::DebugTool> {
        use crate::cli::DebugTool;
        
        match tool_name {
            "kubectl_get_pods" => Some(DebugTool::KubectlGetPods),
            "kubectl_describe_pod" => Some(DebugTool::KubectlDescribePod),
            "kubectl_get_services" => Some(DebugTool::KubectlGetServices),
            "kubectl_get_nodes" => Some(DebugTool::KubectlGetNodes),
            "kubectl_get_events" => Some(DebugTool::KubectlGetEvents),
            "journalctl_recent" => Some(DebugTool::JournalctlRecent),
            "journalctl_service" => Some(DebugTool::JournalctlService),
            "journalctl_boot" => Some(DebugTool::JournalctlBoot),
            "journalctl_errors" => Some(DebugTool::JournalctlErrors),
            "systemctl_status" => Some(DebugTool::SystemctlStatus),
            "ps_aux" => Some(DebugTool::PsAux),
            "netstat" => Some(DebugTool::Netstat),
            "df" => Some(DebugTool::Df),
            "free" => Some(DebugTool::Free),
            "systemctl_failed" => Some(DebugTool::SystemctlFailed),
            // Network diagnostic tools
            "ip_addr" => Some(DebugTool::IpAddr),
            "ip_route" => Some(DebugTool::IpRoute),
            "ss" => Some(DebugTool::Ss),
            "ping" => Some(DebugTool::Ping),
            "dig" => Some(DebugTool::Dig),
            "traceroute" => Some(DebugTool::Traceroute),
            "dns_config" => Some(DebugTool::DnsConfig),
            "dns_test" => Some(DebugTool::DnsTest),
            "connectivity_test" => Some(DebugTool::ConnectivityTest),
            "network_setup_check" => Some(DebugTool::NetworkSetupCheck),
            "arp_table" => Some(DebugTool::ArpTable),
            "iptables" => Some(DebugTool::Iptables),
            "ufw_status" => Some(DebugTool::UfwStatus),
            "networkmanager_status" => Some(DebugTool::NetworkManagerStatus),
            "wireless_info" => Some(DebugTool::WirelessInfo),
            "interface_stats" => Some(DebugTool::InterfaceStats),
            "network_health_check" => Some(DebugTool::NetworkHealthCheck),
            _ => None,
        }
    }

    async fn execute_tool(
        &self,
        tool: crate::cli::DebugTool,
        namespace: Option<String>,
        pod: Option<String>,
        service: Option<String>,
        lines: Option<usize>,
    ) -> crate::tools::DebugToolResult {
        use crate::cli::DebugTool;
        
        // Print what tool is being executed
        println!("🔧 AI is running tool: {:?}", tool);
        
        let result = match tool {
            DebugTool::KubectlGetPods => {
                self.debug_tools.run_kubectl_get_pods(namespace.as_deref()).await
            }
            DebugTool::KubectlDescribePod => {
                if let Some(pod_name) = pod {
                    self.debug_tools
                        .run_kubectl_describe_pod(&pod_name, namespace.as_deref())
                        .await
                } else {
                    crate::tools::DebugToolResult {
                        tool_name: "kubectl_describe_pod".to_string(),
                        command: "kubectl describe pod <missing-pod-name>".to_string(),
                        success: false,
                        output: "To describe a pod, you must first get the list of available pods.\n\nSUGGESTED NEXT STEPS:\n1. Run: kubectl_get_pods [--namespace <namespace>]\n2. Find the pod name you want to describe\n3. Run: kubectl_describe_pod <pod-name> [--namespace <namespace>]\n\nExample:\n- kubectl_get_pods --namespace kube-system\n- kubectl_describe_pod coredns-1234 --namespace kube-system".to_string(),
                        error: Some("Pod name required. Use kubectl_get_pods first to see available pods.".to_string()),
                        execution_time_ms: 0,
                    }
                }
            }
            DebugTool::KubectlGetServices => {
                self.debug_tools
                    .run_kubectl_get_services(namespace.as_deref())
                    .await
            }
            DebugTool::KubectlGetNodes => self.debug_tools.run_kubectl_get_nodes().await,
            DebugTool::KubectlGetEvents => {
                self.debug_tools
                    .run_kubectl_get_events(namespace.as_deref())
                    .await
            }
            DebugTool::JournalctlRecent => self.debug_tools.run_journalctl_recent(lines).await,
            DebugTool::JournalctlService => {
                if let Some(service_name) = service {
                    self.debug_tools
                        .run_journalctl_service(&service_name, lines)
                        .await
                } else {
                    crate::tools::DebugToolResult {
                        tool_name: "journalctl_service".to_string(),
                        command: "journalctl -u <missing-service-name>".to_string(),
                        success: false,
                        output: "To check service logs, you must specify a service name.\n\nCOMMON SERVICES:\n- systemd services: sshd, nginx, docker, NetworkManager\n- kubernetes: kubelet, kube-proxy\n\nSUGGESTED NEXT STEPS:\n1. Use: systemctl_failed to see failed services\n2. Or specify a known service: journalctl_service <service-name>\n\nExample:\n- journalctl_service docker\n- journalctl_service kubelet".to_string(),
                        error: Some("Service name required. Try: systemctl_failed to see available services.".to_string()),
                        execution_time_ms: 0,
                    }
                }
            }
            DebugTool::JournalctlBoot => self.debug_tools.run_journalctl_boot().await,
            DebugTool::JournalctlErrors => self.debug_tools.run_journalctl_errors(lines).await,
            DebugTool::SystemctlStatus => {
                if let Some(service_name) = service {
                    self.debug_tools.run_systemctl_status(&service_name).await
                } else {
                    crate::tools::DebugToolResult {
                        tool_name: "systemctl_status".to_string(),
                        command: "systemctl status <missing-service-name>".to_string(),
                        success: false,
                        output: "To check service status, you must specify a service name.\n\nCOMMON SERVICES:\n- systemd services: sshd, nginx, docker, NetworkManager\n- kubernetes: kubelet, kube-proxy\n\nSUGGESTED NEXT STEPS:\n1. Use: systemctl_failed to see failed services\n2. Or specify a known service: systemctl_status <service-name>\n\nExample:\n- systemctl_status docker\n- systemctl_status kubelet".to_string(),
                        error: Some("Service name required. Try: systemctl_failed to see available services.".to_string()),
                        execution_time_ms: 0,
                    }
                }
            }
            DebugTool::PsAux => self.debug_tools.run_ps_aux().await,
            DebugTool::Netstat => self.debug_tools.run_netstat().await,
            DebugTool::Df => self.debug_tools.run_df().await,
            DebugTool::Free => self.debug_tools.run_free().await,
            DebugTool::SystemctlFailed => self.debug_tools.run_systemctl_failed().await,
            // Network diagnostic tools
            DebugTool::IpAddr => self.debug_tools.run_ip_addr().await,
            DebugTool::IpRoute => self.debug_tools.run_ip_route().await,
            DebugTool::Ss => self.debug_tools.run_ss().await,
            DebugTool::Ping => {
                // Default ping to google.com if no specific host provided
                self.debug_tools.run_ping("8.8.8.8").await
            }
            DebugTool::Dig => {
                // Default dig lookup for google.com
                self.debug_tools.run_dig("google.com").await
            }
            DebugTool::Traceroute => {
                self.debug_tools.run_traceroute("8.8.8.8").await
            }
            DebugTool::DnsConfig => self.debug_tools.run_dns_config().await,
            DebugTool::DnsTest => self.debug_tools.run_dns_test("google.com").await,
            DebugTool::ConnectivityTest => self.debug_tools.run_connectivity_test().await,
            DebugTool::NetworkSetupCheck => self.debug_tools.run_network_setup_check().await,
            DebugTool::ArpTable => self.debug_tools.run_arp_table().await,
            DebugTool::Iptables => self.debug_tools.run_iptables().await,
            DebugTool::UfwStatus => self.debug_tools.run_ufw_status().await,
            DebugTool::NetworkManagerStatus => self.debug_tools.run_networkmanager_status().await,
            DebugTool::WirelessInfo => self.debug_tools.run_wireless_info().await,
            DebugTool::InterfaceStats => self.debug_tools.run_interface_stats().await,
            DebugTool::NetworkHealthCheck => {
                // For the comprehensive health check, run it and return combined results
                let results = self.debug_tools.run_network_health_check().await;
                
                // Show each individual command that was executed
                let combined_output = results.iter()
                    .map(|r| format!("=== {} ===\nCommand: {}\n{}", r.tool_name, r.command, r.output))
                    .collect::<Vec<_>>()
                    .join("\n\n");
                
                // List all the actual commands that were run
                let commands_run = results.iter()
                    .map(|r| r.command.clone())
                    .collect::<Vec<_>>()
                    .join("; ");
                    
                crate::tools::DebugToolResult {
                    tool_name: "network_health_check".to_string(),
                    command: commands_run,
                    success: results.iter().any(|r| r.success),
                    output: combined_output,
                    error: None,
                    execution_time_ms: results.iter().map(|r| r.execution_time_ms).sum(),
                }
            }
            // Add more tool implementations as needed
            _ => {
                crate::tools::DebugToolResult {
                    tool_name: format!("{:?}", tool),
                    command: format!("{:?} - not implemented", tool),
                    success: false,
                    output: String::new(),
                    error: Some("Tool not implemented in agent".to_string()),
                    execution_time_ms: 0,
                }
            }
        };
        
        // Print the actual command that was executed
        println!("💻 Command executed: {}", result.command);
        if result.success {
            println!("✅ Command completed successfully");
        } else {
            println!("❌ Command failed");
            if let Some(error) = &result.error {
                println!("   Error: {}", error);
            }
        }
        
        result
    }

    fn get_available_tools_description(&self) -> String {
        r#"
KUBERNETES TOOLS:
- kubectl_get_pods [--namespace <ns>]: List all pods in namespace
- kubectl_describe_pod <pod_name> [--namespace <ns>]: Get detailed pod information (REQUIRES pod name)
- kubectl_get_services [--namespace <ns>]: List all services in namespace
- kubectl_get_nodes: List all cluster nodes
- kubectl_get_events [--namespace <ns>]: Get recent cluster events

IMPORTANT: For kubectl_describe_pod, you MUST provide a pod name. First use kubectl_get_pods to see available pods, then describe specific ones.
Example: 
  1. CALL_TOOL: kubectl_get_pods --namespace kube-system
  2. CALL_TOOL: kubectl_describe_pod coredns-12345 --namespace kube-system

NETWORK DIAGNOSTIC TOOLS:
- ip_addr: Show network interfaces and IP addresses
- ip_route: Show routing table
- ss: Show socket statistics and listening ports
- ping: Test connectivity to 8.8.8.8 (Google DNS)
- dig: Perform DNS lookup for google.com
- traceroute: Trace network route to 8.8.8.8
- dns_config: Show DNS configuration (/etc/resolv.conf)
- dns_test: Test DNS resolution with multiple servers
- connectivity_test: Test connectivity to multiple hosts
- network_setup_check: Quick network setup check for standard users
- network_health_check: Comprehensive network health check (runs multiple tools)
- arp_table: Show ARP table
- iptables: Show firewall rules
- ufw_status: Check UFW firewall status
- networkmanager_status: Check NetworkManager status
- wireless_info: Show wireless interface information
- interface_stats: Show network interface statistics

SYSTEM LOGS:
- journalctl_recent [--lines <n>]: Get recent system logs (default 50 lines)
- journalctl_service <service_name> [--lines <n>]: Get logs for specific service (REQUIRES service name)
- journalctl_boot: Get boot logs
- journalctl_errors [--lines <n>]: Get error logs only

SYSTEM SERVICES:
- systemctl_status <service_name>: Get status of specific service (REQUIRES service name)
- systemctl_failed: Show failed systemd units (use this first to find service names)

IMPORTANT: For service-specific tools, use systemctl_failed first to see available service names.
Example workflow:
  1. CALL_TOOL: systemctl_failed
  2. CALL_TOOL: systemctl_status docker
  3. CALL_TOOL: journalctl_service docker --lines 50

PROCESS & PERFORMANCE:
- ps_aux: List all running processes
- free: Show memory usage
- df: Show disk usage
- netstat: Show network connections (legacy)
        "#.to_string()
    }

    /// Get a summary of the conversation for debugging
    pub fn get_conversation_summary(&self) -> String {
        format!(
            "Conversation with {} messages, {} tool calls used of {} limit",
            self.conversation_history.len(),
            self.current_tool_calls,
            self.max_tool_calls
        )
    }

    /// Get the full conversation history
    pub fn get_conversation_history(&self) -> &[AIAgentMessage] {
        &self.conversation_history
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::DebugTool;

    #[tokio::test]
    async fn test_ai_agent_creation() {
        let dummy_ai = Box::new(DummyAI);
        let config = AIAgentConfig::default();
        
        let agent = AIAgent::new(dummy_ai, config).await;
        
        assert_eq!(agent.max_tool_calls, 50);
        assert_eq!(agent.current_tool_calls, 0);
        assert_eq!(agent.conversation_history.len(), 0);
    }

    #[tokio::test]
    async fn test_ai_agent_config_customization() {
        let dummy_ai = Box::new(DummyAI);
        let config = AIAgentConfig {
            max_tool_calls: 100,
            pause_on_limit: false,
            allow_user_continuation: false,
            verbose_logging: true,
        };
        
        let agent = AIAgent::new(dummy_ai, config).await;
        
        assert_eq!(agent.max_tool_calls, 100);
    }

    #[tokio::test]
    async fn test_ai_agent_tool_mapping() {
        let dummy_ai = Box::new(DummyAI);
        let config = AIAgentConfig::default();
        let agent = AIAgent::new(dummy_ai, config).await;
        
        // Test tool string to enum mapping
        assert!(matches!(
            agent.string_to_debug_tool("kubectl_get_pods"),
            Some(DebugTool::KubectlGetPods)
        ));
        
        assert!(matches!(
            agent.string_to_debug_tool("systemctl_status"),
            Some(DebugTool::SystemctlStatus)
        ));
        
        assert!(agent.string_to_debug_tool("nonexistent_tool").is_none());
    }

    #[tokio::test]
    async fn test_ai_agent_argument_extraction() {
        let dummy_ai = Box::new(DummyAI);
        let config = AIAgentConfig::default();
        let agent = AIAgent::new(dummy_ai, config).await;
        
        let parts = vec!["kubectl_get_pods", "--namespace", "default", "--lines", "20"];
        
        assert_eq!(
            agent.extract_arg(&parts, "--namespace"),
            Some("default".to_string())
        );
        assert_eq!(
            agent.extract_arg(&parts, "--lines"), 
            Some("20".to_string())
        );
        assert_eq!(agent.extract_arg(&parts, "--missing"), None);
    }

    #[tokio::test]
    async fn test_conversation_tracking() {
        let dummy_ai = Box::new(DummyAI);
        let config = AIAgentConfig::default();
        let mut agent = AIAgent::new(dummy_ai, config).await;
        
        agent.add_message(MessageRole::User, "Test message".to_string());
        agent.add_message(MessageRole::Assistant, "Test response".to_string());
        
        assert_eq!(agent.conversation_history.len(), 2);
        assert!(matches!(agent.conversation_history[0].role, MessageRole::User));
        assert!(matches!(agent.conversation_history[1].role, MessageRole::Assistant));
        
        let summary = agent.get_conversation_summary();
        assert!(summary.contains("2 messages"));
        assert!(summary.contains("0 tool calls"));
    }

    #[test]
    fn test_ai_agent_result_display() {
        let success_result = AIAgentResult::Success {
            final_analysis: "Analysis complete".to_string(),
            tool_calls_used: 5,
        };
        
        match success_result {
            AIAgentResult::Success { final_analysis, tool_calls_used } => {
                assert_eq!(final_analysis, "Analysis complete");
                assert_eq!(tool_calls_used, 5);
            }
            _ => panic!("Expected Success result"),
        }
        
        let limit_result = AIAgentResult::LimitReached {
            partial_analysis: "Partial analysis".to_string(),
            tool_calls_used: 50,
        };
        
        match limit_result {
            AIAgentResult::LimitReached { partial_analysis, tool_calls_used } => {
                assert_eq!(partial_analysis, "Partial analysis");
                assert_eq!(tool_calls_used, 50);
            }
            _ => panic!("Expected LimitReached result"),
        }
    }
}
