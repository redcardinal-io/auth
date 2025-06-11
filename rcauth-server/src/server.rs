use axum::Router;

pub async fn build_http_server(host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("{}:{}", host, port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;

    let app = Router::new();
    let app = app.nest("/api", crate::api::routes());

    axum::serve(listener, app).await?;
    Ok(())
}
