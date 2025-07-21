use crate::cli::ConfigAction;
use crate::config::RaidConfig;

pub async fn run_config_command(
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