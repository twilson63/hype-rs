use hype_rs::modules::builtins::http::{HttpClient, HttpModule};
use hype_rs::modules::builtins::BuiltinModule;
use std::collections::HashMap;

fn main() {
    println!("═══════════════════════════════════════════");
    println!("  HTTP Module Demo (Rust)");
    println!("═══════════════════════════════════════════\n");

    demo_module_exports();
    
    #[cfg(feature = "http")]
    {
        demo_http_get();
        demo_http_post_json();
        demo_http_fetch();
        demo_error_handling();
        demo_response_status();
    }
    
    #[cfg(not(feature = "http"))]
    {
        println!("⚠️  HTTP feature not enabled");
        println!("Run with: cargo run --example http_demo_rust --features http");
    }

    println!("\n═══════════════════════════════════════════");
    println!("  Demo Complete!");
    println!("═══════════════════════════════════════════");
}

fn demo_module_exports() {
    println!("--- HTTP Module Structure ---\n");
    
    let http_module = HttpModule::new();
    
    println!("Module name: {}", http_module.name());
    
    match http_module.exports() {
        Ok(exports) => {
            println!("✓ Module exports loaded");
            if let Some(id) = exports.get("__id") {
                println!("  ID: {}", id);
            }
            if let Some(desc) = exports.get("__desc") {
                println!("  Description: {}", desc);
            }
            
            println!("\nAvailable functions:");
            for (key, _) in exports.as_object().unwrap() {
                if !key.starts_with("__") {
                    println!("  • {}", key);
                }
            }
        }
        Err(e) => {
            println!("✗ Failed to load exports: {}", e);
        }
    }
    
    println!();
}

#[cfg(feature = "http")]
fn demo_http_get() {
    println!("--- Example 1: Simple GET Request ---\n");
    
    let client = match HttpClient::new() {
        Ok(c) => c,
        Err(e) => {
            println!("✗ Failed to create HTTP client: {}", e);
            return;
        }
    };
    
    println!("Fetching GitHub user: octocat");
    
    match client.get("https://api.github.com/users/octocat") {
        Ok(response) => {
            println!("✓ Request successful");
            println!("  Status: {}", response.status);
            println!("  Status Text: {}", response.status_text);
            println!("  OK: {}", response.ok());
            
            if let Ok(json) = response.json() {
                if let Some(name) = json.get("name") {
                    println!("  Name: {}", name);
                }
                if let Some(location) = json.get("location") {
                    println!("  Location: {}", location);
                }
                if let Some(repos) = json.get("public_repos") {
                    println!("  Public Repos: {}", repos);
                }
            }
        }
        Err(e) => {
            println!("✗ Request failed: {}", e);
        }
    }
    
    println!();
}

#[cfg(feature = "http")]
fn demo_http_post_json() {
    println!("--- Example 2: POST Request ---\n");
    
    let client = match HttpClient::new() {
        Ok(c) => c,
        Err(e) => {
            println!("✗ Failed to create HTTP client: {}", e);
            return;
        }
    };
    
    let json_body = r#"{"title": "Hello from Hype-RS", "body": "Test post", "userId": 1}"#;
    
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    
    println!("Creating new post on JSONPlaceholder...");
    
    match client.post("https://jsonplaceholder.typicode.com/posts", Some(json_body.to_string()), Some(headers)) {
        Ok(response) => {
            println!("✓ POST successful");
            println!("  Status: {}", response.status);
            
            if let Ok(json) = response.json() {
                if let Some(id) = json.get("id") {
                    println!("  Created Post ID: {}", id);
                }
                if let Some(title) = json.get("title") {
                    println!("  Title: {}", title);
                }
            }
        }
        Err(e) => {
            println!("✗ POST failed: {}", e);
        }
    }
    
    println!();
}

#[cfg(feature = "http")]
fn demo_http_fetch() {
    println!("--- Example 3: Universal Fetch API ---\n");
    
    let client = match HttpClient::new() {
        Ok(c) => c,
        Err(e) => {
            println!("✗ Failed to create HTTP client: {}", e);
            return;
        }
    };
    
    let mut headers = HashMap::new();
    headers.insert("Accept".to_string(), "application/vnd.github.v3+json".to_string());
    headers.insert("User-Agent".to_string(), "Hype-RS-HTTP-Client".to_string());
    
    println!("Fetching Rust repository info...");
    
    match client.fetch(
        "GET",
        "https://api.github.com/repos/rust-lang/rust",
        None,
        Some(headers),
        Some(10000), // 10 second timeout
    ) {
        Ok(response) => {
            println!("✓ Fetch successful");
            println!("  Status: {}", response.status);
            
            if let Ok(json) = response.json() {
                if let Some(name) = json.get("name") {
                    println!("  Name: {}", name);
                }
                if let Some(stars) = json.get("stargazers_count") {
                    println!("  Stars: {}", stars);
                }
                if let Some(language) = json.get("language") {
                    println!("  Language: {}", language);
                }
            }
        }
        Err(e) => {
            println!("✗ Fetch failed: {}", e);
        }
    }
    
    println!();
}

#[cfg(feature = "http")]
fn demo_error_handling() {
    println!("--- Example 4: Error Handling ---\n");
    
    let client = match HttpClient::new() {
        Ok(c) => c,
        Err(e) => {
            println!("✗ Failed to create HTTP client: {}", e);
            return;
        }
    };
    
    println!("Attempting to connect to invalid domain...");
    
    match client.get("https://this-domain-does-not-exist-12345.com") {
        Ok(_) => {
            println!("✗ Request succeeded (unexpected)");
        }
        Err(e) => {
            println!("✓ Error handled gracefully:");
            println!("  Error: {}", e);
        }
    }
    
    println!();
}

#[cfg(feature = "http")]
fn demo_response_status() {
    println!("--- Example 5: Response Status Checking ---\n");
    
    let client = match HttpClient::new() {
        Ok(c) => c,
        Err(e) => {
            println!("✗ Failed to create HTTP client: {}", e);
            return;
        }
    };
    
    println!("Fetching non-existent user...");
    
    match client.get("https://api.github.com/users/this-user-definitely-does-not-exist-xyz") {
        Ok(response) => {
            println!("  Status: {}", response.status);
            println!("  Status Text: {}", response.status_text);
            println!("  Is OK (2xx): {}", response.ok());
            
            if response.status == 404 {
                println!("✓ Correctly identified 404 Not Found");
            }
        }
        Err(e) => {
            println!("✗ Request failed: {}", e);
        }
    }
    
    println!();
}
