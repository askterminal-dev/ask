use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug, Clone, PartialEq)]
pub enum SystemResource {
    Disk,
    Memory,
    Cpu,
    Ports,
    Uptime,
    Os,
}

impl SystemResource {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "disk" | "storage" | "space" => Some(SystemResource::Disk),
            "memory" | "mem" | "ram" => Some(SystemResource::Memory),
            "cpu" | "processor" => Some(SystemResource::Cpu),
            "ports" | "network" => Some(SystemResource::Ports),
            "uptime" | "up" => Some(SystemResource::Uptime),
            "os" | "version" => Some(SystemResource::Os),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Intent {
    Config(String),
    Prompt(String),
    System(String),
    SystemQuery(String),
    Howto(String),
    Explain(String),
    #[allow(dead_code)]
    Exec(String),
    Ai(String),
}

// Lazy-compiled regex patterns
static HOWTO_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^how (do i|to|can i|would i) ").unwrap());

static SYSTEM_QUERY_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^what is using ").unwrap());

static EXPLAIN_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^(explain|describe) ").unwrap());

static AI_QUESTION_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)^(who|when|where|why|what) ").unwrap());

pub fn detect_intent(query: &str) -> Intent {
    let q = query.trim();
    let q_lower = q.to_lowercase();

    // Explicit subcommands
    if q_lower.starts_with("config") {
        let rest = q.get(6..).map(|s| s.trim()).unwrap_or("");
        return Intent::Config(rest.to_string());
    }
    if q_lower.starts_with("prompt ") {
        return Intent::Prompt(q.get(7..).unwrap_or("").to_string());
    }
    if q_lower.starts_with("system ") {
        return Intent::System(q.get(7..).unwrap_or("").to_string());
    }
    if q_lower.starts_with("exec ") {
        return Intent::Exec(q.get(5..).unwrap_or("").to_string());
    }
    if q_lower.starts_with("run ") {
        return Intent::Exec(q.get(4..).unwrap_or("").to_string());
    }

    // Pattern matching
    if HOWTO_PATTERN.is_match(&q_lower) {
        return Intent::Howto(q.to_string());
    }
    if SYSTEM_QUERY_PATTERN.is_match(&q_lower) {
        return Intent::SystemQuery(q.to_string());
    }
    if EXPLAIN_PATTERN.is_match(&q_lower) {
        return Intent::Explain(q.to_string());
    }
    if AI_QUESTION_PATTERN.is_match(&q_lower) {
        return Intent::Ai(q.to_string());
    }

    // Default to AI
    Intent::Ai(q.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_intent() {
        match detect_intent("config show") {
            Intent::Config(args) => assert_eq!(args, "show"),
            _ => panic!("Expected Config intent"),
        }
        match detect_intent("config api_key=test") {
            Intent::Config(args) => assert_eq!(args, "api_key=test"),
            _ => panic!("Expected Config intent"),
        }
    }

    #[test]
    fn test_prompt_intent() {
        match detect_intent("prompt continue?") {
            Intent::Prompt(args) => assert_eq!(args, "continue?"),
            _ => panic!("Expected Prompt intent"),
        }
    }

    #[test]
    fn test_system_intent() {
        match detect_intent("system disk") {
            Intent::System(args) => assert_eq!(args, "disk"),
            _ => panic!("Expected System intent"),
        }
    }

    #[test]
    fn test_howto_intent() {
        match detect_intent("how do I compress a folder") {
            Intent::Howto(q) => assert!(q.contains("compress")),
            _ => panic!("Expected Howto intent"),
        }
        match detect_intent("how to list files") {
            Intent::Howto(q) => assert!(q.contains("list")),
            _ => panic!("Expected Howto intent"),
        }
    }

    #[test]
    fn test_system_query_intent() {
        match detect_intent("what is using port 8080") {
            Intent::SystemQuery(q) => assert!(q.contains("8080")),
            _ => panic!("Expected SystemQuery intent"),
        }
    }

    #[test]
    fn test_explain_intent() {
        match detect_intent("explain grep") {
            Intent::Explain(q) => assert!(q.contains("grep")),
            _ => panic!("Expected Explain intent"),
        }
        match detect_intent("describe ls") {
            Intent::Explain(q) => assert!(q.contains("ls")),
            _ => panic!("Expected Explain intent"),
        }
    }

    #[test]
    fn test_ai_intent() {
        match detect_intent("what is the capital of France") {
            Intent::Ai(q) => assert!(q.contains("capital")),
            _ => panic!("Expected Ai intent"),
        }
        match detect_intent("write me a poem") {
            Intent::Ai(q) => assert!(q.contains("poem")),
            _ => panic!("Expected Ai intent"),
        }
    }

    #[test]
    fn test_system_resource() {
        assert_eq!(SystemResource::from_str("disk"), Some(SystemResource::Disk));
        assert_eq!(SystemResource::from_str("memory"), Some(SystemResource::Memory));
        assert_eq!(SystemResource::from_str("ram"), Some(SystemResource::Memory));
        assert_eq!(SystemResource::from_str("unknown"), None);
    }
}
