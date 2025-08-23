use hyper::{Response, body::Bytes};
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

async fn handle_request(_req: hyper::Request<hyper::body::Incoming>) -> Result<Response<String>, Infallible> {
    Ok(Response::new("Rust Microservice with Hyper 1.7.0".to_string()))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    
    let listener = TcpListener::bind(addr).await?;
    println!("Server running on http://127.0.0.1:8080");
    
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        
        tokio::task::spawn(async move {
            if let Err(err) = auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
