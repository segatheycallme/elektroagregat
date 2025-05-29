use std::error::Error;

use elektroagregat::{ElectronicPart, ScrapedSite};
use reqwest::{Client, Url};
use scraper::{ElementRef, Selector};
use thiserror::Error;

const BASE_URL: &str = "https://mgelectronic.rs/search";

pub const SITE_INFO: ScrapedSite = ScrapedSite {
    name: "MGElectronic",
    url: "https://www.mgelectronic.rs",
    color: "#b11715",
};

#[derive(Debug, Error)]
pub enum MGError {
    #[error("Couldn't find main table")]
    NoTable,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct MGElectronicProduct {
    name: String,
    price: f64,
    stock: bool,
    product_url: String,
    image_url: Option<String>,
    datasheet_url: Option<String>,
    code: String,
    characteristics: String,
    housing: String,
    manufacturer: Option<String>,
    manufacturer_code: Option<String>,
}

impl From<MGElectronicProduct> for ElectronicPart {
    fn from(val: MGElectronicProduct) -> Self {
        let mut description = format!(
            "Characteristics: {}\nHousing: {}",
            val.characteristics, val.housing
        );
        if let Some(ref manafacturer) = val.manufacturer {
            description += &format!("\nManafacturer: {}", manafacturer);
        }
        if let Some(ref manafacturer_code) = val.manufacturer_code {
            description += &format!("\nManafacturer code: {}", manafacturer_code);
        }
        ElectronicPart {
            name: val.name,
            price: val.price,
            stock: val.stock,
            product_url: val.product_url,
            image_url: val.image_url,
            description,
        }
    }
}

pub async fn simple_search(
    query: String,
    client: &Client,
) -> Result<Vec<MGElectronicProduct>, Box<dyn Error>> {
    let url = Url::parse_with_params(BASE_URL, [("q", query)])?;
    let body = client.get(url.to_string()).send().await?.text().await?;
    let document = scraper::html::Html::parse_document(&body);
    let mut rows = document
        .select(&Selector::parse(".search-results table tbody").unwrap())
        .next()
        .ok_or(MGError::NoTable)?
        .child_elements();
    rows.next(); // skip the header row

    Ok(rows
        .filter_map(|row| {
            let inner = parse_row(row)?;
            Some(MGElectronicProduct {
                product_url: url.join(&inner.product_url).unwrap().to_string(),
                datasheet_url: inner
                    .datasheet_url
                    .map(|str| url.join(&str).unwrap().to_string()),
                ..inner
            })
        })
        .collect())
}

fn parse_row(row: ElementRef) -> Option<MGElectronicProduct> {
    let mut td_iter = row.child_elements();
    let image_url = td_iter
        .next()
        .and_then(|el| {
            el.select(&Selector::parse("img").unwrap())
                .next()?
                .attr("src")
        })
        .map(|str| str.to_string());

    let description = td_iter.next()?;
    let (name, product_url) = description
        .select(&Selector::parse("h4 a").unwrap())
        .next()
        .and_then(|el| {
            Some((
                el.inner_html().trim().to_string(),
                el.attr("href")?.to_string(),
            ))
        })?;
    let dd_selector = Selector::parse(".product-spec-list dd").unwrap();
    let mut dd_iter = description
        .select(&dd_selector)
        .map(|el| el.text().collect::<String>().trim().to_string()); // ugh
    let code = dd_iter.next()?;
    let characteristics = dd_iter.next()?;
    let housing = dd_iter.next()?;
    let manufacturer = dd_iter.next();
    let manufacturer_code = dd_iter.next();

    let datasheet_url = td_iter
        .next()
        .and_then(|el| {
            el.select(&Selector::parse("a").unwrap())
                .next()?
                .attr("href")
        })
        .map(|str| str.to_string());

    let stock = &td_iter
        .next()?
        .select(&Selector::parse("span").unwrap())
        .next()?
        .inner_html()
        == "dostupno"; // TODO: only works for serbian

    td_iter.next();
    let price: f64 = td_iter
        .next()?
        .select(&Selector::parse("span").unwrap())
        .next()?
        .inner_html()
        .replace(',', ".")
        .parse()
        .ok()?;

    Some(MGElectronicProduct {
        name,
        manufacturer,
        manufacturer_code,
        image_url,
        product_url,
        characteristics,
        code,
        housing,
        price,
        stock,
        datasheet_url,
    })
}
