use std::error::Error;
use std::fs;


pub fn run(config: Config) -> Result<(), Box<dyn Error>>{

    let contents = fs::read_to_string(config.file_path)?;

    println!("\nWith text: \n{contents}",);

    Ok(())
}

impl Config {
    pub fn build(args : &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
    
        Ok(Config { query, file_path})
    
    }
}

pub struct  Config{
    pub query : String,
    pub file_path : String
}