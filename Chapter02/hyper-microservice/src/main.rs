// Hyper 1.7.0 HTTP server implementation
// This demonstrates the new hyper architecture where server functionality
// has been moved to hyper-util for better modularity
//
// Key concepts:
// - Tokio streams: Async I/O primitives (like TcpStream) that can read/write data
//   without blocking the thread, allowing efficient handling of multiple connections
// - TokioIo: An adapter that makes tokio's async streams compatible with hyper's I/O system
// - Auto connection handling: 
//   1. Auto-detects HTTP/1.1 vs HTTP/2: Reads the initial request to determine protocol version
//   2. Handles protocol differences transparently: HTTP/1.1 (simple) vs HTTP/2 (multiplexed) without code changes

use hyper::{Response, body::Bytes};  // Response for HTTP responses, Bytes for binary data handling
use hyper::service::service_fn;      // Creates a Service from a function
use hyper_util::rt::TokioIo;           // Wrapper that converts tokio's TcpStream to hyper's I/O interface
use hyper_util::server::conn::auto;    // 1. Auto-detects HTTP/1.1 vs HTTP/2, 2. Handles protocol differences transparently
use std::convert::Infallible;          // Error type that can never fail (for our simple handler)
use std::net::SocketAddr;              // Represents an IP address + port (like 127.0.0.1:8080)
use tokio::net::TcpListener;           // Async TCP listener from tokio

/// Request handler function that processes incoming HTTP requests
/// 
/// In hyper 1.7.0, the request body is `hyper::body::Incoming` which represents
/// a streaming body that can be read asynchronously
/// 
/// Returns a Response with a String body containing "Rust Microservice with Hyper 1.7.0"
async fn handle_request(_req: hyper::Request<hyper::body::Incoming>) -> Result<Response<String>, Infallible> {
    // Create a simple response with our microservice message
    // Using String instead of the complex Body trait for simplicity
    Ok(Response::new("Rust Microservice with Hyper 1.7.0".to_string()))
}

/// Main server function using tokio runtime
/// 
/// The #[tokio::main] attribute automatically sets up the tokio runtime
/// and converts this function to a regular main() function
/// 
/// Return type breakdown:
/// - Result<(), ...>: Success returns (), failure returns an error
/// - Box<dyn std::error::Error>: Any type that implements the Error trait
///   (dyn = dynamic dispatch, allows different error types at runtime)
/// - Send: Error can be sent between threads (needed for async)
/// - Sync: Error can be shared between threads (needed for async)
/// 
/// Example error types this function might return:
/// - std::net::AddrParseError: If "127.0.0.1:8080" can't be parsed
/// - std::io::Error: If TcpListener::bind() fails (port already in use, etc.)
/// - tokio::task::JoinError: If spawned tasks fail, Could happen if there are issues with the HTTP connection handling
/// - hyper::Error: If there's an issue with the HTTP protocol (e.g., invalid request)
/// - hyper_util::server::conn::auto::Error: If there's an issue with the HTTP/2 connection handling
/// - hyper_util::rt::TokioExecutorError: If there's an issue with the tokio runtime
/// - hyper_util::rt::TokioIoError: If there's an issue with the tokio I/O adapter
/// - hyper_util::rt::TokioIoError: If there's an issue with the tokio I/O adapter
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {  // Returns Ok(()) on success, or any error that can be sent between threads
    // Parse the server address (localhost:8080)
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    
    // Bind the TCP listener to the specified address
    // This creates a socket that can accept incoming connections
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on http://127.0.0.1:8080");
    
    // Main server loop - continuously accept new connections
    loop {
        // Accept a new TCP connection
        // This returns a (TcpStream, SocketAddr) tuple
        let (stream, _) = listener.accept().await?;  // stream = the connection, _ = client's address (ignored)
        
        // Wrap the tokio TcpStream with hyper's I/O adapter
        // This allows hyper to work with tokio's async I/O primitives
        let io = TokioIo::new(stream);
        
        // Spawn a new task for each connection to handle them concurrently
        // This allows the server to handle multiple requests simultaneously
        // 
        // Why 'move' is needed:
        // - The spawned task runs independently and might outlive the current loop iteration
        // - Variables like 'io' and 'handle_request' need to be moved into the task
        // - Without 'move', the task would try to borrow variables that might not exist anymore
        // 
        // What TokioExecutor does:
        // - hyper needs to know how to spawn tasks (for things like background processing)
        // - TokioExecutor tells hyper: "use tokio's runtime to spawn tasks"
        // - This connects hyper's task spawning to tokio's async runtime
        tokio::task::spawn(async move {  // move = take ownership of variables, needed because async blocks can outlive the current scope
            // Use hyper_util's auto builder to automatically detect HTTP version
            // and serve the connection with our request handler
            if let Err(err) = auto::Builder::new(hyper_util::rt::TokioExecutor::new())  // TokioExecutor = tells hyper how to spawn tasks on tokio runtime
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                // Log any errors that occur while serving the connection
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
