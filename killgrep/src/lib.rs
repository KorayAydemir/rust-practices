use std::env;
use std::error::Error;
use std::fs;

#[cfg(test)]
mod tests {
    use crate::search;
    use crate::search_case_insensitive;
    const CONTENTS: &str = "\
Rust:
safe, fast, productive rust.
Pick three.";

    #[test]
    fn case_sensitive() {
        let query = "rust";
        assert_eq!(search(query, CONTENTS), ["safe, fast, productive rust."]);
    }

    #[test]
    fn no_results() {
        let query = "metamorphosis";
        assert_eq!(search(query, CONTENTS), Vec::<&str>::new());
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        assert_eq!(
            search_case_insensitive(query, CONTENTS),
            ["Rust:", "safe, fast, productive rust."]
        )
    }
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub struct Config<'a> {
    query: &'a String,
    file_path: &'a String,
    ignore_case_env: bool,
    ignore_case_arg: bool,
}

impl Config<'_> {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("You need to pass at least 2 arguments.
Example:
killgrep sugar recipes.txt");
        }
        let query = &args[1];
        let file_path = &args[2];
        let ignore_case_arg = if args.get(3).is_some() && &args[3] == "-i" {
            true
        } else {
            false
        };
        let ignore_case_env = env::var("IGNORE_CASE").is_ok();

        println!("Searching for {}", query);
        println!("In file {}", file_path);
        Ok(Config {
            query,
            file_path,
            ignore_case_arg,
            ignore_case_env,
        })
    }
}
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case_env || config.ignore_case_arg {
        search_case_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }
    Ok(())
}
