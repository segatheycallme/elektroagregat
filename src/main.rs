use elektroagregat::ElectronicPart;
use reqwest::Client;

mod mgelectronic;
mod mikroprinc;

// https://www.reddit.com/r/htmx/comments/1d6m1f2/comment/l6w06vv/?utm_source=share&utm_medium=web3x&utm_name=web3xcss&utm_term=1&utm_content=share_button
#[tokio::main]
async fn main() {
    let client = Client::new();
    // dbg!(
    //     mgelectronic::simple_search("SPREJ 100 ML PCB ČISTAČ", &client)
    //         .await
    //         .unwrap()
    // );
    mikroprinc::MikroPrincProduct::simple_search("1086".to_string(), &client)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| println!("{}", x.description()));
    mgelectronic::MGElectronicProduct::simple_search("1086".to_string(), &client)
        .await
        .unwrap()
        .into_iter()
        .for_each(|x| println!("{}", x.description()));
}
