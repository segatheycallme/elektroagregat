use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    serve,
};
use elektroagregat::{ElectronicPart, ScrapedSite};
use mgelectronic::MGElectronicProduct;
use mikroprinc::MikroPrincProduct;
use reqwest::Client;
use reqwest::ClientBuilder;
use serde::Deserialize;
use std::error::Error;
use tower_http::compression::CompressionLayer;

mod mgelectronic;
mod mikroprinc;

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

#[derive(Debug, Deserialize)]
struct SearchOptions {
    query: String,
}

// #[derive(Template, WebTemplate)]
// #[template(path = "search.html")]
// struct SearchResults {
//     title: String,
//     navbar: Navbar,
//     products: Vec<impl ElectronicPart>,
// }
//
// async fn search(
//     Query(search_options): Query<SearchOptions>,
//     State(client): State<Client>,
// ) -> impl IntoResponse {
//     let mg_products = MGElectronicProduct::simple_search(search_options.query, &client)
//         .await
//         .unwrap();
//     ""
// }

#[derive(Template, WebTemplate)]
#[template(path = "landing.html")]
struct LandingPage {
    title: String,
    navbar: Navbar,
    sites: Vec<ScrapedSite>,
}

#[derive(Template)]
#[template(path = "navbar.html")]
enum Navbar {
    Standard,
    #[allow(dead_code)]
    None,
}

async fn landing() -> impl IntoResponse {
    LandingPage {
        title: "caooo".to_string(),
        navbar: Navbar::Standard,
        sites: [MGElectronicProduct::SITE_INFO, MikroPrincProduct::SITE_INFO].into(),
    }
}
