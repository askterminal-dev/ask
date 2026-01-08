use crate::error::Result;
use std::io::{self, Write};

pub fn handle(args: &str) -> Result<()> {
    print!("{} [y/N]: ", args);
    io::stdout().flush()?;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let response = input.trim().to_lowercase();
            if response == "y" || response == "yes" {
                println!("Yes");
                std::process::exit(0);
            } else {
                println!("No");
                std::process::exit(1);
            }
        }
        Err(_) => {
            println!("\nAborted");
            std::process::exit(1);
        }
    }
}
