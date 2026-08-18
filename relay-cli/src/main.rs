use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::collections::HashMap;
use reqwest::Client;
use std::fs;
use serde::Serialize;

use relay_core::{
    parser::parse_file,
    vars::{resolve_scopes, interpolate},
    http::execute,
    discovery,
};

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

#[derive(Serialize)]
struct TestResult {
    file: String,
    method: String,
    url: String,
    status_code: u16,
    time_ms: u128,
    error: Option<String>,
}

fn escape_xml(s: &str) -> String {
    s.replace("&", "&amp;")
     .replace("<", "&lt;")
     .replace(">", "&gt;")
     .replace("\"", "&quot;")
     .replace("'", "&apos;")
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Run { path, env: _, junit, json } => {
            let files = discovery::find_rl_files(path);
            
            let client = Client::new();
            
            // For now, load empty maps for scopes.
            let global = HashMap::new();
            let collection = HashMap::new();
            let env_vars = HashMap::new();
            let secrets = HashMap::new();
            let request_vars = HashMap::new();

            let mut results = Vec::new();

            for f in files {
                let file_name = f.display().to_string();
                let mut req_file = match parse_file(&f) {
                    Ok(r) => r,
                    Err(e) => {
                        if !json { eprintln!("[ERROR] {}: Failed to parse: {}", file_name, e); }
                        results.push(TestResult {
                            file: file_name,
                            method: "UNKNOWN".to_string(),
                            url: "UNKNOWN".to_string(),
                            status_code: 0,
                            time_ms: 0,
                            error: Some(format!("Parse error: {}", e)),
                        });
                        continue;
                    }
                };

                let resolved_vars = resolve_scopes(&global, &collection, &env_vars, &secrets, &request_vars);
                
                req_file.request.url = interpolate(&req_file.request.url, &resolved_vars);
                for (_k, v) in req_file.request.headers.iter_mut() {
                    *v = interpolate(v, &resolved_vars);
                }
                
                let method = req_file.request.method.clone();
                let url = req_file.request.url.clone();
                
                match execute(&client, &req_file.request).await {
                    Ok(res) => {
                        if res.status >= 200 && res.status < 300 {
                            if !json { println!("[OK] {} {} {} ({}ms)", res.status, method, url, res.time_ms); }
                            results.push(TestResult { file: file_name, method, url, status_code: res.status, time_ms: res.time_ms, error: None });
                        } else {
                            if !json { println!("[ERR] {} {} {} ({}ms)", res.status, method, url, res.time_ms); }
                            results.push(TestResult { file: file_name, method, url, status_code: res.status, time_ms: res.time_ms, error: Some(format!("HTTP Status {}", res.status)) });
                        }
                    },
                    Err(e) => {
                        if !json { println!("[FAIL] {} {}: {}", method, url, e); }
                        results.push(TestResult { file: file_name, method, url, status_code: 0, time_ms: 0, error: Some(e.to_string()) });
                    }
                }
            }

            if *json {
                if let Ok(j) = serde_json::to_string_pretty(&results) {
                    println!("{}", j);
                }
            }

            if *junit {
                let mut xml = String::from("<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n");
                let failures = results.iter().filter(|r| r.error.is_some()).count();
                let total_time_s = results.iter().map(|r| r.time_ms).sum::<u128>() as f64 / 1000.0;
                
                xml.push_str(&format!("<testsuites>\n  <testsuite name=\"relay\" tests=\"{}\" failures=\"{}\" errors=\"0\" time=\"{:.3}\">\n", results.len(), failures, total_time_s));
                
                for res in &results {
                    let time_s = res.time_ms as f64 / 1000.0;
                    let name = escape_xml(&format!("{} {}", res.method, res.url));
                    let classname = escape_xml(&res.file);
                    
                    if let Some(err) = &res.error {
                        xml.push_str(&format!("    <testcase name=\"{}\" classname=\"{}\" time=\"{:.3}\">\n", name, classname, time_s));
                        xml.push_str(&format!("      <failure message=\"{}\">{}</failure>\n", escape_xml(err), escape_xml(err)));
                        xml.push_str("    </testcase>\n");
                    } else {
                        xml.push_str(&format!("    <testcase name=\"{}\" classname=\"{}\" time=\"{:.3}\" />\n", name, classname, time_s));
                    }
                }
                
                xml.push_str("  </testsuite>\n</testsuites>\n");
                
                if let Err(e) = fs::write("relay-report.xml", xml) {
                    if !json { eprintln!("Failed to write JUnit report: {}", e); }
                } else {
                    if !json { println!("JUnit report written to relay-report.xml"); }
                }
            }

            let any_failed = results.iter().any(|r| r.error.is_some());
            if any_failed {
                std::process::exit(1);
            }
        }
    }
}
