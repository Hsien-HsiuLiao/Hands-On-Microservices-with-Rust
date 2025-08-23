use hyper::{Method, Request, Response, StatusCode};
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

const INDEX: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Hyper 1.7.0 Microservice with Handlers</title>
    </head>
    <body>
        <h3>Rust Microservice - Hyper 1.7.0 Microservice with Handlers</h3>
        <p>This microservice demonstrates:</p>
        <ul>
            <li><strong>hyper 1.7.0</strong> - HTTP library</li>
            <li><strong>hyper-util 0.1</strong> - Server utilities</li>
            <li><strong>tokio 1.0</strong> - Async runtime</li>
            <li><strong>Route handling</strong> - Different responses for different paths</li>
        </ul>
        <p>Try visiting <code>/</code> for this page or any other path for a 404 response.</p>
    </body>
</html>
"#;

async fn microservice_handler(req: Request<hyper::body::Incoming>) -> Result<Response<String>, Infallible> {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Ok(Response::new(INDEX.to_string()))
        },
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("404 Not Found".to_string())
                .unwrap();
            Ok(response)
        },
    }
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
                .serve_connection(io, service_fn(microservice_handler))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
