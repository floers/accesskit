use anyhow::anyhow;
use axum::Router;
use tower_http::services::ServeDir;

mod tests;

#[tokio::main]
async fn main() {
    let app = Router::new().fallback_service(ServeDir::new("web"));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    if std::env::var("TEST").is_ok() {
        let test = tokio::spawn(async { tests::run().await });
        let serve = axum::serve(listener, app);
        let next = tokio::select! {
            r = test => match r {
                Ok(r) => r,
                Err(e) => Err(anyhow!(e)),
            },
            r = serve => r.map(|_| Default::default()).map_err(|e|anyhow!(e)),
        };

        match next {
            Ok(s) => {
                s.finish();
            }
            Err(e) => {
                eprintln!("\n  Failed: {e}");
                std::process::exit(-1);
            }
        }
    } else {
        println!("go to http://localhost:3000");
        axum::serve(listener, app).await.unwrap();
    }
}
