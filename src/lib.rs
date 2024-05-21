use std::{env, error::Error, fs};

pub struct Config {
    query: String,
    search_path: String,
    ignore_case: bool,
}

impl Config {
    pub fn build(args:&[String])->Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = &args[1];
        let path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();
    
       Ok( 
            Config {
                query: query.to_string(),
                search_path: path.to_string(),
                ignore_case,
            }
        )
    }
}

pub fn run(config:Config)->Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.search_path)?;

    let search = if config.ignore_case {
        search_case_insensitive
    } else {
        search
    };

    for line in search(&config.query, &contents) {
        println!("{}", line)
    }

    Ok(())
}

pub fn search<'a>(query:&str, contents: &'a str)->Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(query){
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query:&str, contents:&'a str)->Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }
    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn search_sensetive(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."],search(query, contents));
    }
    #[test]
    fn search_insensetive(){
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["Rust:"],search_case_insensitive(query, contents));
    }
}