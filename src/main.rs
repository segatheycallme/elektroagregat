use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Router,
    // extract::{Query, State},
    response::IntoResponse,
    routing::get,
    serve,
};
// use elektroagregat::ElectronicPart;
use reqwest::ClientBuilder;
// use reqwest::Client;
// use serde::Deserialize;
use std::error::Error;
use tower_http::compression::CompressionLayer;

// https://www.reddit.com/r/htmx/comments/1d6m1f2/comment/l6w06vv/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(landing))
        // .route("/search", get(search))
        .layer(CompressionLayer::new().br(true))
        .with_state(ClientBuilder::new().brotli(true).build()?);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    serve(listener, app).await?;

    Ok(())
}

// #[derive(Debug, Deserialize)]
// struct SearchOptions {
//     query: String,
// }
//
// async fn search(
//     Query(search_options): Query<SearchOptions>,
//     State(client): State<Client>,
// ) -> impl IntoResponse {
//     "search"
// }

#[derive(Template, WebTemplate)]
#[template(path = "landing.html")]
struct LandingPage {
    title: String,
    navbar: Navbar,
}

#[derive(Template, WebTemplate)]
#[template(path = "navbar.html")]
struct Navbar {}

async fn landing() -> impl IntoResponse {
    LandingPage {
        title: "caooo".to_string(),
        navbar: Navbar {},
    }
}
