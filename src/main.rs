use askama::Template;
use askama_web::WebTemplate;
use axum::{
    Router,
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    serve,
};
use futures::future::join_all;
use itertools::Itertools;
use reqwest::Client;
use reqwest::ClientBuilder;
use scraping::{AVALAIBLE_SITES, AvalaibleSite, ElectronicPart};
use std::{collections::HashMap, error::Error, sync::Arc};
use tower_http::compression::CompressionLayer;

mod scraping;

struct AppState {
    client: Client,
    available_sites: HashMap<String, AvalaibleSite>,
}

// https://www.reddit.com/r/htmx/comments/1d6m1f2/comment/l6w06vv/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut available_sites = HashMap::new();
    for site in AVALAIBLE_SITES {
        available_sites.insert(site.get_key().to_string(), site);
    }
    let state = AppState {
        client: ClientBuilder::new().brotli(true).build()?,
        available_sites,
    };

    let app = Router::new()
        .route("/", get(landing))
        .route("/search", get(search))
        .layer(CompressionLayer::new().br(true))
        .with_state(Arc::new(state));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    serve(listener, app).await?;

    Ok(())
}

#[derive(Template, WebTemplate)]
#[template(path = "search.html")]
struct SearchResults {
    maybe_products: Result<Vec<ElectronicPart>, String>,
}

async fn search(
    Query(search_options): Query<HashMap<String, String>>,
    State(app_state): State<Arc<AppState>>,
) -> impl IntoResponse {
    let query = search_options
        .get("query")
        .map(|x| x.to_string())
        .unwrap_or_default();
    if query.len() < 3 {
        return SearchResults {
            maybe_products: Err("Query string must be longer than 3 characters".to_string()),
        };
    }

    let mut selected_sites = vec![];
    for site in &app_state.available_sites {
        if search_options.get(site.0).is_some_and(|x| x == "on") {
            selected_sites.push(site.1);
        }
    }

    let order_by = search_options
        .get("order_by")
        .map(|x| x.as_str())
        .unwrap_or_default();

    let maybe_products = get_products(query, &app_state.client, &selected_sites)
        .await
        .map(|mut products| {
            match order_by {
                "name_asc" => products.sort_by(|a, b| a.name.cmp(&b.name)),
                "name_desc" => products.sort_by(|a, b| b.name.cmp(&a.name)),
                "price_asc" => products.sort_by(|a, b| a.price.total_cmp(&b.price)),
                "price_desc" => products.sort_by(|a, b| b.price.total_cmp(&a.price)),
                _ => products.sort_by_key(|x| x.stock),
            }
            products.sort_by_key(|a| !a.stock);
            products
        });
    SearchResults { maybe_products }
}

async fn get_products(
    query: String,
    client: &Client,
    avalaible_sites: &[&AvalaibleSite],
) -> Result<Vec<ElectronicPart>, String> {
    let mut pending_site_searches = vec![];
    for site in avalaible_sites {
        pending_site_searches.push(async {
            site.simple_search(query.clone(), client)
                .await
                .unwrap_or_else(|err| {
                    eprintln!("An error has occured: {err}");
                    vec![] // TODO: workaround
                })
            // .map_err(|err| format!("An error has occured: {err}"))
        });
    }

    Ok(join_all(pending_site_searches)
        .await
        .into_iter()
        .flat_map(|x| x.into_iter())
        .collect_vec())
}

#[derive(Template, WebTemplate)]
#[template(path = "landing.html")]
struct LandingPage {
    title: String,
    navbar: Navbar,
    sites: Vec<AvalaibleSite>,
    products: Option<SearchResults>,
}

#[derive(Template)]
#[template(path = "navbar.html")]
enum Navbar {
    Standard,
}

async fn landing() -> impl IntoResponse {
    LandingPage {
        title: "caooo".to_string(),
        navbar: Navbar::Standard,
        sites: AVALAIBLE_SITES.into(),
        products: None,
    }
}
