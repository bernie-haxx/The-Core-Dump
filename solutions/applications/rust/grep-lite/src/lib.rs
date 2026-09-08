use std::{env, error::Error, fs};

pub struct Config {
    pub word: String,
    pub file: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Remove the string path to the executer cause we don't need
        // it.
        args.next();

        let word = if let Some(x) = args.next() {
            x
        } else {
            return Err("Could not recover the string to match.");
        };

        let file = if let Some(x) = args.next() {
            x
        } else {
            return Err("Could not recover the file to pass.");
        };

        let case_sensitive = env::var("CASE_SENSITIVE_UWU").is_ok();

        Ok(Config {
            word,
            file,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file)?;
    let results = if config.case_sensitive {
        search_case_insensitive(&config.word, &contents)
    } else {
        search(&config.word, &contents)
    };

    for result in results {
        println!("{result}");
    }
    Ok(())
}

fn search_case_insensitive<'a>(
    query: &'a str,
    contents: &'a str,
) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    let query = query.to_lowercase();
    Box::new(
        contents
            .lines()
            .filter(move |line| line.to_lowercase().contains(&query)),
    )
}
fn search<'a>(query: &'a str, contents: &'a str) -> Box<dyn Iterator<Item = &'a str> + 'a> {
    let query = query.to_string();
    Box::new(contents.lines().filter(move |line| line.contains(&query)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

        assert_eq!(
            Some("safe, fast, productive."),
            search(query, contents).next()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";
        let mut result = search_case_insensitive(query, contents);

        assert_eq!(Some("Rust:"), result.next());
        assert_eq!(Some("Trust me."), result.next());
    }
    #[test]
    fn case_insensitive_1() {
        let query = "BRR";
        let contents = "\
Rust:
safe, fast, productive.
BRR BRR BRR.
brr brr brr.
Trust me.";
        let mut result = search_case_insensitive(query, contents);

        assert_eq!(Some("BRR BRR BRR."), result.next());
        assert_eq!(Some("brr brr brr."), result.next());
    }
}
