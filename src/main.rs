use axum::{Router, routing::get, http::header};
use tower_http::services::{ServeDir, ServeFile};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let catalog = std::fs::read_to_string("data/club.json")?;
    let app = Router::new()
        .route("/api/club", get(move || async move { ([(header::CONTENT_TYPE, "application/json")], catalog) }))
        .route("/api/health", get(|| async { "ok" }))
        .fallback_service(ServeDir::new("public").not_found_service(ServeFile::new("public/index.html")));
    let address = std::env::var("BIND_ADDR").unwrap_or_else(|_| "0.0.0.0:8080".into());
    let listener = tokio::net::TcpListener::bind(&address).await?;
    println!("Bookish listening on http://{}", listener.local_addr()?);
    axum::serve(listener, app).with_graceful_shutdown(async { let _ = tokio::signal::ctrl_c().await; }).await?;
    Ok(())
}
