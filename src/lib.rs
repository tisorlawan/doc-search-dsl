pub use doc_search_dsl_macro::rule;
use lazy_regex::regex::Regex;
use rayon::prelude::*;

#[derive(Clone)]
pub enum Rule {
    One(&'static Regex),
    Sequence(Vec<&'static Regex>),
    And(Vec<Rule>),
    Or(Vec<Rule>),
}

impl Rule {
    pub fn search(&self, page: &[String]) -> usize {
        match self {
            Rule::One(r) => page
                .par_iter()
                .filter(|line| r.is_match(line.trim()))
                .count(),
            Rule::Sequence(patterns) => {
                if patterns.len() == 1 {
                    return page
                        .par_iter()
                        .filter(|line| patterns[0].is_match(line.trim()))
                        .count();
                }
                page.windows(patterns.len())
                    .filter(|window| {
                        window
                            .iter()
                            .zip(patterns)
                            .all(|(line, pattern)| pattern.is_match(line.trim()))
                    })
                    .count()
            }
            Rule::And(patterns) => patterns.iter().map(|p| p.search(page)).min().unwrap_or(0),
            Rule::Or(patterns) => patterns.iter().map(|p| p.search(page)).sum(),
        }
    }
}
