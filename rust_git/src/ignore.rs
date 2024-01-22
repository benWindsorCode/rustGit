use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use glob::Pattern;
use crate::index::Index;
use crate::repository::Repository;

#[derive(PartialEq, Debug)]
pub struct Ignore {
    rules: Vec<IgnoreRule>
}

#[derive(PartialEq, Debug)]
pub enum IgnoreRule {
    Normal(String),
    Negate(String),
    Literal(String),
    Comment(String),
    Empty
}

impl Ignore {

    pub fn new(rules: Vec<IgnoreRule>) -> Self {
        Ignore { rules }
    }

    pub fn add_rule(&mut self, rule: IgnoreRule) {
        self.rules.push(rule);
    }

    /// Given a repository read the .gitignore file in the index (so only staged gitignore files)
    /// AND only the one that is at the root directory, so the core .gitignore file
    ///
    /// Note: I simplify this a little from the tutorial, in the original tutorial
    /// the gitignore setup is more advanced and supports subfolders containing
    /// .gitignore as well as .gitignore files in the index itself which
    /// adds complexity https://wyag.thb.lt/#cmd-check-ignore
    ///
    pub fn read(repo: &Repository) -> Self {
        match Index::read(&repo) {
            Ok(index) => {
                if let Some(entry) = index.get_gitignore() {
                    println!("Opening gitignore file: {:?}", entry.name);
                    let file = File::open(entry.name).unwrap();
                    let reader = BufReader::new(file);

                    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

                    Ignore::parse_file(lines)
                } else {
                    Ignore::new(vec![])
                }
            }
            Err(_err) => Ignore::new(vec![])
        }
    }

    /// Given the contents of a file turn it into a full Ignore ruleset
    /// This skips over any Comment or Empty fields
    /// ```
    /// use rust_git::ignore::{Ignore, IgnoreRule};
    /// let lines = vec![
    ///     "test.txt".to_string(),
    ///     "#comment to skip".to_string(),
    ///     "!negative.pattern".to_string()
    /// ];
    /// let parsed_output = Ignore::parse_file(lines);
    ///
    /// let expected_output = Ignore::new(vec![
    ///     IgnoreRule::Normal("test.txt".to_string()),
    ///     IgnoreRule::Negate("negative.pattern".to_string())
    /// ]);
    /// assert_eq!(parsed_output, expected_output);
    /// ```
    pub fn parse_file(lines: Vec<String>) -> Self {
        let mut rules = Vec::new();
        for line in lines {
            match IgnoreRule::from_string(line) {
                IgnoreRule::Empty | IgnoreRule::Comment(_) => continue,
                rule @ _ => rules.push(rule)
            }
        }

        Ignore { rules }
    }

    /// Return:
    ///     None = if no patterns match
    ///     Some(bool) = the last match in the list of matches
    ///
    /// ```
    /// use rust_git::ignore::{Ignore, IgnoreRule};
    /// let lines = vec![
    ///     "test.txt".to_string(),
    ///     "#comment to skip".to_string(),
    ///     "*.iml".to_string()
    /// ];
    /// let ignore = Ignore::parse_file(lines);
    ///
    /// let result_1 = ignore.check_ignore("my/file/name.txt".to_string());
    /// let expected_1 = None;
    ///
    /// assert_eq!(result_1, expected_1);
    /// ```
    pub fn check_ignore(&self, path: String) -> Option<bool> {
        let path = Path::new(&path);

        let mut matches = vec![];
        for rule in &self.rules {
            match rule {
                IgnoreRule::Normal(rule) => {
                    println!("Checking Normal rule: {:?}", rule);
                    let pattern = Pattern::new(&rule);
                    matches.push(pattern.unwrap().matches_path(&path));
                },
                IgnoreRule::Negate(_) | IgnoreRule::Literal(_) | IgnoreRule::Comment(_) | IgnoreRule::Empty => {},
            }
        }

        println!("Matches computed for {:?} are:{:?}", path, matches);
        let all_false = matches.iter().all(|x| !x.to_owned());

        if all_false {
            None
        } else {
            Some(matches.last().unwrap().clone())
        }
    }
}

impl IgnoreRule {
    /// Given a line in a file turn it into an IgnoreRule
    /// ```
    /// use rust_git::ignore::IgnoreRule;
    /// let comment_line = "#this is a comment".to_string();
    /// let comment_rule = IgnoreRule::from_string(comment_line);
    ///
    /// let comment_expected = IgnoreRule::Comment("this is a comment".to_string());
    /// assert_eq!(comment_rule, comment_expected);
    /// ```
    pub fn from_string(line: String) -> Self {
        // TODO: shouldn't I implement the From trait?
        let line = line.trim().to_string();

        if line.len() == 0 {
            return IgnoreRule::Empty;
        }

        let mut line_drop_first = line.chars();
        line_drop_first.next();

        match line.chars().next().unwrap() {
            '#' => IgnoreRule::Comment(line_drop_first.as_str().into()),
            '!' => IgnoreRule::Negate(line_drop_first.as_str().into()),
            '\\' => IgnoreRule::Literal(line_drop_first.as_str().into()),
            _ => IgnoreRule::Normal(line)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ignore::Ignore;

    #[test]
    fn test_ignore_star_pattern() {
        let lines = vec![
            "test.txt".to_string(),
            "#comment to skip".to_string(),
            "*.iml".to_string()
        ];
        let ignore = Ignore::parse_file(lines);
        let result_3 = ignore.check_ignore("now/other.iml".to_string());
        let expected_3 = Some(true);
        assert_eq!(result_3, expected_3);
    }
}