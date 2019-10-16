extern crate redis;
use redis::Commands;

#[tokio::main]
async fn gorun() -> Result<(), reqwest::Error> {
    let res = reqwest::Client::new()
        .get("https://hacker-news.firebaseio.com/v0/item/8863.json")
        .send()
        .await?;

    let body = res.text().await?;

    // connect to redis
    let client = redis::Client::open("redis://127.0.0.1/").expect("No");
    // let mut con = client.get_connection()?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");

    let _: () = con.hset("hm-hnfav", "8863", body).expect("No 2");

    Ok(())
}

fn main() {
    let _ = gorun();
}
