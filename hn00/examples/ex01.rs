
use std::env;
//use std::fs;
use std::process;
use std::string::String;

use redis::Commands;

use serde_json::{Value};
use serde_json;



use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};


fn read_file_to_string(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn string_to_json(data: String) -> serde_json::Result<(Value)> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&data)?;
    Ok(v)
}

fn write_json_to_redis(json: Value) -> redis::RedisResult<()> {
    let client = redis::Client::open("redis://127.0.0.1/")?;
    // let mut con = client.get_connection()?;
    let mut con = client.get_connection().expect("Failed to connect to Redis");
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





fn dir_reader(mydir: String) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    let mypath = Path::new(&mydir);
    println!("Entries in {:?}:", mypath);

    let mut vec: Vec<PathBuf> = Vec::new();

    for entry in fs::read_dir(mypath)? {
        let entry = entry?;
        let path = entry.path();
        //println!("{:?}", path.file_name().ok_or("No filename")?);
        vec.push(path);
    }

    Ok(vec)
}

fn processor(mydir: String) -> Result<(), Box<dyn Error>> {
    let vec = dir_reader(mydir).unwrap();
    // println!("vec len = {:?}", vec.len());

    for name in vec {
        println!("{:?}", name.file_name().ok_or("No filename")?);
    }

    Ok(())
}

fn main() {
    let mydir = String::from("/tmp09/rust-hackernews/hn00/data/in");
    let _ = processor(mydir);
}
