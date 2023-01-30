use serde_json::from_reader;
use std::fs::File;
use std::io::BufReader;
use serde_json::Value;
//use reqwest::get;

fn main() {
    let name = "Rust with containers";
    println!("Hello, {}!", name);
}

// write a function to make an api call with reqwest
// Path: src/main.rs
async fn make_api_call() {
    // let url = "https://jsonplaceholder.typicode.com/todos/1";
    // let response = get(url).await.unwrap();
    // let json: Value = response.json().await.unwrap();
    // println!("JSON:\n{:#?}", json);
}


// write a function to read from a json file
// Path: src/main.rs
fn read_from_json_file() {
    let file = File::open("data.json").unwrap();
    let reader = BufReader::new(file);
    let json: Value = from_reader(reader).unwrap();
    println!("JSON:\n{:#?}", json);
}
