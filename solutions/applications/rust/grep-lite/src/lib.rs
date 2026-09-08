use std::{env, error::Error, fs};

/// Search Task manual enum wrapper
enum SearchTask<A, B> {
    // For the case sensitive case
    CaseSensitive(A),
    // For the case insensitive case
    CaseInsensitive(B),
}

/// Implementing the iterator trait to the wrapper.
///
/// Enabling the values to be iterated through.
impl<'a, A, B> Iterator for SearchTask<A, B>
where
    A: Iterator<Item = &'a str>,
    B: Iterator<Item = &'a str>,
{
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            SearchTask::CaseSensitive(it) => it.next(),
            SearchTask::CaseInsensitive(it) => it.next(),
        }
    }
}

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
/// Based on the `CASE_SENSITIVE_UWU` flag, wraps the result of the
/// matching search function in the corresponding `SearchTask` variant,
/// so both branches can be iterated thorugh a single type.
///
///
/// # Errors
///
/// Returns `Err` if the filepath is not found
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // contents are retrieved if not a error is passed.
    let contents = fs::read_to_string(config.filepath)?;

    let results = if config.case_sensitive {
        SearchTask::CaseSensitive(search(config.query, contents.as_str()))
    } else {
        SearchTask::CaseInsensitive(search_case_insensitive(config.query, contents.as_str()))
    };

    for result in results {
        println!("{}", result)
    }

    Ok(())
}

fn search<'a>(query: String, contents: &'a str) -> impl Iterator<Item = &'a str> {
    contents.lines().filter(move |line| line.contains(&query))
}

fn search_case_insensitive<'a>(query: String, contents: &'a str) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(move |line| line.to_lowercase().contains(&query))
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
Duct tape";

        assert_eq!(
            Some("safe, fast, productive."),
            search(query, contents).next()
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

        assert_eq!(Some("Rust:"), result.next());
        assert_eq!(Some("Trust me."), result.next());
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

        assert_eq!(Some("BRR BRR BRR."), result.next());
        assert_eq!(Some("brr brr brr."), result.next());
    }
}
