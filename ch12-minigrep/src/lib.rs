fn search_by<'a, F>(contents: &'a str, mut predicate: F) -> impl Iterator<Item = &'a str>
where
    F: FnMut(&str) -> bool,
{
    contents.lines().filter(move |line| predicate(line))
}

pub fn search<'a>(query: &str, contents: &'a str) -> impl Iterator<Item = &'a str> {
    search_by(contents, move |line| line.contains(query))
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
) -> impl Iterator<Item = &'a str> {
    let query = query.to_lowercase();

    search_by(contents, move |line| line.to_lowercase().contains(&query))
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
Pick three.";

        let result: Vec<_> = search(query, contents).collect();

        assert_eq!(vec!["safe, fast, productive."], result);
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        let result: Vec<_> = search_case_insensitive(query, contents).collect();

        assert_eq!(vec!["Rust:", "Trust me."], result);
    }
}
