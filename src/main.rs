use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In file '{}'", config.search_path);

    let contents = fs::read_to_string(config.search_path)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    search_path: String,
}

impl Config {
    fn new(args:&[String])->Config {
        let query = &args[1];
        let path = &args[2];
    
        Config {
            query: query.clone(),
            search_path: path.clone(),
        }
    }
}

