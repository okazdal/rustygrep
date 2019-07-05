use std::env;
use std::process;
use rustygrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
//    println!("{:?}", args);

//    let query = &args[1];
//    let filename = &args[2];

//    let config = parse_config(&args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

//    println!("Searching for {}", query);
//    println!("In file: {} ", filename);

//    let contents = fs::read_to_string(config.filename)
//        .expect("Error reading file");

//    println!("Text: \n{}", contents);

//    run(config);

    if let Err(e) = rustygrep::run(config) {
        println!("App error: {}", e);

        process::exit(1);

    }

}




//fn parse_config(args: &[String]) -> Config {
//    let query = args[1].clone();
//    let filename = args[2].clone();
//
//    Config{query, filename}
//}