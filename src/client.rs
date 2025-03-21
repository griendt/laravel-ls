use std::error::Error;

use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = tokio::net::TcpStream::connect("127.0.0.1:8080").await?;

    let initialize_request: Value = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "initialize",
        "params": {
            "process_id": null,
            "rootUri": null,
            "capabilities": {},
        }
    });

    let initialize_request_str = initialize_request.to_string();

    let initialize_request_formatted: String = format!(
        "Content-Length: {}\r\n\r\n{}",
        initialize_request_str.len(),
        initialize_request_str,
    );

    stream
        .write_all(initialize_request_formatted.as_bytes())
        .await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;

    let response = String::from_utf8_lossy(&buffer[..n]);

    println!("Received initialize response: {:?}", response);

    let execute_command_request: Value = json!(
        {
            "jsonrpc": "2.0",
            "id": 2,
            "method": "workspace/executeCommand",
            "params": {
                "command": "banaan",
                "arguments": [{
                    "title": "Hello!",
                    "message": "Hello from the client!",
                    "description": "This is a description!",
                }]
            }
        }
    );

    let execute_command_str = execute_command_request.to_string();
    let execute_command_formatted = format!(
        "Content-Length: {}\r\n\r\n{}",
        execute_command_str.len(),
        execute_command_str,
    );

    stream
        .write_all(execute_command_formatted.as_bytes())
        .await?;

    let mut buffer = [0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);

    println!("Received command response: {:?}", response);

    Ok(())
}
