// documentation for the src/lib.rs instead of anything which goes after the comment
//! # minigrep
//!
//! `minigrep-vielenkz` is a lightweight analogue of `grep` to find a match in a file

use std::{env, fs};
use std::error::Error;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    // if we return Result<Something, str> - str is always static
    /// Build [Config] from arguments provided.
    /// The function is expected arguments:
    /// 1. Name of the program
    /// 2. Query to find in file
    /// 3. Filename to search in
    ///
    /// If IGNORE_CASE flag is present in environment the search ignore case will be applied
    ///
    /// # Exception
    /// In case the arguments are not provided the [Err] with string is returned
    pub fn build(mut args: impl Iterator<Item=String>) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// dyn - dynamic, all types which implemented trait Error
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

/// Search for all lines in contents string containing query.
/// Case sensitive.
///
/// # Examples
///
/// ```rust
/// use minigrep_vielenkz::search;
///
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct tape.";
///
/// assert_eq!(vec!["safe, fast, productive."], search(query, contents));
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|l| l.contains(query))
        .collect()
}

/// Search for all lines in contents string containing query.
/// Case insensitive.
///
/// # Examples
///
/// ```rust
/// use minigrep_vielenkz::search_case_insensitive;
///
/// let query = "rUsT";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Trust me.";
///
/// assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|l| l.to_lowercase().contains(&query.to_string()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], search_case_insensitive(query, contents));
    }
}
