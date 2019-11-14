#![deny(warnings)]

fn build_url(favid: &str) -> String {
    let mut s = String::from("https://hacker-news.firebaseio.com/v0/item/");

    s.push_str(favid);
    s.push_str(".json");
    s
}

async fn get_body(url: String) -> Result<(String), reqwest::Error> {
    let res = reqwest::Client::new()
        .get(url.as_str())
        .send()
        .await
        .expect("Problem 1");

    println!("Status: {}", res.status());

    let body = res.text().await.expect("Problem 2");

    println!("Body:\n{}", body);

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = build_url("8863");

    println!("{}", url);

    let _mybody = get_body(url).await?;

    Ok(())
}
