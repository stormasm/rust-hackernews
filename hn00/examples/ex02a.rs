#![deny(warnings)]

fn build_url(favid: &str) -> String {
    let mut s = String::from("https://hacker-news.firebaseio.com/v0/item/");

    s.push_str(favid);
    s.push_str(".json");
    s
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = build_url("8863");

    println!("{}", url);

    let res = reqwest::Client::new().get(url.as_str()).send().await?;

    println!("Status: {}", res.status());

    let body = res.text().await?;

    println!("Body:\n{}", body);

    Ok(())
}
