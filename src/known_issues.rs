use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnownIssue {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: IssueCategory,
    pub severity: IssueSeverity,
    pub patterns: Vec<String>, // Patterns to match in system output
    pub keywords: Vec<String>, // Keywords to search for
    pub symptoms: Vec<String>, // Common symptoms
    pub verification_commands: Vec<String>, // Commands to verify the issue
    pub fix_commands: Vec<String>, // Commands to fix the issue
    pub prerequisites: Vec<String>, // Prerequisites for this issue
    pub distribution_specific: Option<String>, // Specific to a Linux distribution
    pub tags: Vec<String>, // Additional tags for categorization
    pub next_steps: Vec<String>, // Steps to take before attempting fixes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueCategory {
    System,
    Container,
    Kubernetes,
    Cgroups,
    Systemd,
    Journal,
    Network,
    Storage,
    Security,
    Performance,
    Configuration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone)]
pub struct IssueMatch {
    pub issue: KnownIssue,
    pub confidence: f32,
    pub matched_patterns: Vec<String>,
    pub matched_keywords: Vec<String>,
}

pub struct KnownIssuesDatabase {
    issues: Arc<RwLock<HashMap<String, KnownIssue>>>,
}

impl KnownIssuesDatabase {
    pub async fn new() -> Self {
        let db = Self {
            issues: Arc::new(RwLock::new(HashMap::new())),
        };
        db.initialize_default_issues().await;
        db
    }

    pub async fn add_issue(&self, issue: KnownIssue) {
        let mut issues = self.issues.write().await;
        issues.insert(issue.id.clone(), issue);
    }

    pub async fn get_issue(&self, id: &str) -> Option<KnownIssue> {
        let issues = self.issues.read().await;
        issues.get(id).cloned()
    }

    pub async fn get_all_issues(&self) -> Vec<KnownIssue> {
        let issues = self.issues.read().await;
        issues.values().cloned().collect()
    }

    pub async fn search_issues(&self, query: &str) -> Vec<KnownIssue> {
        let issues = self.issues.read().await;
        let query_lower = query.to_lowercase();
        
        issues.values()
            .filter(|issue| {
                issue.title.to_lowercase().contains(&query_lower) ||
                issue.description.to_lowercase().contains(&query_lower) ||
                issue.keywords.iter().any(|k| query_lower.contains(&k.to_lowercase())) ||
                issue.tags.iter().any(|t| query_lower.contains(&t.to_lowercase()))
            })
            .cloned()
            .collect()
    }

    pub async fn match_issues(&self, system_output: &str, category: Option<IssueCategory>) -> Vec<IssueMatch> {
        let issues = self.issues.read().await;
        let output_lower = system_output.to_lowercase();
        let mut matches = Vec::new();

        for issue in issues.values() {
            // Skip if category doesn't match
            if let Some(ref cat) = category {
                if std::mem::discriminant(&issue.category) != std::mem::discriminant(cat) {
                    continue;
                }
            }

            let mut confidence = 0.0;
            let mut matched_patterns = Vec::new();
            let mut matched_keywords = Vec::new();

            // Check patterns
            for pattern in &issue.patterns {
                if output_lower.contains(&pattern.to_lowercase()) {
                    confidence += 0.4;
                    matched_patterns.push(pattern.clone());
                }
            }

            // Check keywords
            for keyword in &issue.keywords {
                if output_lower.contains(&keyword.to_lowercase()) {
                    confidence += 0.2;
                    matched_keywords.push(keyword.clone());
                }
            }

            // Check symptoms
            for symptom in &issue.symptoms {
                if output_lower.contains(&symptom.to_lowercase()) {
                    confidence += 0.3;
                }
            }

            // Check tags
            for tag in &issue.tags {
                if output_lower.contains(&tag.to_lowercase()) {
                    confidence += 0.1;
                }
            }

            if confidence > 0.1 {
                matches.push(IssueMatch {
                    issue: issue.clone(),
                    confidence,
                    matched_patterns,
                    matched_keywords,
                });
            }
        }

        // Sort by confidence (highest first)
        matches.sort_by(|a, b| b.confidence.partial_cmp(&a.confidence).unwrap_or(std::cmp::Ordering::Equal));
        matches
    }

    async fn initialize_default_issues(&self) {
        let mut issues = self.issues.write().await;
        let issues_vec = vec![
            // System issues
            KnownIssue {
                id: "system-high-memory-usage".to_string(),
                title: "High Memory Usage".to_string(),
                description: "System memory usage is critically high, potentially causing performance issues or system instability.".to_string(),
                category: IssueCategory::System,
                severity: IssueSeverity::High,
                patterns: vec![
                    "out of memory".to_string(),
                    "memory pressure".to_string(),
                    "oom-killer".to_string(),
                ],
                keywords: vec!["memory", "ram", "swap", "oom"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "High memory usage (>90%)".to_string(),
                    "System swapping".to_string(),
                    "Processes being killed by OOM killer".to_string(),
                ],
                verification_commands: vec![
                    "free -h".to_string(),
                    "cat /proc/meminfo".to_string(),
                    "dmesg | grep -i oom".to_string(),
                ],
                fix_commands: vec![
                    "Identify memory-hungry processes: ps aux --sort=-%mem | head -10".to_string(),
                    "Kill unnecessary processes: kill -9 <PID>".to_string(),
                    "Increase swap space if needed".to_string(),
                ],
                prerequisites: vec![],
                distribution_specific: None,
                tags: vec!["memory", "performance", "system"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Identify which processes are consuming the most memory".to_string(),
                    "Check if there are any memory leaks in running applications".to_string(),
                    "Verify if swap space is being used and how much".to_string(),
                ],
            },

            // Container issues
            KnownIssue {
                id: "container-oom".to_string(),
                title: "Container Out of Memory".to_string(),
                description: "Container is running out of memory and may be killed by the OOM killer.".to_string(),
                category: IssueCategory::Container,
                severity: IssueSeverity::Critical,
                patterns: vec![
                    "container oom".to_string(),
                    "docker oom".to_string(),
                    "podman oom".to_string(),
                ],
                keywords: vec!["container", "docker", "podman", "oom", "memory"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "Container being killed repeatedly".to_string(),
                    "High memory usage in container".to_string(),
                    "OOM messages in container logs".to_string(),
                ],
                verification_commands: vec![
                    "docker stats".to_string(),
                    "docker logs <container> | grep -i oom".to_string(),
                    "cat /sys/fs/cgroup/memory/memory.usage_in_bytes".to_string(),
                ],
                fix_commands: vec![
                    "Increase container memory limit: docker run --memory=2g <image>".to_string(),
                    "Optimize application memory usage".to_string(),
                    "Add swap space to container".to_string(),
                ],
                prerequisites: vec!["Docker or Podman installed".to_string()],
                distribution_specific: None,
                tags: vec!["container", "memory", "docker", "podman"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Check the current memory usage of the container".to_string(),
                    "Verify the memory limit set for the container".to_string(),
                    "Identify if the application inside the container has memory leaks".to_string(),
                ],
            },

            // Kubernetes issues
            KnownIssue {
                id: "k8s-pod-crashloop".to_string(),
                title: "Kubernetes Pod CrashLoopBackOff".to_string(),
                description: "Pod is in CrashLoopBackOff state, indicating repeated failures to start.".to_string(),
                category: IssueCategory::Kubernetes,
                severity: IssueSeverity::Critical,
                patterns: vec![
                    "crashloopbackoff".to_string(),
                    "back-off restarting failed container".to_string(),
                ],
                keywords: vec!["crashloop", "backoff", "restart", "failed"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "Pod status shows CrashLoopBackOff".to_string(),
                    "High restart count".to_string(),
                    "Container exits immediately after start".to_string(),
                ],
                verification_commands: vec![
                    "kubectl get pods".to_string(),
                    "kubectl describe pod <pod-name>".to_string(),
                    "kubectl logs <pod-name> --previous".to_string(),
                ],
                fix_commands: vec![
                    "Check application logs: kubectl logs <pod-name>".to_string(),
                    "Verify container image: kubectl describe pod <pod-name>".to_string(),
                    "Check resource limits and requests".to_string(),
                ],
                prerequisites: vec!["kubectl configured".to_string()],
                distribution_specific: None,
                tags: vec!["kubernetes", "pod", "crashloop", "restart"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Check the logs of the failing container to understand the root cause".to_string(),
                    "Verify the container image and its configuration".to_string(),
                    "Check resource limits and requests for the pod".to_string(),
                ],
            },

            // Systemd issues
            KnownIssue {
                id: "systemd-failed-units".to_string(),
                title: "Systemd Failed Units".to_string(),
                description: "One or more systemd units have failed to start or are in a failed state.".to_string(),
                category: IssueCategory::Systemd,
                severity: IssueSeverity::High,
                patterns: vec![
                    "failed units".to_string(),
                    "unit failed".to_string(),
                    "activating failed".to_string(),
                ],
                keywords: vec!["failed", "unit", "systemd", "service"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "Systemd units in failed state".to_string(),
                    "Services not starting".to_string(),
                    "System boot issues".to_string(),
                ],
                verification_commands: vec![
                    "systemctl --failed".to_string(),
                    "systemctl status <unit-name>".to_string(),
                    "journalctl -u <unit-name>".to_string(),
                ],
                fix_commands: vec![
                    "Restart failed unit: systemctl restart <unit-name>".to_string(),
                    "Check unit configuration: systemctl cat <unit-name>".to_string(),
                    "Reset failed unit: systemctl reset-failed <unit-name>".to_string(),
                ],
                prerequisites: vec!["systemd running".to_string()],
                distribution_specific: None,
                tags: vec!["systemd", "service", "unit", "failed"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Check the specific error messages in the unit status".to_string(),
                    "Verify the unit configuration and dependencies".to_string(),
                    "Check if required files or directories exist".to_string(),
                ],
            },

            // Cgroup issues
            KnownIssue {
                id: "cgroup-memory-limit".to_string(),
                title: "Cgroup Memory Limit Exceeded".to_string(),
                description: "Process or container has exceeded its cgroup memory limit.".to_string(),
                category: IssueCategory::Cgroups,
                severity: IssueSeverity::High,
                patterns: vec![
                    "memory limit exceeded".to_string(),
                    "cgroup memory".to_string(),
                    "memory.usage_in_bytes".to_string(),
                ],
                keywords: vec!["cgroup", "memory", "limit", "exceeded"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "Process killed due to memory limit".to_string(),
                    "High memory usage in cgroup".to_string(),
                    "Memory pressure warnings".to_string(),
                ],
                verification_commands: vec![
                    "cat /sys/fs/cgroup/memory/memory.usage_in_bytes".to_string(),
                    "cat /sys/fs/cgroup/memory/memory.limit_in_bytes".to_string(),
                    "cat /proc/cgroups".to_string(),
                ],
                fix_commands: vec![
                    "Increase memory limit for cgroup".to_string(),
                    "Optimize application memory usage".to_string(),
                ],
                prerequisites: vec!["cgroups enabled".to_string()],
                distribution_specific: None,
                tags: vec!["cgroup", "memory", "limit"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Check that your system has enough available memory".to_string(),
                    "Verify the current memory limit for the cgroup".to_string(),
                    "Identify which process or container is hitting the limit".to_string(),
                ],
            },

            // Journal issues
            KnownIssue {
                id: "journal-errors".to_string(),
                title: "System Journal Errors".to_string(),
                description: "Multiple error messages in system journal indicating system problems.".to_string(),
                category: IssueCategory::Journal,
                severity: IssueSeverity::Medium,
                patterns: vec![
                    "error".to_string(),
                    "failed".to_string(),
                    "critical".to_string(),
                ],
                keywords: vec!["error", "failed", "critical", "journal"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "Multiple error messages in journal".to_string(),
                    "Service failures".to_string(),
                    "System instability".to_string(),
                ],
                verification_commands: vec![
                    "journalctl -p err".to_string(),
                    "journalctl -p err --since '1 hour ago'".to_string(),
                    "journalctl -u <service-name> -p err".to_string(),
                ],
                fix_commands: vec![
                    "Check specific service: journalctl -u <service-name>".to_string(),
                    "Restart problematic service: systemctl restart <service-name>".to_string(),
                    "Check system logs for patterns".to_string(),
                ],
                prerequisites: vec!["systemd-journald running".to_string()],
                distribution_specific: None,
                tags: vec!["journal", "error", "log"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Identify which specific services are generating the most errors".to_string(),
                    "Check the timing and frequency of error messages".to_string(),
                    "Look for patterns in the error messages to identify root causes".to_string(),
                ],
            },

            // Network issues
            KnownIssue {
                id: "network-connectivity".to_string(),
                title: "Network Connectivity Issues".to_string(),
                description: "System experiencing network connectivity problems.".to_string(),
                category: IssueCategory::Network,
                severity: IssueSeverity::High,
                patterns: vec![
                    "network unreachable".to_string(),
                    "connection refused".to_string(),
                    "timeout".to_string(),
                ],
                keywords: vec!["network", "connectivity", "unreachable", "timeout"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "Cannot reach external hosts".to_string(),
                    "Slow network performance".to_string(),
                    "Connection timeouts".to_string(),
                ],
                verification_commands: vec![
                    "ping -c 3 8.8.8.8".to_string(),
                    "ip route show".to_string(),
                    "ss -tuln".to_string(),
                ],
                fix_commands: vec![
                    "Check network configuration: ip addr show".to_string(),
                    "Restart network service: systemctl restart NetworkManager".to_string(),
                    "Check firewall rules: iptables -L".to_string(),
                ],
                prerequisites: vec![],
                distribution_specific: None,
                tags: vec!["network", "connectivity", "dns"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Check if the network interface is properly configured".to_string(),
                    "Verify DNS resolution is working correctly".to_string(),
                    "Test connectivity to different network destinations".to_string(),
                ],
            },

            // Storage issues
            KnownIssue {
                id: "disk-space-full".to_string(),
                title: "Disk Space Full".to_string(),
                description: "Disk space is critically low, potentially causing system issues.".to_string(),
                category: IssueCategory::Storage,
                severity: IssueSeverity::Critical,
                patterns: vec![
                    "no space left on device".to_string(),
                    "disk full".to_string(),
                    "quota exceeded".to_string(),
                ],
                keywords: vec!["disk", "space", "full", "quota"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "Cannot write to filesystem".to_string(),
                    "High disk usage (>90%)".to_string(),
                    "Application failures due to disk space".to_string(),
                ],
                verification_commands: vec![
                    "df -h".to_string(),
                    "du -sh /*".to_string(),
                    "find /var/log -name '*.log' -exec ls -lh {} \\;".to_string(),
                ],
                fix_commands: vec![
                    "Clean up old files: find /tmp -type f -mtime +7 -delete".to_string(),
                    "Rotate log files: journalctl --vacuum-time=7d".to_string(),
                    "Remove old packages: pacman -Sc".to_string(),
                ],
                prerequisites: vec![],
                distribution_specific: None,
                tags: vec!["disk", "storage", "space"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Identify which directories are consuming the most space".to_string(),
                    "Check for large files that can be safely removed".to_string(),
                    "Verify if log rotation is working properly".to_string(),
                ],
            },

            // Network issues
            KnownIssue {
                id: "network-interface-down".to_string(),
                title: "Network Interface Down".to_string(),
                description: "Network interface is down or not properly configured.".to_string(),
                category: IssueCategory::Network,
                severity: IssueSeverity::High,
                patterns: vec![
                    "interface down".to_string(),
                    "no carrier".to_string(),
                    "link not ready".to_string(),
                ],
                keywords: vec!["interface", "down", "carrier", "link"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "Cannot connect to network".to_string(),
                    "Interface shows DOWN state".to_string(),
                    "No network connectivity".to_string(),
                ],
                verification_commands: vec![
                    "ip addr show".to_string(),
                    "ip link show".to_string(),
                    "ethtool <interface>".to_string(),
                ],
                fix_commands: vec![
                    "Bring interface up: ip link set <interface> up".to_string(),
                    "Restart network service: systemctl restart NetworkManager".to_string(),
                    "Check cable connections".to_string(),
                ],
                prerequisites: vec![],
                distribution_specific: None,
                tags: vec!["network", "interface", "connectivity"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Check physical cable connections".to_string(),
                    "Verify interface configuration".to_string(),
                    "Check for hardware issues".to_string(),
                ],
            },

            // Process issues
            KnownIssue {
                id: "high-cpu-usage".to_string(),
                title: "High CPU Usage".to_string(),
                description: "System experiencing high CPU usage, potentially causing performance issues.".to_string(),
                category: IssueCategory::Performance,
                severity: IssueSeverity::Medium,
                patterns: vec![
                    "high cpu usage".to_string(),
                    "cpu overload".to_string(),
                    "load average high".to_string(),
                ],
                keywords: vec!["cpu", "load", "performance", "usage"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "System slow response".to_string(),
                    "High load average".to_string(),
                    "Processes consuming excessive CPU".to_string(),
                ],
                verification_commands: vec![
                    "top -b -n 1".to_string(),
                    "ps aux --sort=-%cpu | head -10".to_string(),
                    "uptime".to_string(),
                ],
                fix_commands: vec![
                    "Identify CPU-intensive processes: ps aux --sort=-%cpu".to_string(),
                    "Kill problematic processes: kill -9 <PID>".to_string(),
                    "Check for runaway processes".to_string(),
                ],
                prerequisites: vec![],
                distribution_specific: None,
                tags: vec!["cpu", "performance", "load"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Identify which processes are consuming the most CPU".to_string(),
                    "Check if there are any runaway processes".to_string(),
                    "Monitor system load over time".to_string(),
                ],
            },

            // Security issues
            KnownIssue {
                id: "failed-login-attempts".to_string(),
                title: "Failed Login Attempts".to_string(),
                description: "Multiple failed login attempts detected, potential security threat.".to_string(),
                category: IssueCategory::Security,
                severity: IssueSeverity::High,
                patterns: vec![
                    "failed login".to_string(),
                    "authentication failure".to_string(),
                    "invalid user".to_string(),
                ],
                keywords: vec!["login", "failed", "authentication", "security"].into_iter().map(|s| s.to_string()).collect(),
                symptoms: vec![
                    "Multiple failed login attempts in logs".to_string(),
                    "Suspicious IP addresses".to_string(),
                    "Account lockouts".to_string(),
                ],
                verification_commands: vec![
                    "journalctl -u ssh | grep 'Failed password'".to_string(),
                    "lastb".to_string(),
                    "fail2ban-client status".to_string(),
                ],
                fix_commands: vec![
                    "Check SSH configuration: cat /etc/ssh/sshd_config".to_string(),
                    "Review failed login logs: journalctl -u ssh".to_string(),
                    "Configure fail2ban if not already done".to_string(),
                ],
                prerequisites: vec![],
                distribution_specific: None,
                tags: vec!["security", "login", "ssh", "authentication"].into_iter().map(|s| s.to_string()).collect(),
                next_steps: vec![
                    "Review recent login attempts and identify suspicious patterns".to_string(),
                    "Check if fail2ban is properly configured and running".to_string(),
                    "Consider implementing additional security measures".to_string(),
                ],
            },
        ];

        // Add all issues to the database
        for issue in issues_vec {
            issues.insert(issue.id.clone(), issue);
        }
    }

    pub async fn format_issue_for_ai(&self, issue: &KnownIssue) -> String {
        format!(
            "KNOWN ISSUE: {}\n\
            Category: {:?}\n\
            Severity: {:?}\n\
            Description: {}\n\
            Next Steps:\n{}\n\
            Verification Commands:\n{}\n\
            Fix Commands:\n{}\n",
            issue.title,
            issue.category,
            issue.severity,
            issue.description,
            issue.next_steps.join("\n"),
            issue.verification_commands.join("\n"),
            issue.fix_commands.join("\n")
        )
    }

    pub async fn get_relevant_issues_for_context(&self, context: &str, category: Option<IssueCategory>) -> Vec<KnownIssue> {
        let matches = self.match_issues(context, category).await;
        matches.into_iter()
            .filter(|m| m.confidence > 0.3) // Only include good matches
            .map(|m| m.issue)
            .collect()
    }
} 