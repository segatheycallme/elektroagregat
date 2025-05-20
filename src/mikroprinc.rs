use std::error::Error;

use itertools::Itertools;
use reqwest::{Client, Url};
use scraper::{ElementRef, Selector};
use thiserror::Error;

const BASE_URL: &str = "https://www.mikroprinc.com/sr/pretraga";

#[derive(Debug, Error)]
pub enum MikroPrinc {
    #[error("Couldn't find main table")]
    NoTable,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MikroPrincProduct {
    name: String,
    price: f64,
    stock: bool,
    product_url: String,
    image_url: Option<String>,
    description: String,
}

pub async fn simple_search(
    search: &str,
    client: &Client,
) -> Result<Vec<MikroPrincProduct>, Box<dyn Error>> {
    let url = Url::parse_with_params(BASE_URL, [("phrase", search)])?;
    let body = client.get(url.to_string()).send().await?.text().await?;
    let document = scraper::html::Html::parse_document(&body);
    let rows = document
        .select(&Selector::parse(".products-table table tbody").unwrap())
        .next()
        .ok_or(MikroPrinc::NoTable)?
        .child_elements();

    Ok(rows.map(|row| parse_row(row).unwrap()).collect())
}

fn parse_row(row: ElementRef) -> Option<MikroPrincProduct> {
    let mut td_iter = row.child_elements();
    let image_url = td_iter.next().and_then(|el| {
        Some(
            el.select(&Selector::parse("img").unwrap())
                .next()?
                .attr("src")?
                .to_string(),
        )
    });

    let text_block = td_iter.next()?;
    let (name, product_url) = text_block
        .select(&Selector::parse("a").unwrap())
        .next()
        .and_then(|el| {
            Some((
                el.inner_html().trim().to_string(),
                el.attr("href")?.to_string(),
            ))
        })?;
    let description = text_block
        .select(&Selector::parse(".description").unwrap())
        .next()?
        .inner_html()
        .trim()
        .to_string();

    let price: f64 = td_iter
        .next()?
        .select(&Selector::parse(".price").unwrap())
        .next()?
        .text()
        .filter(|str| !str.trim().is_empty())
        .take(2)
        .map(|str| str.replace(',', "").replace(".", ""))
        .join(".")
        .trim()
        .parse()
        .unwrap();

    let stock = td_iter
        .next()?
        .select(&Selector::parse("p").unwrap())
        .next()?
        .inner_html()
        == "Dostupan"; // TODO: serbian pt2

    Some(MikroPrincProduct {
        image_url,
        name,
        price,
        stock,
        product_url,
        description,
    })
}
