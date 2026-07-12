use std::env;
use std::fs;
use std::process;
use std::error::Error;
use minigrep;
fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });
   
    match run(&config) {
        Ok(contents) => {
            println!("Searching for {}", config.query);
            println!("In file : {}", config.file_path);
        }
        Err(e) => {
            eprintln!("Application Error: {e}");
            process::exit(1);
        }
    }


    dbg!(args);
}

fn run(config: &Config) -> Result<String, Box<dyn Error>> {
    let contents =  fs::read_to_string(&config.file_path)?;
    let results = if config.ignore_case {
        minigrep::search_case_insensitive(&config.query, &contents)
    } else {
        minigrep::search(&config.query, &contents)
    };
    for line in results {
        println!("{line}");
    } 
    Ok(contents)
}

struct Config {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Config {
    fn build(args: &Vec<String>) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough args");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok (Config {
                query,
                file_path,
                ignore_case,
            })
    }
}