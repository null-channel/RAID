use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(
    name = "raid",
    version = "1.0",
    author = "Your Name <you@example.com>",
    about = "RAID - Rust Analysis and Informative Debugger",
    long_about = "RAID (Rust Analysis and Informative Debugger) - A comprehensive system health checker that leverages AI to analyze your system for potential issues, security concerns, and performance problems."
)]
pub struct Cli {
    /// Problem description to analyze (e.g., 'my pod is stuck in crash loop backoff')
    #[arg(value_name = "PROBLEM")]
    pub problem_description: Option<String>,

    /// AI provider to use
    #[arg(long, short = 'p', value_enum, env = "AI_PROVIDER", default_value = "open-ai")]
    pub ai_provider: AIProvider,

    /// API key for the AI provider
    #[arg(long, short = 'k', env = "AI_API_KEY")]
    pub ai_api_key: Option<String>,

    /// AI model to use
    #[arg(long, short = 'm', env = "AI_MODEL")]
    pub ai_model: Option<String>,

    /// Base URL for AI provider (for custom endpoints)
    #[arg(long, env = "AI_BASE_URL")]
    pub ai_base_url: Option<String>,

    /// Maximum tokens for AI response
    #[arg(long, env = "AI_MAX_TOKENS")]
    pub ai_max_tokens: Option<u32>,

    /// Temperature for AI response (0.0-1.0)
    #[arg(long, env = "AI_TEMPERATURE")]
    pub ai_temperature: Option<f32>,

    /// Run without AI analysis (just collect and display system info)
    #[arg(long)]
    pub dry_run: bool,

    /// Enable verbose output
    #[arg(long, short = 'v', default_value = "false")]
    pub verbose: bool,

    /// Output format (text, yaml, json)
    #[arg(long, short = 'o', value_enum, default_value = "text")]
    pub output_format: OutputFormat,

    /// Subcommand to execute
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Check specific system components
    Check {
        /// Component to check
        #[arg(value_enum)]
        component: CheckComponent,
    },
    /// Run debugging tools
    Debug {
        /// Debug tool to run
        #[arg(value_enum)]
        tool: DebugTool,
        /// Namespace for Kubernetes commands
        #[arg(long, short = 'n')]
        namespace: Option<String>,
        /// Pod name for describe commands
        #[arg(long, short = 'p')]
        pod: Option<String>,
        /// Service name for service-specific commands
        #[arg(long, short = 's')]
        service: Option<String>,
        /// Number of lines to show (for journalctl)
        #[arg(long, short = 'l')]
        lines: Option<usize>,
    },
    /// Manage known issues database
    Issues {
        /// Action to perform on known issues
        #[arg(value_enum)]
        action: IssueAction,
        /// Issue ID (for get, update, delete actions)
        #[arg(long, short = 'i')]
        issue_id: Option<String>,
        /// Search query (for search action)
        #[arg(long, short = 'q')]
        query: Option<String>,
    },
}

#[derive(ValueEnum, Debug, Clone)]
pub enum OutputFormat {
    Text,
    Yaml,
    Json,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum AIProvider {
    OpenAI,
    Anthropic,
    Local,
}

impl AIProvider {
    pub fn as_str(&self) -> &'static str {
        match self {
            AIProvider::OpenAI => "openai",
            AIProvider::Anthropic => "anthropic",
            AIProvider::Local => "local",
        }
    }
}

#[derive(ValueEnum, Debug, Clone)]
pub enum CheckComponent {
    All,
    System,
    Containers,
    Kubernetes,
    Cgroups,
    Systemd,
    Journal,
    Debug,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum IssueAction {
    /// List all known issues
    List,
    /// Get a specific issue by ID
    Get,
    /// Search issues by query
    Search,
    /// Add a new issue
    Add,
    /// Update an existing issue
    Update,
    /// Delete an issue
    Delete,
}

#[derive(ValueEnum, Debug, Clone)]
pub enum DebugTool {
    /// Get Kubernetes pods
    KubectlGetPods,
    /// Describe a Kubernetes pod
    KubectlDescribePod,
    /// Get Kubernetes services
    KubectlGetServices,
    /// Get Kubernetes nodes
    KubectlGetNodes,
    /// Get Kubernetes events
    KubectlGetEvents,
    /// Get recent journal logs
    JournalctlRecent,
    /// Get logs for a specific service
    JournalctlService,
    /// Get boot logs
    JournalctlBoot,
    /// Get error logs
    JournalctlErrors,
    /// Get systemctl status for a service
    SystemctlStatus,
    /// Get process list
    PsAux,
    /// Get network connections
    Netstat,
    /// Get disk usage
    Df,
    /// Get memory usage
    Free,
    /// Get cgroups information from /proc/cgroups
    CatProcCgroups,
    /// List cgroup filesystem
    LsCgroup,
    /// Get current process cgroup information
    CatProcSelfCgroup,
    /// Get mount information for current process
    CatProcSelfMountinfo,
    /// List all namespaces
    Lsns,
    /// Get current process status information
    CatProcSelfStatus,
    /// List namespace files for current process
    CatProcSelfNs,
}

#[derive(Debug, Clone)]
pub enum AIAgentAction {
    /// Run a debug tool
    RunTool {
        tool: DebugTool,
        namespace: Option<String>,
        pod: Option<String>,
        service: Option<String>,
        lines: Option<usize>,
    },
    /// Provide final analysis/answer
    ProvideAnalysis {
        analysis: String,
    },
    /// Ask user for more information
    AskUser {
        question: String,
    },
}

impl CheckComponent {
    pub fn as_str(&self) -> &'static str {
        match self {
            CheckComponent::All => "all",
            CheckComponent::System => "system",
            CheckComponent::Containers => "containers",
            CheckComponent::Kubernetes => "kubernetes",
            CheckComponent::Cgroups => "cgroups",
            CheckComponent::Systemd => "systemd",
            CheckComponent::Journal => "journal",
            CheckComponent::Debug => "debug",
        }
    }
}

impl Cli {
    /// Get the default model for the selected AI provider
    pub fn get_default_model(&self) -> String {
        match self.ai_provider {
            AIProvider::OpenAI => "gpt-4o-mini".to_string(),
            AIProvider::Anthropic => "claude-3-5-sonnet-20241022".to_string(),
            AIProvider::Local => "llama2".to_string(),
        }
    }

    /// Get the model to use (either specified or default)
    pub fn get_model(&self) -> String {
        self.ai_model.clone().unwrap_or_else(|| self.get_default_model())
    }

    /// Check if this is a full system check (stores in database)
    pub fn is_full_check(&self) -> bool {
        match &self.command {
            Some(Commands::Check { component }) => matches!(component, CheckComponent::All),
            Some(Commands::Debug { .. }) => false, // Debug commands don't store in database
            Some(Commands::Issues { .. }) => false, // Issues commands don't store in database
            None => true, // Default to full check when no subcommand
        }
    }

    /// Get the check component to execute
    pub fn get_check_component(&self) -> CheckComponent {
        match &self.command {
            Some(Commands::Check { component }) => component.clone(),
            Some(Commands::Debug { .. }) => CheckComponent::Debug,
            Some(Commands::Issues { .. }) => CheckComponent::All, // Issues commands default to all
            None => CheckComponent::All, // Default to all if no subcommand
        }
    }
} 