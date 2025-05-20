use mgelectronic::simple_search;
use reqwest::Client;

mod mgelectronic;

#[tokio::main]
async fn main() {
    let client = Client::new();
    dbg!(
        simple_search("SPREJ 100 ML PCB ČISTAČ", &client)
            .await
            .unwrap()
    );
}
