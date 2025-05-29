// use std::error::Error;

pub struct ScrapedSite {
    pub name: &'static str,
    pub url: &'static str,
    pub color: &'static str,
}

pub struct ElectronicPart {
    pub name: String,
    pub price: f64,
    pub stock: bool,
    pub product_url: String,
    pub image_url: Option<String>,
    pub description: String,
}
