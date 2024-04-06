use std::error::Error;
use std::fs;

pub struct Config<'a> {
    pub query: &'a String,
    pub file_path: &'a String,
}

impl<'a> Config<'a> {
    // if we return Result<Something, str> - str is always static
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }
        let query = &args[1];
        let file_path = &args[2];
        Ok(Config { query, file_path })
    }
}

// dyn - dynamic, all types which implemented trait Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}

