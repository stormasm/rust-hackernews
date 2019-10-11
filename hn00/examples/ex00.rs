use std::env;
use std::fs;
use std::process;
use std::string::String;

use redis::Commands;
use serde_json::{Result, Value};
//use serde_json::Value;

fn read_file_to_string(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn string_to_json(data: String) -> Result<(Value)> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&data)?;

    // println!("{}", v[0]);

    Ok(v)
}

fn write_json_to_redis(json: Value) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    let mut con = client.get_connection()?;

    println!("{}", json);

    // let myid = serde_json::from_str::json[0];

    // let myid: String = json[0].to_string();

    // Convert the serde value to a Vector
    let vec = json.as_array().unwrap();


    // iterate over the vector
    for i in 0..vec.len() {
        //let vx = &vec[i].as_str().unwrap();
        //println!("{}", vx);
        let vy = &vec[i].as_str().unwrap().to_string();
        // let () = vy;
        redis::cmd("SADD").arg("bill").arg(vy).execute(&mut con);
    }

    let myid = String::from("999");

    // let () = myid;

    con.set("rick", myid)?;

    let k: Option<String> = con.get("rick")?;
    let k1 = k.unwrap();

    println!("No !! {}", k1);

    redis::cmd("SADD").arg("bill").arg(k1).execute(&mut con);

    let jim = String::from("55");

    redis::cmd("SADD").arg("bill").arg(jim).execute(&mut con);

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("In file {}", filename);

    let data = read_file_to_string(filename.to_string());
    // println!("With text:\n{}", data);

    let json = string_to_json(data).unwrap();
    // println!("{}", json[0]);

    let _x = write_json_to_redis(json);
}
