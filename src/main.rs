use std::env;
use std::fs;
use std::process;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Error when building config: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!(" In {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("Fail to read file");

    println!("Contents are {contents}");
}
