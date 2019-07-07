use std::fs;
use std::error::Error;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
//    println!("Text: \n{}", contents);

    for line in search(&config.query, &contents) {
        println!("{}", line);

    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {

//        if args.len() < 3 {
//            return Err("not enough arguments")
//        }
//        let query = args[1].clone();
//        let filename = args[2].clone();
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Noi query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("No filename string"),
        };

        Ok(Config{ query, filename })
    }
}

pub fn search<'a> (query: &str, contents: &'a str) -> Vec<&'a str> {
//    let mut results = Vec::new();

//    for line in contents.lines() {
//        if line.contains(query) {
//            results.push(line);
//
//        }
//    }
//    results
    contents.lines()
        .filter(|line| line.contains(query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}

