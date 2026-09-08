use std::{env, error::Error, fs};

pub struct Config {
    pub filepath: String,
    pub query: String,
    pub case_sensitive: bool,
}

impl Config {
    /// Parses command-line arguments into a `Config`.
    ///
    ///
    /// Expects an Iterator of arguments in the shape of `env::args()`:
    /// - The first item is skipped due to the value being the executable path.
    /// - Followed by the query and the filepath.
    /// - Check for the `CASE_SENSITIVE_UWU` flag for case insensitive cases.
    ///
    ///
    /// # Errors
    ///
    /// Returns `Err` if the query or filepath is not identified in `args`.
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // Skip the executable path.
        args.next();

        // Get query string.
        let query = if let Some(query) = args.next() {
            query
        } else {
            return Err("Query string not found.");
        };

        // Get filepath string.
        let filepath = if let Some(filepath) = args.next() {
            filepath
        } else {
            return Err("Filepath not found");
        };

        // Check for case_sensitive_uwu_flag
        let case_sensitive = env::var("CASE_SENSITIVE_UWU").is_ok();

        Ok(Config {
            query,
            filepath,
            case_sensitive,
        })
    }
}

/// evaluates the contents of the file and return search results.
///
/// Based on the `CASE_SENSITIVE_UWU` flag, we pass the right function
/// to be used.
///
///
/// # Errors
///
/// Returns `Err` if the filepath is not found
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // contents are retrieved if not a error is passed.
    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.case_sensitive {
        search(config.query, &contents)
    } else {
        search_case_insensitive(config.query, &contents)
    };

    for result in results {
        println!("{}", result)
    }

    Ok(())
}

fn search(query: String, contents: &String) -> Box<dyn Iterator<Item = String> + '_> {
    Box::new(
        contents
            .lines()
            .filter(move |line| line.contains(&query))
            .map(move |line| line.to_string()),
    )
}

fn search_case_insensitive(
    query: String,
    contents: &String,
) -> Box<dyn Iterator<Item = String> + '_> {
    let query = query.to_lowercase();
    Box::new(
        contents
            .lines()
            .filter(move |line| line.to_lowercase().contains(&query))
            .map(|line| line.to_string()),
    )
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct".to_string();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape"
            .to_string();

        assert_eq!(
            Some("safe, fast, productive.".to_string()),
            search(query, &contents).next()
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT".to_string();
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me."
            .to_string();
        let mut result = search_case_insensitive(query, &contents);

        assert_eq!(Some("Rust:".to_string()), result.next());
        assert_eq!(Some("Trust me.".to_string()), result.next());
    }
    #[test]
    fn case_insensitive_1() {
        let query = "BRR".to_string();
        let contents = "\
Rust:
safe, fast, productive.
BRR BRR BRR.
brr brr brr.
Trust me."
            .to_string();
        let mut result = search_case_insensitive(query, &contents);

        assert_eq!(Some("BRR BRR BRR.".to_string()), result.next());
        assert_eq!(Some("brr brr brr.".to_string()), result.next());
    }
}
