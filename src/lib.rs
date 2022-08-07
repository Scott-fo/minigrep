use std::fs;
use std::env;
use std::error::Error;
/// # Structure for the env args passed in
/// ```
/// Structure defines the expected keys and types we expect to be passed into this program
/// ```
pub struct Config {
  pub query: String,
  pub filename: String,
  pub ignore_case: bool,
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;
  let results = if config.ignore_case {
    search_case_insensitive(&config.query, &contents)
  } else {
    search(&config.query, &contents)
  };

  for line in results {
    println!("{}", line);
  }
  Ok(())
}

impl Config {
  pub fn new(
    mut args: impl Iterator<Item = String>,
  ) -> Result<Config, &'static str> {
    args.next();

    let query = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get query string"),
    };  

    let filename = match args.next() {
      Some(arg) => arg,
      None => return Err("Didn't get a file name"),
    };

    let ignore_case = env::var("IGNORE_CASE").is_ok();

    Ok(Config { query, filename, ignore_case })
  }
}
/// # Search Method (not case sensitive)
/// ```
/// Uses iterator pattern to search the contents of the file for a given query
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
  contents
    .lines()
    .filter(|line| line.contains(query))
    .collect()
} 

/// # Search Method (case sensitive)
/// ```
/// Uses iterator pattern to search the contents of the file for a given query
/// ```
pub fn search_case_insensitive<'a>(
  query: &str,
  contents: &'a str,
) -> Vec<&'a str> {
  let query = query.to_lowercase();
  contents
    .lines()
    .filter(|line| line.to_lowercase().contains(&query))
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
    assert_eq!(
      vec!["Rust:", "Trust me."],
      search_case_insensitive(query, contents)
    );
  }
}