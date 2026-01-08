mod api;
mod config;
mod error;
mod handlers;
mod intent;

use clap::Parser;
use colored::Colorize;
use config::Config;
use error::{AskError, Result};
use intent::{detect_intent, Intent};
use std::io::{self, BufRead, Write};

const VERSION: &str = env!("CARGO_PKG_VERSION");

const HELP_TEXT: &str = r#"ask - A multi-purpose CLI query tool

Usage: ask <query>
       ask -i              (interactive mode - avoids shell escaping issues)
       echo "query" | ask  (pipe mode)

Examples:
    ask how do I compress a folder
    ask what is using port 8080
    ask system disk
    ask prompt continue with install
    ask explain grep
    ask -i                 # then type: what is 2 + 2?

Note: For queries with special characters (? & ! *), use quotes or interactive mode:
    ask "what is the capital of france?"
    ask -i
"#;

#[derive(Parser)]
#[command(name = "ask", version = VERSION, about = "A multi-purpose CLI query tool")]
struct Cli {
    /// Run in interactive mode
    #[arg(short, long)]
    interactive: bool,

    /// Query to process
    #[arg(trailing_var_arg = true)]
    query: Vec<String>,
}

fn get_query(cli: &Cli) -> Result<Option<String>> {
    // Interactive mode
    if cli.interactive {
        print!("ask> ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        return Ok(Some(input.trim().to_string()));
    }

    // Pipe mode - read from stdin if no args and stdin is not a tty
    if cli.query.is_empty() && !atty::is(atty::Stream::Stdin) {
        let stdin = io::stdin();
        let mut lines = Vec::new();
        for line in stdin.lock().lines() {
            lines.push(line?);
        }
        return Ok(Some(lines.join("\n").trim().to_string()));
    }

    // Args mode
    if !cli.query.is_empty() {
        return Ok(Some(cli.query.join(" ")));
    }

    // No query - show help
    Ok(None)
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        match e {
            AskError::MissingApiKey => {
                // Already printed helpful message
                std::process::exit(1);
            }
            _ => {
                eprintln!("{}: {}", "Error".red(), e);
                std::process::exit(1);
            }
        }
    }
}

async fn run() -> Result<()> {
    // Handle --version/-v before clap parsing for simpler output
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 && (args[1] == "-v" || args[1] == "--version") {
        println!("ask {}", VERSION);
        return Ok(());
    }
    if args.len() == 2 && (args[1] == "-h" || args[1] == "--help") {
        print!("{}", HELP_TEXT);
        return Ok(());
    }

    let cli = Cli::parse();

    // Load config
    let mut config = Config::load()?;

    // Get query
    let query = match get_query(&cli)? {
        Some(q) if !q.is_empty() => q,
        _ => {
            print!("{}", HELP_TEXT);
            return Ok(());
        }
    };

    // Detect intent and route
    let intent = detect_intent(&query);

    match intent {
        Intent::Config(args) => handlers::config::handle(&args, &mut config)?,
        Intent::Prompt(args) => handlers::prompt::handle(&args)?,
        Intent::System(args) => handlers::system::handle(&args)?,
        Intent::SystemQuery(q) => handlers::system::handle_query(&q)?,
        Intent::Howto(q) => handlers::howto::handle(&q, &config).await?,
        Intent::Explain(q) => handlers::explain::handle(&q)?,
        Intent::Exec(_) => {
            println!("Exec command generation is not yet implemented");
        }
        Intent::Ai(q) => handlers::ai::handle(&q, &config).await?,
    }

    Ok(())
}
