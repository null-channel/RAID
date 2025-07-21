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
    #[arg(
        long,
        short = 'p',
        value_enum,
        env = "AI_PROVIDER",
        default_value = "open-ai"
    )]
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

    /// Maximum tool calls for AI agent mode (default: 50)
    #[arg(long, env = "AI_MAX_TOOL_CALLS", default_value = "50")]
    pub ai_max_tool_calls: usize,

    /// Enable iterative AI agent mode (multiple rounds of tool calls)
    #[arg(long)]
    pub ai_agent_mode: bool,

    /// Run without AI analysis (just collect and display system info)
    #[arg(long)]
    pub dry_run: bool,

    /// Enable verbose output
    #[arg(long, short = 'v', default_value = "false")]
    pub verbose: bool,

    /// Output format (text, yaml, json)
    #[arg(long, short = 'o', value_enum, default_value = "text")]
    pub output_format: OutputFormat,

    /// Configuration file path
    #[arg(long, short = 'c')]
    pub config: Option<String>,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Disable progress indicators
    #[arg(long)]
    pub no_progress: bool,

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
    /// Configuration management
    Config {
        /// Config action to perform
        #[arg(value_enum)]
        action: ConfigAction,
        /// Output path for generated config (for init action)
        #[arg(long, short = 'o')]
        output: Option<String>,
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
pub enum ConfigAction {
    /// Initialize a new configuration file
    Init,
    /// Show current configuration (merged from all sources)
    Show,
    /// Validate configuration file
    Validate,
    /// Show configuration file locations
    Locations,
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
    /// [Arch] List all installed packages
    PacmanListPackages,
    /// [Arch] List orphaned packages
    PacmanOrphans,
    /// [Arch] Check package file integrity
    PacmanCheckFiles,
    /// [Arch] Check for available updates
    Checkupdates,
    /// [Arch] Show package cache information
    PaccacheInfo,
    /// [Arch] Analyze boot time
    SystemdAnalyzeTime,
    /// [Arch] Show boot critical chain
    SystemdAnalyzeCriticalChain,
    /// [Arch] Show boot blame (slowest services)
    SystemdAnalyzeBlame,
    /// [Arch] List all boot sessions
    JournalctlListBoots,
    /// [Arch] List loaded kernel modules
    Lsmod,
    /// [Arch] Show failed systemd units
    SystemctlFailed,
    /// [Arch] Check if reboot needed (kernel updates)
    NeedsReboot,
    /// [Arch] Show active pacman mirrors
    PacmanMirrorlist,
    /// [Arch] Show AUR helper information
    AurHelperInfo,
    /// [K8s] Get deployments in namespace
    KubectlGetDeployments,
    /// [K8s] Get ConfigMaps in namespace  
    KubectlGetConfigmaps,
    /// [K8s] Get pod logs
    KubectlLogs,
    /// [K8s] Get resource usage (top pods)
    KubectlTopPods,
    /// [K8s] Get resource usage (top nodes)
    KubectlTopNodes,
    /// [K8s] Get cluster information
    KubectlClusterInfo,
    /// [K8s] Get persistent volumes
    KubectlGetPv,
    /// [K8s] Get persistent volume claims
    KubectlGetPvc,
    /// [K8s] Get kubelet service status
    KubeletStatus,
    /// [K8s] Get kubelet logs
    KubeletLogs,
    /// [K8s] Get kubelet configuration
    KubeletConfig,
    /// [K8s] Check etcd cluster health
    EtcdClusterHealth,
    /// [K8s] Get etcd member list
    EtcdMemberList,
    /// [K8s] Check etcd endpoint health
    EtcdEndpointHealth,
    /// [K8s] Get etcd endpoint status and database size
    EtcdEndpointStatus,
    /// [Network] Show IP addresses and network interfaces
    IpAddr,
    /// [Network] Show routing table
    IpRoute,
    /// [Network] Show socket statistics and listening ports
    Ss,
    /// [Network] Test network connectivity with ping
    Ping,
    /// [Network] Trace network route to destination
    Traceroute,
    /// [Network] Perform DNS lookup
    Dig,
    /// [Network] Show firewall rules (iptables)
    Iptables,
    /// [Network] Show ethernet interface statistics
    Ethtool,
    /// [Network] Show network connections and routing tables (deprecated netstat alternative)
    NetstatLegacy,
    /// [Network] Show ARP table
    ArpTable,
    /// [Network] Show network interface statistics
    InterfaceStats,
    /// [Network] Test bandwidth between hosts
    Iperf3,
    /// [Network] Show network namespaces
    NetworkNamespaces,
    /// [Network] Monitor network traffic
    TcpdumpSample,
    /// [Network] Show bridge information
    BridgeInfo,
    /// [Network] Show wireless interface information
    WirelessInfo,
    /// [Network] Show firewall rules (nftables)
    Nftables,
    /// [Network] Test DNS resolution speed
    DnsTest,
    /// [eBPF] List all loaded BPF programs
    BpftoolProgList,
    /// [eBPF] Show detailed information about a BPF program
    BpftoolProgShow,
    /// [eBPF] Dump BPF program bytecode (translated)
    BpftoolProgDumpXlated,
    /// [eBPF] Dump BPF program JIT-compiled code
    BpftoolProgDumpJited,
    /// [eBPF] List all BPF maps
    BpftoolMapList,
    /// [eBPF] Show detailed information about a BPF map
    BpftoolMapShow,
    /// [eBPF] Dump BPF map contents
    BpftoolMapDump,
    /// [eBPF] List all BPF links (attachments)
    BpftoolLinkList,
    /// [eBPF] Show BPF feature support information
    BpftoolFeatureProbe,
    /// [eBPF] List BPF network attachments
    BpftoolNetList,
    /// [eBPF] List BPF cgroup attachments
    BpftoolCgroupList,
    /// [eBPF] List BPF BTF (Type Format) objects
    BpftoolBtfList,
    /// [eBPF] Check if BPF filesystem is mounted
    BpfMountCheck,
    /// [eBPF] List BPF pinned objects in filesystem
    BpfLsPinned,
    /// [eBPF] Show kernel BPF configuration
    BpfKernelConfig,
    /// [eBPF] Simple BPF tracing (syscall counts)
    BpftraceSyscalls,
    /// [eBPF] List available BPF tracepoints
    BpftraceListTracepoints,
    /// [eBPF] Check BPF JIT compiler status
    BpfJitStatus,
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
    ProvideAnalysis { analysis: String },
    /// Ask user for more information
    AskUser { question: String },
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
        self.ai_model
            .clone()
            .unwrap_or_else(|| self.get_default_model())
    }

    /// Check if this is a full system check (stores in database)
    pub fn is_full_check(&self) -> bool {
        match &self.command {
            Some(Commands::Check { component }) => matches!(component, CheckComponent::All),
            Some(Commands::Debug { .. }) => false, // Debug commands don't store in database
            Some(Commands::Issues { .. }) => false, // Issues commands don't store in database
            Some(Commands::Config { .. }) => false, // Config commands don't store in database
            None => true,                          // Default to full check when no subcommand
        }
    }

    /// Get the check component to execute
    pub fn get_check_component(&self) -> CheckComponent {
        match &self.command {
            Some(Commands::Check { component }) => component.clone(),
            Some(Commands::Debug { .. }) => CheckComponent::Debug,
            Some(Commands::Issues { .. }) => CheckComponent::All, // Issues commands default to all
            Some(Commands::Config { .. }) => CheckComponent::All, // Config commands default to all
            None => CheckComponent::All,                          // Default to all if no subcommand
        }
    }
}
