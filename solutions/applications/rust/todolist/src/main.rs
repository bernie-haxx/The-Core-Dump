use std::fs;

use serde_json::from_reader;

fn main() {
    let file = fs::File::open("list.json").expect("Expected a file found nothing.");

    let contents: serde_json::Value = from_reader(file).expect("Expected valid JSON");

    let title = contents.get("status");

    println!("{title:?}")
}
