use std::{error::Error, fs};

pub struct Config {
    query: String,
    search_path: String,
}

impl Config {
    pub fn build(args:&[String])->Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let path = &args[2];
    
       Ok( 
            Config {
                query: query.clone(),
                search_path: path.clone(),
            }
        )
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
        
    println!("Searching for {}", config.query);
    println!("In file '{}'", config.search_path);
    
    let contents = fs::read_to_string(config.search_path)?;

    println!("With text:\n{}", contents);

    Ok(())
}