use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let search_path = &args[2];

    println!("Searching for {}", query);
    println!("In file '{}'", search_path);

    let contents = fs::read_to_string(search_path)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
