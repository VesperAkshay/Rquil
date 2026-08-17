use relay_core::parser::parse_file;
use relay_core::http::execute;
use relay_core::vars::interpolate;
use reqwest::Client;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    // 1. Setup a simple variable for interpolation
    let mut vars = HashMap::new();
    vars.insert("greeting".to_string(), "hello_from_relay_core".to_string());

    // 2. Parse the sample .rl file
    println!("Parsing examples/sample.rl...");
    let req_file = parse_file("examples/sample.rl").expect("Failed to parse sample.rl");
    
    // 3. Resolve variables (only URL for this simple manual test)
    let mut resolved_req = req_file.request;
    resolved_req.url = interpolate(&resolved_req.url, &vars);

    // 4. Send the request via our executor
    let client = Client::new();
    println!("Sending {} request to: {}", resolved_req.method, resolved_req.url);
    
    let response = execute(&client, &resolved_req).await.expect("HTTP request failed");
    
    // 5. Output results
    println!("\n--- Response ---");
    println!("Status: {}", response.status);
    println!("Time: {}ms", response.time_ms);
    println!("Body: \n{}", response.body);
}
