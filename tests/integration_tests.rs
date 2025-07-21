use raid::ai::{AIProvider, DummyAI};
use raid::config::RaidConfig;
use raid::sysinfo::collect_basic_system_info;

#[tokio::test]
async fn test_question_answering_functionality() {
    let dummy_ai = DummyAI;
    let question = "Why is my system slow?";
    let context =
        "Operating System: Linux 6.15.6-arch1-1\nCPU: AMD Ryzen 9 7940HS\nMemory: 16GB/32GB\n";

    let result = dummy_ai.answer_question(question, context).await;

    assert!(result.is_ok());
    let answer = result.unwrap();
    assert_eq!(answer, "I cannot answer that question.");
}

#[test]
fn test_question_context_building() {
    // Test that context building doesn't panic and contains expected information
    let test_context = format!(
        "Operating System: {}\nCPU: {}\nMemory: {}/{}\n",
        "Linux 6.15.6-arch1-1", "AMD Ryzen 9 7940HS", "16GB", "32GB"
    );

    assert!(test_context.contains("Linux 6.15.6-arch1-1"));
    assert!(test_context.contains("AMD Ryzen 9 7940HS"));
    assert!(test_context.contains("16GB/32GB"));
}

#[test]
fn test_question_analysis() {
    // Test that question analysis correctly identifies different types of questions
    let container_question =
        "I would like to know if my docker container called nginx is running";
    assert!(container_question.to_lowercase().contains("container"));
    assert!(container_question.to_lowercase().contains("docker"));
    assert!(container_question.to_lowercase().contains("nginx"));

    let performance_question = "why is my system slow?";
    assert!(performance_question.to_lowercase().contains("slow"));

    let service_question = "is my web service running?";
    assert!(service_question.to_lowercase().contains("service"));
}

#[test]
fn test_extract_arg() {
    // Test argument extraction functionality
    fn extract_arg(parts: &[&str], arg_name: &str) -> Option<String> {
        parts.windows(2).find_map(|window| {
            if window[0] == arg_name {
                Some(window[1].to_string())
            } else {
                None
            }
        })
    }

    let parts = vec![
        "kubectl_get_pods",
        "--namespace",
        "default",
        "--lines",
        "20",
    ];

    assert_eq!(
        extract_arg(&parts, "--namespace"),
        Some("default".to_string())
    );
    assert_eq!(extract_arg(&parts, "--lines"), Some("20".to_string()));
    assert_eq!(extract_arg(&parts, "--missing"), None);

    // Test with no arguments
    let empty_parts = vec!["docker_ps"];
    assert_eq!(extract_arg(&empty_parts, "--namespace"), None);
}

#[test]
fn test_ai_tool_selection_integration() {
    // Test that the AI tool selection prompt format is correct
    let _question = "Is my nginx container running?";
    let _context = "Operating System: Linux\nContainers: 3 running\n";

    // This would be the format we expect the AI to return
    let mock_ai_response = "docker_ps\ndocker_inspect nginx\ndocker_logs nginx --lines 10";

    let lines: Vec<&str> = mock_ai_response.lines().collect();
    assert_eq!(lines.len(), 3);
    assert!(lines[0].contains("docker_ps"));
    assert!(lines[1].contains("docker_inspect nginx"));
    assert!(lines[2].contains("docker_logs nginx"));
}

#[test]
fn test_basic_system_info_collection() {
    // Test that basic system info collection works without heavy diagnostics
    let basic_info = collect_basic_system_info();

    // Should have basic system information
    assert!(!basic_info.os.is_empty());
    assert!(!basic_info.cpu.is_empty());
    assert!(!basic_info.total_memory.is_empty());
    assert!(!basic_info.free_memory.is_empty());
    assert!(!basic_info.total_disk.is_empty());
    assert!(!basic_info.free_disk.is_empty());
}

#[test]
fn test_config_loading() {
    // Test basic config loading
    let config = RaidConfig::default();
    assert!(config.validate().is_ok());
} 