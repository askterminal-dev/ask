use crate::config::Config;
use crate::error::Result;
use crate::handlers::ai;
use once_cell::sync::Lazy;
use regex::Regex;

struct Pattern {
    regex: &'static Lazy<Regex>,
    suggestion: &'static str,
}

// Define each pattern as a separate static
static PATTERN_LIST: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)list.*(file|folder|dir|content)").unwrap());
static PATTERN_COMPRESS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)compress|zip|archive").unwrap());
static PATTERN_FIND: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)find.*(file|folder|dir)").unwrap());
static PATTERN_COUNT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)(count|number).*(line|word)").unwrap());
static PATTERN_SEARCH: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)search.*(text|content|inside)").unwrap());
static PATTERN_RENAME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)rename.*(file|multiple)").unwrap());
static PATTERN_DELETE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)delete|remove").unwrap());
static PATTERN_COPY: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)copy").unwrap());
static PATTERN_PERMISSION: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)permission|chmod").unwrap());
static PATTERN_PROCESS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?i)process|running|kill").unwrap());
static PATTERN_DISK: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)disk.*(usage|space)").unwrap());
static PATTERN_DOWNLOAD: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?i)download").unwrap());

static PATTERNS: &[Pattern] = &[
    Pattern {
        regex: &PATTERN_LIST,
        suggestion: "ls -la           # detailed list\nls -lah          # with human-readable sizes\nls -lt           # sorted by time",
    },
    Pattern {
        regex: &PATTERN_COMPRESS,
        suggestion: "tar -czvf archive.tar.gz folder/\nzip -r archive.zip folder/",
    },
    Pattern {
        regex: &PATTERN_FIND,
        suggestion: "find /path -name \"pattern\"\nlocate filename",
    },
    Pattern {
        regex: &PATTERN_COUNT,
        suggestion: "wc -l filename    # lines\nwc -w filename    # words",
    },
    Pattern {
        regex: &PATTERN_SEARCH,
        suggestion: "grep \"pattern\" file\ngrep -r \"pattern\" folder/",
    },
    Pattern {
        regex: &PATTERN_RENAME,
        suggestion: "mv oldname newname\nrename 's/old/new/' files*",
    },
    Pattern {
        regex: &PATTERN_DELETE,
        suggestion: "rm filename\nrm -r folder/\nfind . -name \"*.tmp\" -delete",
    },
    Pattern {
        regex: &PATTERN_COPY,
        suggestion: "cp source dest\ncp -r folder/ dest/\nrsync -av source/ dest/",
    },
    Pattern {
        regex: &PATTERN_PERMISSION,
        suggestion: "chmod +x script.sh\nchmod 755 file\nchmod -R 644 folder/",
    },
    Pattern {
        regex: &PATTERN_PROCESS,
        suggestion: "ps aux | grep name\npgrep name\nkill PID\npkill name",
    },
    Pattern {
        regex: &PATTERN_DISK,
        suggestion: "df -h\ndu -sh folder/\ndu -sh * | sort -h",
    },
    Pattern {
        regex: &PATTERN_DOWNLOAD,
        suggestion: "curl -O url\nwget url",
    },
];

pub async fn handle(query: &str, config: &Config) -> Result<()> {
    let q = query.to_lowercase();
    let mut suggestions: Vec<&str> = Vec::new();

    for pattern in PATTERNS {
        if pattern.regex.is_match(&q) {
            suggestions.push(pattern.suggestion);
        }
    }

    if !suggestions.is_empty() {
        println!("Try:\n");
        for suggestion in suggestions {
            println!("{}", suggestion);
            println!();
        }
    } else {
        println!("No local suggestions for: {}", query);
        if !config.api_key.is_empty() {
            println!("Asking AI for help...\n");
            ai::handle(query, config).await?;
        } else {
            println!(
                "Tip: Try 'ask explain <command>' or configure an API key for AI assistance"
            );
        }
    }

    Ok(())
}
