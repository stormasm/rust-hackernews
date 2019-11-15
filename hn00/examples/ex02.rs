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

fn write_redis(body: String, id: &str) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");

    let _x3 = redis::cmd("HSET")
        .arg("hnfav11-14")
        .arg(id)
        .arg(body)
        .query::<u64>(&mut con)
        .unwrap();

    Ok(())
}

async fn gorun(id: &str) -> Result<(), reqwest::Error> {
    let url = build_url(id);
    println!("{}", url);
    let body = get_body(url).await?;
    let _ = write_redis(body, id);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let ids = [
        "8863", "18987885", "18986544", "18987102", "18972892", "18973008",
    ];

    for id in ids.iter() {
        let _ = gorun(id).await?;
    }
    Ok(())
}
