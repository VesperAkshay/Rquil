use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "relay")]
#[command(about = "Relay API Client CLI", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a request or a collection of requests
    Run {
        /// Path to a .rl file or a directory of .rl files
        path: PathBuf,

        /// Environment to use (e.g. prod, dev)
        #[arg(short, long)]
        env: Option<String>,

        /// Output results in JUnit XML format
        #[arg(long)]
        junit: bool,

        /// Output results in JSON format
        #[arg(long)]
        json: bool,
    },
}

mod discovery;

fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Run { path, env, junit, json } => {
            println!("Parsed command: relay run");
            println!("  Path: {}", path.display());
            println!("  Env: {:?}", env);
            println!("  JUnit flag: {}", junit);
            println!("  JSON flag: {}", json);

            let files = discovery::find_rl_files(path);
            println!("  Discovered {} .rl files:", files.len());
            for f in files {
                println!("    - {}", f.display());
            }
        }
    }
}
