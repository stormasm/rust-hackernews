use std::env;
use std::fs;
use std::process;
use std::string::String;

use serde_json::{Result, Value};

fn read_file_to_string(filename: String) -> String {
    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");
    contents
}

fn string_to_json(data: String) -> Result<()> {
    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(&data)?;

    println!("{}", v[0]);

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
    println!("With text:\n{}", data);

    string_to_json(data);
}
