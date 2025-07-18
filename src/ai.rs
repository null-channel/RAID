use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::cli::AIProvider as CliAIProvider;
use crate::known_issues::{KnownIssuesDatabase, IssueCategory};

#[async_trait]
pub trait AIProvider: Send + Sync {
    async fn analyze(&self, input: &str) -> Result<String, AIError>;
    async fn analyze_with_known_issues(&self, input: &str, category: Option<IssueCategory>) -> Result<String, AIError>;
    /// Answer a user question about their system or issue
    async fn answer_question(&self, question: &str, system_context: &str) -> Result<String, AIError>;
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
            _ => return Err(AIError::ConfigError(format!("Unknown provider: {}", provider))),
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

    pub async fn from_cli(cli_provider: &CliAIProvider, api_key: Option<String>, model: Option<String>, base_url: Option<String>, max_tokens: Option<u32>, temperature: Option<f32>) -> Result<Self, AIError> {
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

    async fn analyze_with_known_issues(&self, input: &str, category: Option<IssueCategory>) -> Result<String, AIError> {
        // Get relevant known issues for this context
        let relevant_issues = self.known_issues.get_relevant_issues_for_context(input, category).await;
        
        // Build enhanced prompt with known issues
        let mut enhanced_input = input.to_string();
        if !relevant_issues.is_empty() {
            enhanced_input.push_str("\n\nKNOWN ISSUES THAT MAY BE RELEVANT:\n");
            for issue in relevant_issues {
                enhanced_input.push_str(&format!("- {}: {}\n", issue.title, issue.description));
            }
            enhanced_input.push_str("\nConsider these known issues when analyzing the system state.\n");
        }

        match self.config.provider {
            AIProviderType::OpenAI => self.analyze_openai(&enhanced_input).await,
            AIProviderType::Anthropic => self.analyze_anthropic(&enhanced_input).await,
            AIProviderType::Local => self.analyze_local(&enhanced_input).await,
        }
    }

    async fn answer_question(&self, question: &str, system_context: &str) -> Result<String, AIError> {
        // Get relevant known issues for this context
        let relevant_issues = self.known_issues.get_relevant_issues_for_context(question, None).await;
        
        // Build context with known issues
        let mut enhanced_context = system_context.to_string();
        if !relevant_issues.is_empty() {
            enhanced_context.push_str("\n\nRELEVANT KNOWN ISSUES:\n");
            for issue in relevant_issues {
                enhanced_context.push_str(&format!("- {}: {}\n", issue.title, issue.description));
            }
        }

        match self.config.provider {
            AIProviderType::OpenAI => self.answer_question_openai(question, &enhanced_context).await,
            AIProviderType::Anthropic => self.answer_question_anthropic(question, &enhanced_context).await,
            AIProviderType::Local => self.answer_question_local(question, &enhanced_context).await,
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
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| AIError::ConfigError("OpenAI API key not found".to_string()))?;

        let base_url = self.config.base_url.as_deref()
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

        let response = self.client
            .post(&format!("{}/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!("OpenAI API error: {}", error_text)));
        }

        let response_json: serde_json::Value = response.json().await?;
        
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AIError::APIError("Invalid response format".to_string()))?;

        Ok(content.to_string())
    }

    async fn analyze_anthropic(&self, input: &str) -> Result<String, AIError> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| AIError::ConfigError("Anthropic API key not found".to_string()))?;

        let base_url = self.config.base_url.as_deref()
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

        let response = self.client
            .post(&format!("{}/messages", base_url))
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!("Anthropic API error: {}", error_text)));
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
        
        let base_url = self.config.base_url.as_deref()
            .unwrap_or("http://localhost:11434");

        // Try Ollama first
        if let Ok(response) = self.try_ollama(base_url, input).await {
            return Ok(response);
        }

        // Fallback to a simple local analysis
        Ok(format!("[Local AI] Analysis of system information: {}. This is a placeholder response. To use a real local model, configure Ollama or another local model server.", input))
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

        let response = self.client
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

    async fn answer_question_openai(&self, question: &str, system_context: &str) -> Result<String, AIError> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| AIError::ConfigError("OpenAI API key not found".to_string()))?;

        let base_url = self.config.base_url.as_deref()
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

        let response = self.client
            .post(&format!("{}/chat/completions", base_url))
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!("OpenAI API error: {}", error_text)));
        }

        let response_json: serde_json::Value = response.json().await?;
        
        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AIError::APIError("Invalid response format".to_string()))?;

        Ok(content.to_string())
    }

    async fn answer_question_anthropic(&self, question: &str, system_context: &str) -> Result<String, AIError> {
        let api_key = self.config.api_key.as_ref()
            .ok_or_else(|| AIError::ConfigError("Anthropic API key not found".to_string()))?;

        let base_url = self.config.base_url.as_deref()
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

        let response = self.client
            .post(&format!("{}/messages", base_url))
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            return Err(AIError::APIError(format!("Anthropic API error: {}", error_text)));
        }

        let response_json: serde_json::Value = response.json().await?;
        
        let content = response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| AIError::APIError("Invalid response format".to_string()))?;

        Ok(content.to_string())
    }

    async fn answer_question_local(&self, question: &str, system_context: &str) -> Result<String, AIError> {
        let base_url = self.config.base_url.as_deref()
            .unwrap_or("http://localhost:11434");

        // Try Ollama first
        if let Ok(response) = self.try_ollama_question(base_url, question, system_context).await {
            return Ok(response);
        }

        // Fallback response
        Ok(format!("[Local AI] Question: {}. Context available but using placeholder response. To use a real local model, configure Ollama or another local model server.", question))
    }

    async fn try_ollama_question(&self, base_url: &str, question: &str, system_context: &str) -> Result<String, AIError> {
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

        let response = self.client
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

    async fn analyze_with_known_issues(&self, _input: &str, _category: Option<IssueCategory>) -> Result<String, AIError> {
        Ok("System appears healthy. Any ACPI/BIOS errors shown above are often normal on Linux systems and can be ignored unless you're experiencing specific hardware problems.".to_string())
    }

    async fn answer_question(&self, _question: &str, _system_context: &str) -> Result<String, AIError> {
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
pub async fn create_ai_provider_from_cli(cli_provider: &CliAIProvider, api_key: Option<String>, model: Option<String>, base_url: Option<String>, max_tokens: Option<u32>, temperature: Option<f32>) -> Result<Box<dyn AIProvider>, AIError> {
    if let Ok(client) = AIClient::from_cli(cli_provider, api_key, model, base_url, max_tokens, temperature).await {
        return Ok(Box::new(client));
    }

    // Fallback to dummy AI
    Ok(Box::new(DummyAI))
} 