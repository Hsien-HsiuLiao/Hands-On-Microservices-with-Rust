use std::fmt;
use std::sync::{Arc, Mutex};
use slab::Slab;
use hyper::{Method, Request, Response, StatusCode};
use hyper::service::service_fn;
use hyper_util::rt::TokioIo;
use hyper_util::server::conn::auto;
use std::convert::Infallible;
use std::net::SocketAddr;
use tokio::net::TcpListener;

const INDEX: &str = r#"
<!doctype html>
<html>
    <head>
        <title>Rust Microservice</title>
    </head>
    <body>
        <h3>Rust Microservice - REST</h3>
    </body>
</html>
"#;

type UserId = u64;

struct UserData;

impl fmt::Display for UserData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("{}")
    }
}

type UserDb = Arc<Mutex<Slab<UserData>>>;

const USER_PATH: &str = "/user/";

async fn microservice_handler(req: Request<hyper::body::Incoming>, user_db: &UserDb) -> Result<Response<String>, Infallible> {
    let response = match (req.method(), req.uri().path()) {
        (&Method::GET, "/") => {
            Response::new(INDEX.to_string())
        },
        (method, path) if path.starts_with(USER_PATH) => {
            let user_id = path.trim_start_matches(USER_PATH)
                .parse::<UserId>()
                .ok()
                .map(|x| x as usize);
            let mut users = user_db.lock().unwrap();
            match (method, user_id) {
                (&Method::GET, Some(id)) => {
                    if let Some(data) = users.get(id) {
                        Response::new(data.to_string())
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                },
                (&Method::POST, None) => {
                    let id = users.insert(UserData);
                    Response::new(id.to_string())
                },
                (&Method::POST, Some(_)) => {
                    response_with_code(StatusCode::BAD_REQUEST)
                },
                (&Method::PUT, Some(id)) => {
                    if let Some(user) = users.get_mut(id) {
                        *user = UserData;
                        response_with_code(StatusCode::OK)
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                },
                (&Method::DELETE, Some(id)) => {
                    if users.contains(id) {
                        users.remove(id);
                        response_with_code(StatusCode::OK)
                    } else {
                        response_with_code(StatusCode::NOT_FOUND)
                    }
                },
                _ => {
                    response_with_code(StatusCode::METHOD_NOT_ALLOWED)
                },
            }
        },
        _ => {
            response_with_code(StatusCode::NOT_FOUND)
        },
    };
    Ok(response)
}

fn response_with_code(status_code: StatusCode) -> Response<String> {
    Response::builder()
        .status(status_code)
        .body("".to_string())
        .unwrap()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let user_db = Arc::new(Mutex::new(Slab::new()));
    
    let listener = TcpListener::bind(addr).await?;
    println!("REST Microservice running on http://127.0.0.1:8080");
    
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        let user_db = user_db.clone();
        
        tokio::task::spawn(async move {
            if let Err(err) = auto::Builder::new(hyper_util::rt::TokioExecutor::new())
                .serve_connection(io, service_fn(move |req| {
                    let user_db = user_db.clone();
                    async move { microservice_handler(req, &user_db).await }
                }))
                .await
            {
                eprintln!("Error serving connection: {:?}", err);
            }
        });
    }
}
