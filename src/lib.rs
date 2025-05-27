// use std::error::Error;

pub struct ScrapedSite {
    pub name: &'static str,
    pub url: &'static str,
    pub color: &'static str,
}

pub trait ElectronicPart {
    // const SITE_INFO: ScrapedSite;
    fn name(&self) -> &str;
    fn price(&self) -> f64;
    fn stock(&self) -> bool;
    fn product_url(&self) -> &str;
    fn image_url(&self) -> Option<&str>;
    fn description(&self) -> String;
    // fn simple_search(
    //     query: String,
    //     client: &reqwest::Client,
    // ) -> impl Future<Output = Result<Vec<Self>, Box<dyn Error>>>;
}
