mod rpc;
mod command;
mod validator;

use rpc::{JsonRpcRequest, JsonRpcResponse};
use serde_json::json;
use warp::Filter;
use std::convert::Infallible;
use std::fs;
use std::path::Path;

#[tokio::main]
async fn main() {
    // Load command exclusion patterns
    if let Err(e) = validator::try_load_exclusion_patterns() {
        eprintln!("Warning: Failed to load exclusion patterns: {}", e);
        eprintln!("The server will continue but with no command restrictions.");
    } else {
        println!("Command exclusion patterns loaded successfully");
    }

    // POST requests to "/" will be treated as JSON-RPC calls.
    let rpc_route = warp::post()
        .and(warp::path::end())
        .and(warp::body::json())
        .and_then(handle_rpc);
        
    // GET requests to "/context" will return context for AI systems
    let context_route = warp::get()
        .and(warp::path("context"))
        .and(warp::path::end())
        .and_then(handle_context);

    // Combine routes
    let routes = rpc_route.or(context_route);

    println!("MCP command server running on port 3030...");
    warp::serve(routes)
        .run(([0, 0, 0, 0], 3030))
        .await;
}

/// Handler for the /context endpoint that provides AI systems with
/// detailed information about how to interact with this API
async fn handle_context() -> Result<impl warp::Reply, warp::Rejection> {
    // Try multiple possible locations for the .context file
    let possible_paths = vec![
        ".context",                 // Current directory
        "/app/.context",            // Docker container root directory
        "../.context",              // One directory up
        "/.context",                // Root directory
    ];
    
    // Try each path until we find the file
    for path in possible_paths {
        if Path::new(path).exists() {
            match fs::read_to_string(path) {
                Ok(content) => {
                    println!("Successfully read context file from {}", path);
                    // Return the markdown content with appropriate content type
                    return Ok(warp::reply::with_header(
                        content,
                        "Content-Type",
                        "text/markdown",
                    ));
                }
                Err(e) => {
                    println!("Error reading context file from {}: {}", path, e);
                    continue;
                }
            }
        }
    }
    
    // If we get here, we couldn't find or read the .context file
    println!("Could not find .context file in any of the expected locations");
    Err(warp::reject::not_found())
}

async fn handle_rpc(req: JsonRpcRequest) -> Result<impl warp::Reply, Infallible> {
    // Extract parameters (if any)
    let params = req.params.unwrap_or_default();

    // Dispatch on the method
    let response = match req.method.as_str() {
        "command/get" => {
            if let Some(cmd) = params.get("command").and_then(|v| v.as_str()) {
                match command::execute_command(cmd).await {
                    Ok(stdout) => JsonRpcResponse {
                        jsonrpc: "2.0".into(),
                        id: req.id,
                        result: Some(json!({ "stdout": stdout })),
                        error: None,
                    },
                    Err(err) => JsonRpcResponse {
                        jsonrpc: "2.0".into(),
                        id: req.id,
                        result: None,
                        error: Some(json!({
                            "code": -32000,
                            "message": format!("Command execution error: {}", err)
                        })),
                    },
                }
            } else {
                JsonRpcResponse {
                    jsonrpc: "2.0".into(),
                    id: req.id,
                    result: None,
                    error: Some(json!({
                        "code": -32602,
                        "message": "Missing 'command' parameter"
                    })),
                }
            }
        },
        _ => JsonRpcResponse {
            jsonrpc: "2.0".into(),
            id: req.id,
            result: None,
            error: Some(json!({
                "code": -32601,
                "message": "Method not found"
            })),
        },
    };

    Ok(warp::reply::json(&response))
}
