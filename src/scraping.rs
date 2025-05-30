// use std::error::Error;

use std::error::Error;

mod mgelectronic;
mod mikroprinc;

use reqwest::Client;

pub enum AvalaibleSite {
    MGElectronic,
    MikroPrinc,
}

impl AvalaibleSite {
    pub fn get_name(&self) -> &'static str {
        match self {
            Self::MGElectronic => mgelectronic::NAME,
            Self::MikroPrinc => mikroprinc::NAME,
        }
    }

    pub fn get_url(&self) -> &'static str {
        match self {
            Self::MGElectronic => mgelectronic::URL,
            Self::MikroPrinc => mikroprinc::URL,
        }
    }

    pub fn get_color(&self) -> &'static str {
        match self {
            Self::MGElectronic => mgelectronic::COLOR,
            Self::MikroPrinc => mikroprinc::COLOR,
        }
    }

    pub async fn simple_search(
        &self,
        query: String,
        client: &Client,
    ) -> Result<Vec<ElectronicPart>, Box<dyn Error>> {
        match self {
            Self::MGElectronic => mgelectronic::simple_search(query, client).await,
            Self::MikroPrinc => mikroprinc::simple_search(query, client).await,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ElectronicPart {
    pub name: String,
    pub price: f64,
    pub stock: bool,
    pub product_url: String,
    pub image_url: Option<String>,
    pub description: String,
    pub color: String,
}
