use actix_web::http::header::HeaderValue;
use actix_web::{get, App, Error, HttpRequest, HttpResponse, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    let server = HttpServer::new(|| {
        App::new()
            .service(actix_files::Files::new("/dist", "frontend/dist"))
            .service(render_yew_app)
    })
    .bind(("0.0.0.0", 7000))?
    .run();

    server.await
}

#[get("/{tail:.*}")]
async fn render_yew_app(req: HttpRequest) -> Result<HttpResponse, Error> {
    let index_html_s = tokio::fs::read_to_string("frontend/dist/index.html")
        .await
        .expect("failed to read index.html");

    let uri = req.uri().to_string();

    let content = frontend::render_app(uri).await;

    Ok(HttpResponse::Ok()
        .content_type(HeaderValue::from_static("text/html; charset=utf-8"))
        .body(index_html_s.replace("<body>", &format!("<body>{content}"))))
}
