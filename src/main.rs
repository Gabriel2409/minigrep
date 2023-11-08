use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (query, file_path) = parse_config(&args);

    println!("Searching for {query}");
    println!("{file_path}");

    let contents = fs::read_to_string(file_path).expect("Fail to read file");

    println!("Contents are {contents}");
}

fn parse_config(args: &[String]) -> (String, String) {
    let query = args[1].clone();
    let file_path = args[2].clone();
    (query, file_path)
}
