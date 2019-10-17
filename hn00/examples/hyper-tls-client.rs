use hyper::Client;
use hyper_tls::HttpsConnector;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let https = HttpsConnector::new().unwrap();
    let client = Client::builder().build::<_, hyper::Body>(https);

    let res = client
        .get(
            "https://hacker-news.firebaseio.com/v0/item/8863.json"
                .parse()
                .unwrap(),
        )
        .await?;

    let mut body = res.into_body();
    while let Some(chunk) = body.next().await {
        let chunk = chunk?;
        std::io::stdout()
            .write_all(&chunk)
            .expect("example expects stdout to work");
    }
    Ok(())
}
