// This is an exact copy of:
// https://github.com/seanmonstar/reqwest/blob/master/examples/simple.rs
//

#![deny(warnings)]

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let res = reqwest::Client::new()
        .get("https://hacker-news.firebaseio.com/v0/item/8863.json")
        .send()
        .await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n\n{}", body);

    Ok(())
}
