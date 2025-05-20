use reqwest::Client;

// mod mgelectronic;
mod mikroprinc;

#[tokio::main]
async fn main() {
    let client = Client::new();
    // dbg!(
    //     mgelectronic::simple_search("SPREJ 100 ML PCB ČISTAČ", &client)
    //         .await
    //         .unwrap()
    // );
    dbg!(mikroprinc::simple_search("1086", &client).await.unwrap());
}
