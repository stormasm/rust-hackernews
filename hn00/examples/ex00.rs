use std::env;
use std::fs;
use std::process;
use std::string::String;

use redis::Commands;
use serde_json::{Result, Value};

fn read_file_to_string(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn string_to_json(data: String) -> Result<(Value)> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&data)?;
    Ok(v)
}

fn write_json_to_redis(json: Value) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;
    // Convert the serde value to a Vector
    let vec = json.as_array().unwrap();

    // iterate over the vector
    for i in 0..vec.len() {
        // you must convert &str to String
        let vy = &vec[i].as_str().unwrap().to_string();
        redis::cmd("SADD").arg("bill").arg(vy).execute(&mut con);
    }

    let myid = String::from("999");
    con.set("rick", myid)?;

    let k: Option<String> = con.get("rick")?;
    let k1 = k.unwrap();
    redis::cmd("SADD").arg("pete").arg(k1).execute(&mut con);

    let x55 = String::from("55");
    redis::cmd("SADD").arg("pete").arg(x55).execute(&mut con);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    let data = read_file_to_string(filename.to_string());
    let json = string_to_json(data).unwrap();
    let _x = write_json_to_redis(json);
}
