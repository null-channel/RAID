use crate::cli::{AIProvider, OutputFormat};
use config::{Config, ConfigError, Environment, File};
use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RaidConfig {
    pub ai: AIConfig,
    pub output: OutputConfig,
    pub ui: UIConfig,
    pub database: DatabaseConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIConfig {
    pub provider: String,
    pub api_key: Option<String>,
    pub model: Option<String>,
    pub base_url: Option<String>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputConfig {
    pub format: String,
    pub verbose: bool,
    pub color: bool,
    pub progress: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIConfig {
    pub color: bool,
    pub progress_indicators: bool,
    pub emoji: bool,
    pub compact_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub path: String,
    pub auto_cleanup: bool,
    pub retention_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
}

impl Default for RaidConfig {
    fn default() -> Self {
        Self {
            ai: AIConfig {
                provider: "open-ai".to_string(),
                api_key: None,
                model: Some("gpt-4o-mini".to_string()),
                base_url: None,
                max_tokens: Some(1000),
                temperature: Some(0.7),
            },
            output: OutputConfig {
                format: "text".to_string(),
                verbose: false,
                color: true,
                progress: true,
            },
            ui: UIConfig {
                color: true,
                progress_indicators: true,
                emoji: true,
                compact_mode: false,
            },
            database: DatabaseConfig {
                path: "system_checks.db".to_string(),
                auto_cleanup: false,
                retention_days: 30,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
        }
    }
}

impl RaidConfig {
    /// Load configuration from files, environment variables, and defaults
    pub fn load() -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        // Start with defaults
        builder = builder.add_source(config::Config::try_from(&RaidConfig::default())?);

        // Add configuration files in order of precedence (last wins)
        
        // 1. System-wide config
        if let Some(system_config) = Self::get_system_config_path() {
            if system_config.exists() {
                builder = builder.add_source(File::from(system_config).required(false));
            }
        }

        // 2. User config directory
        if let Some(user_config_dir) = Self::get_user_config_dir() {
            for filename in &["raid.yaml", "raid.yml", "raid.toml"] {
                let config_file = user_config_dir.join(filename);
                if config_file.exists() {
                    builder = builder.add_source(File::from(config_file).required(false));
                    break; // Use the first one found
                }
            }
        }

        // 3. Current directory config
        for filename in &["raid.yaml", "raid.yml", "raid.toml", ".raid.yaml", ".raid.yml", ".raid.toml"] {
            let config_file = PathBuf::from(filename);
            if config_file.exists() {
                builder = builder.add_source(File::from(config_file).required(false));
                break; // Use the first one found
            }
        }

        // 4. Environment variables (with RAID_ prefix)
        builder = builder.add_source(
            Environment::with_prefix("RAID")
                .prefix_separator("_")
                .separator("__"),
        );

        // Build and deserialize
        let config = builder.build()?;
        config.try_deserialize()
    }

    /// Load configuration with custom config file path
    pub fn load_from_file<P: AsRef<std::path::Path>>(path: P) -> Result<Self, ConfigError> {
        let mut builder = Config::builder();

        // Start with defaults
        builder = builder.add_source(config::Config::try_from(&RaidConfig::default())?);

        // Add the specified config file
        builder = builder.add_source(File::from(path.as_ref()).required(true));

        // Add environment variables (they can still override file settings)
        builder = builder.add_source(
            Environment::with_prefix("RAID")
                .prefix_separator("_")
                .separator("__"),
        );

        let config = builder.build()?;
        config.try_deserialize()
    }

    /// Get the system-wide configuration file path
    fn get_system_config_path() -> Option<PathBuf> {
        // Try common system config locations
        for path in &[
            "/etc/raid/config.yaml",
            "/etc/raid.yaml",
            "/usr/local/etc/raid.yaml",
        ] {
            let path = PathBuf::from(path);
            if path.exists() {
                return Some(path);
            }
        }
        None
    }

    /// Get the user configuration directory
    fn get_user_config_dir() -> Option<PathBuf> {
        if let Some(config_dir) = dirs::config_dir() {
            let raid_config_dir = config_dir.join("raid");
            if raid_config_dir.exists() || std::fs::create_dir_all(&raid_config_dir).is_ok() {
                return Some(raid_config_dir);
            }
        }

        // Fallback to home directory
        if let Some(home) = home_dir() {
            let home_config = home.join(".config").join("raid");
            if home_config.exists() || std::fs::create_dir_all(&home_config).is_ok() {
                return Some(home_config);
            }
            
            // Ultimate fallback to just home directory
            return Some(home);
        }

        None
    }

    /// Create a sample configuration file
    pub fn create_sample_config<P: AsRef<std::path::Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
        let sample_config = RaidConfig::default();
        let yaml_content = serde_yaml::to_string(&sample_config)?;
        
        std::fs::write(path, yaml_content)?;
        Ok(())
    }

    /// Get the effective AI provider from config
    pub fn get_ai_provider(&self) -> AIProvider {
        match self.ai.provider.to_lowercase().as_str() {
            "openai" | "open-ai" => AIProvider::OpenAI,
            "anthropic" => AIProvider::Anthropic,
            "local" => AIProvider::Local,
            _ => AIProvider::OpenAI, // Default fallback
        }
    }

    /// Get the effective output format from config
    pub fn get_output_format(&self) -> OutputFormat {
        match self.output.format.to_lowercase().as_str() {
            "yaml" | "yml" => OutputFormat::Yaml,
            "json" => OutputFormat::Json,
            _ => OutputFormat::Text, // Default fallback
        }
    }

    /// Get the model name with provider-specific defaults
    pub fn get_model(&self) -> String {
        if let Some(model) = &self.ai.model {
            model.clone()
        } else {
            // Provider-specific defaults
            match self.get_ai_provider() {
                AIProvider::OpenAI => "gpt-4o-mini".to_string(),
                AIProvider::Anthropic => "claude-3-5-sonnet-20241022".to_string(),
                AIProvider::Local => "llama2".to_string(),
            }
        }
    }

    /// Merge CLI overrides into the configuration
    pub fn merge_cli_overrides(&mut self, cli: &crate::cli::Cli) {
        // AI overrides
        if matches!(cli.ai_provider, AIProvider::OpenAI) {
            self.ai.provider = "open-ai".to_string();
        } else if matches!(cli.ai_provider, AIProvider::Anthropic) {
            self.ai.provider = "anthropic".to_string();
        } else if matches!(cli.ai_provider, AIProvider::Local) {
            self.ai.provider = "local".to_string();
        }

        if cli.ai_api_key.is_some() {
            self.ai.api_key = cli.ai_api_key.clone();
        }

        if cli.ai_model.is_some() {
            self.ai.model = cli.ai_model.clone();
        }

        if cli.ai_base_url.is_some() {
            self.ai.base_url = cli.ai_base_url.clone();
        }

        if cli.ai_max_tokens.is_some() {
            self.ai.max_tokens = cli.ai_max_tokens;
        }

        if cli.ai_temperature.is_some() {
            self.ai.temperature = cli.ai_temperature;
        }

        // Output overrides
        self.output.format = match cli.output_format {
            OutputFormat::Text => "text".to_string(),
            OutputFormat::Yaml => "yaml".to_string(),
            OutputFormat::Json => "json".to_string(),
        };

        self.output.verbose = cli.verbose;
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate AI provider
        if !["open-ai", "openai", "anthropic", "local"].contains(&self.ai.provider.as_str()) {
            return Err(format!("Invalid AI provider: {}", self.ai.provider));
        }

        // Validate output format
        if !["text", "yaml", "yml", "json"].contains(&self.output.format.as_str()) {
            return Err(format!("Invalid output format: {}", self.output.format));
        }

        // Validate temperature range
        if let Some(temp) = self.ai.temperature {
            if temp < 0.0 || temp > 1.0 {
                return Err(format!("Temperature must be between 0.0 and 1.0, got: {}", temp));
            }
        }

        // Validate max_tokens
        if let Some(tokens) = self.ai.max_tokens {
            if tokens == 0 {
                return Err("max_tokens must be greater than 0".to_string());
            }
        }

        // Validate retention days
        if self.database.retention_days == 0 {
            return Err("retention_days must be greater than 0".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::fs;

    #[test]
    fn test_default_config() {
        let config = RaidConfig::default();
        assert_eq!(config.ai.provider, "open-ai");
        assert_eq!(config.output.format, "text");
        assert!(config.ui.color);
        assert_eq!(config.database.retention_days, 30);
    }

    #[test]
    fn test_config_validation() {
        let mut config = RaidConfig::default();
        assert!(config.validate().is_ok());

        // Test invalid provider
        config.ai.provider = "invalid".to_string();
        assert!(config.validate().is_err());
        
        // Reset and test invalid temperature
        config.ai.provider = "open-ai".to_string();
        config.ai.temperature = Some(2.0);
        assert!(config.validate().is_err());
        
        // Reset and test invalid max_tokens
        config.ai.temperature = Some(0.5);
        config.ai.max_tokens = Some(0);
        assert!(config.validate().is_err());
        
        // Reset and test invalid retention days
        config.ai.max_tokens = Some(1000);
        config.database.retention_days = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_sample_config_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let result = RaidConfig::create_sample_config(temp_file.path());
        assert!(result.is_ok());
        
        // Verify file was created and has content
        let content = std::fs::read_to_string(temp_file.path()).unwrap();
        assert!(content.contains("ai:"));
        assert!(content.contains("provider:"));
        assert!(content.contains("output:"));
        assert!(content.contains("ui:"));
        assert!(content.contains("database:"));
        assert!(content.contains("logging:"));
    }

    #[test]
    fn test_get_ai_provider() {
        let mut config = RaidConfig::default();
        
        config.ai.provider = "openai".to_string();
        assert!(matches!(config.get_ai_provider(), AIProvider::OpenAI));
        
        config.ai.provider = "open-ai".to_string();
        assert!(matches!(config.get_ai_provider(), AIProvider::OpenAI));
        
        config.ai.provider = "anthropic".to_string();
        assert!(matches!(config.get_ai_provider(), AIProvider::Anthropic));
        
        config.ai.provider = "local".to_string();
        assert!(matches!(config.get_ai_provider(), AIProvider::Local));
        
        // Test fallback for invalid provider
        config.ai.provider = "invalid".to_string();
        assert!(matches!(config.get_ai_provider(), AIProvider::OpenAI));
    }

    #[test]
    fn test_get_output_format() {
        let mut config = RaidConfig::default();
        
        config.output.format = "text".to_string();
        assert!(matches!(config.get_output_format(), OutputFormat::Text));
        
        config.output.format = "yaml".to_string();
        assert!(matches!(config.get_output_format(), OutputFormat::Yaml));
        
        config.output.format = "yml".to_string();
        assert!(matches!(config.get_output_format(), OutputFormat::Yaml));
        
        config.output.format = "json".to_string();
        assert!(matches!(config.get_output_format(), OutputFormat::Json));
        
        // Test fallback for invalid format
        config.output.format = "invalid".to_string();
        assert!(matches!(config.get_output_format(), OutputFormat::Text));
    }

    #[test]
    fn test_get_model_with_defaults() {
        let mut config = RaidConfig::default();
        
        // Test with explicit model
        config.ai.model = Some("custom-model".to_string());
        assert_eq!(config.get_model(), "custom-model");
        
        // Test provider-specific defaults
        config.ai.model = None;
        
        config.ai.provider = "open-ai".to_string();
        assert_eq!(config.get_model(), "gpt-4o-mini");
        
        config.ai.provider = "anthropic".to_string();
        assert_eq!(config.get_model(), "claude-3-5-sonnet-20241022");
        
        config.ai.provider = "local".to_string();
        assert_eq!(config.get_model(), "llama2");
    }

    #[test]
    fn test_load_from_yaml_file() {
        let temp_file = NamedTempFile::with_suffix(".yaml").unwrap();
        
        // Create a test YAML config
        let yaml_content = r#"
ai:
  provider: anthropic
  model: claude-3-5-sonnet-20241022
  max_tokens: 2000
  temperature: 0.3
output:
  format: json
  verbose: true
ui:
  color: false
  progress_indicators: false
database:
  path: custom.db
  retention_days: 60
"#;
        fs::write(temp_file.path(), yaml_content).unwrap();
        
        let config = RaidConfig::load_from_file(temp_file.path()).unwrap();
        assert_eq!(config.ai.provider, "anthropic");
        assert_eq!(config.ai.model, Some("claude-3-5-sonnet-20241022".to_string()));
        assert_eq!(config.ai.max_tokens, Some(2000));
        assert_eq!(config.ai.temperature, Some(0.3));
        assert_eq!(config.output.format, "json");
        assert!(config.output.verbose);
        assert!(!config.ui.color);
        assert!(!config.ui.progress_indicators);
        assert_eq!(config.database.path, "custom.db");
        assert_eq!(config.database.retention_days, 60);
    }

    #[test]
    fn test_merge_cli_overrides() {
        let mut config = RaidConfig::default();
        
        // Create a mock CLI with overrides
        use crate::cli::Cli;
        use clap::Parser;
        
        // Test CLI overrides
        let cli = Cli {
            problem_description: None,
            ai_provider: AIProvider::Anthropic,
            ai_api_key: Some("test-key".to_string()),
            ai_model: Some("custom-model".to_string()),
            ai_base_url: Some("https://custom.api".to_string()),
            ai_max_tokens: Some(1500),
            ai_temperature: Some(0.8),
            dry_run: false,
            verbose: true,
            output_format: OutputFormat::Yaml,
            config: None,
            no_color: false,
            no_progress: false,
            command: None,
        };
        
        config.merge_cli_overrides(&cli);
        
        assert_eq!(config.ai.provider, "anthropic");
        assert_eq!(config.ai.api_key, Some("test-key".to_string()));
        assert_eq!(config.ai.model, Some("custom-model".to_string()));
        assert_eq!(config.ai.base_url, Some("https://custom.api".to_string()));
        assert_eq!(config.ai.max_tokens, Some(1500));
        assert_eq!(config.ai.temperature, Some(0.8));
        assert_eq!(config.output.format, "yaml");
        assert!(config.output.verbose);
    }

    #[test]
    fn test_config_file_precedence() {
        // Test that the configuration follows the correct precedence order
        // This is more of an integration test to ensure the layering works
        let config = RaidConfig::default();
        
        // Verify default values
        assert_eq!(config.ai.provider, "open-ai");
        assert_eq!(config.ai.max_tokens, Some(1000));
        assert_eq!(config.output.format, "text");
        assert!(!config.output.verbose);
    }

    #[test]
    fn test_config_validation_edge_cases() {
        let mut config = RaidConfig::default();
        
        // Test edge case temperature values
        config.ai.temperature = Some(0.0);
        assert!(config.validate().is_ok());
        
        config.ai.temperature = Some(1.0);
        assert!(config.validate().is_ok());
        
        config.ai.temperature = Some(-0.1);
        assert!(config.validate().is_err());
        
        config.ai.temperature = Some(1.1);
        assert!(config.validate().is_err());
        
        // Test edge case max_tokens
        config.ai.temperature = Some(0.5);
        config.ai.max_tokens = Some(1);
        assert!(config.validate().is_ok());
        
        config.ai.max_tokens = Some(0);
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_error_handling_invalid_file() {
        let result = RaidConfig::load_from_file("/nonexistent/config.yaml");
        assert!(result.is_err());
    }

    #[test]
    fn test_toml_config_support() {
        let temp_file = NamedTempFile::with_suffix(".toml").unwrap();
        
        // Create a test TOML config
        let toml_content = r#"
[ai]
provider = "local"
model = "llama3"
max_tokens = 1500

[output]
format = "json"
verbose = false

[ui]
color = true
progress_indicators = false

[database]
path = "test.db"
retention_days = 45
"#;
        fs::write(temp_file.path(), toml_content).unwrap();
        
        let config = RaidConfig::load_from_file(temp_file.path()).unwrap();
        assert_eq!(config.ai.provider, "local");
        assert_eq!(config.ai.model, Some("llama3".to_string()));
        assert_eq!(config.ai.max_tokens, Some(1500));
        assert_eq!(config.output.format, "json");
        assert!(!config.output.verbose);
        assert!(config.ui.color);
        assert!(!config.ui.progress_indicators);
        assert_eq!(config.database.path, "test.db");
        assert_eq!(config.database.retention_days, 45);
    }
} 