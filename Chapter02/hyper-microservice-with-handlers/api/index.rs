use hyper::{Method, Request, Response, StatusCode};
use hyper::body::Body;
use std::convert::Infallible;
use vercel_runtime::{run, Body as VercelBody, Error, Request as VercelRequest, Response as VercelResponse};

const INDEX: &'static str = r#"
<!doctype html>
<html>
    <head>
        <title>Rust Microservice - Hyper 1.7.0 on Vercel</title>
    </head>
    <body>
        <h3>Rust Microservice - Hyper 1.7.0 on Vercel</h3>
        <p>This microservice demonstrates:</p>
        <ul>
            <li><strong>hyper 1.7.0</strong> - HTTP library</li>
            <li><strong>Vercel Runtime</strong> - Serverless deployment</li>
            <li><strong>Route handling</strong> - Different responses for different paths</li>
        </ul>
        <p>Deployed on Vercel as a serverless function!</p>
    </body>
</html>
"#;

async fn handler(req: VercelRequest) -> Result<VercelResponse<VercelBody>, Error> {
    let path = req.uri().path();
    
    match (req.method(), path) {
        (&Method::GET, "/") => {
            let response = Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "text/html")
                .body(VercelBody::from(INDEX))
                .unwrap();
            
            Ok(response.map(VercelBody::from))
        },
        _ => {
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(VercelBody::from("404 Not Found"))
                .unwrap();
            
            Ok(response.map(VercelBody::from))
        },
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
} 