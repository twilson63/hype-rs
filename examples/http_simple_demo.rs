use hype_rs::modules::builtins::http::HttpClient;

fn main() {
    println!("🚀 Hype-RS HTTP Module - Simple Demo\n");

    #[cfg(feature = "http")]
    {
        demo_working_requests();
    }

    #[cfg(not(feature = "http"))]
    {
        println!("⚠️  HTTP feature not enabled");
        println!("Run with: cargo run --example http_simple_demo --features http");
    }
}

#[cfg(feature = "http")]
fn demo_working_requests() {
    let client = HttpClient::new().expect("Failed to create HTTP client");

    println!("📡 Example 1: Fetch HTTPBin (GET)\n");
    match client.get("https://httpbin.org/get") {
        Ok(response) => {
            println!("  ✓ Status: {} {}", response.status, response.status_text);
            println!("  ✓ Response OK: {}", response.ok());

            if let Ok(json) = response.json() {
                if let Some(headers) = json.get("headers") {
                    println!("  ✓ Headers received: {}", headers);
                }
            }
        }
        Err(e) => println!("  ✗ Error: {}", e),
    }

    println!("\n📤 Example 2: POST JSON to HTTPBin\n");

    use std::collections::HashMap;
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());

    let body = r#"{"message": "Hello from Hype-RS!", "version": "0.1.0"}"#;

    match client.post(
        "https://httpbin.org/post",
        Some(body.to_string()),
        Some(headers),
    ) {
        Ok(response) => {
            println!("  ✓ Status: {} {}", response.status, response.status_text);

            if let Ok(json) = response.json() {
                if let Some(data) = json.get("json") {
                    println!("  ✓ Server echoed back: {}", data);
                }
            }
        }
        Err(e) => println!("  ✗ Error: {}", e),
    }

    println!("\n🎯 Example 3: Universal Fetch API\n");

    let mut fetch_headers = HashMap::new();
    fetch_headers.insert("User-Agent".to_string(), "Hype-RS/0.1.0".to_string());

    match client.fetch(
        "GET",
        "https://httpbin.org/headers",
        None,
        Some(fetch_headers),
        Some(5000), // 5 second timeout
    ) {
        Ok(response) => {
            println!("  ✓ Status: {} {}", response.status, response.status_text);
            println!("  ✓ Response size: {} bytes", response.body.len());

            if let Ok(json) = response.json() {
                if let Some(headers) = json.get("headers") {
                    if let Some(ua) = headers.get("User-Agent") {
                        println!("  ✓ User-Agent sent: {}", ua);
                    }
                }
            }
        }
        Err(e) => println!("  ✗ Error: {}", e),
    }

    println!("\n❌ Example 4: Handling 404\n");

    match client.get("https://httpbin.org/status/404") {
        Ok(response) => {
            println!("  ✓ Status: {} {}", response.status, response.status_text);
            println!("  ✓ Is OK (2xx): {}", response.ok());

            if response.status == 404 {
                println!("  ✓ Correctly identified 404 Not Found");
            }
        }
        Err(e) => println!("  ✗ Error: {}", e),
    }

    println!("\n⏱️  Example 5: Different HTTP Methods\n");

    let methods = vec![
        ("GET", "https://httpbin.org/get"),
        ("DELETE", "https://httpbin.org/delete"),
    ];

    for (method, url) in methods {
        match client.fetch(method, url, None, None, None) {
            Ok(response) => {
                println!(
                    "  ✓ {} request: {} {}",
                    method, response.status, response.status_text
                );
            }
            Err(e) => {
                println!("  ✗ {} request failed: {}", method, e);
            }
        }
    }

    println!("\n🎉 Demo Complete!\n");
    println!("The HTTP module is fully functional with:");
    println!("  • GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS");
    println!("  • Custom headers");
    println!("  • JSON request/response handling");
    println!("  • Timeout control");
    println!("  • Connection pooling");
    println!("  • HTTP/2 support");
    println!("  • Full TLS/HTTPS support");
}
