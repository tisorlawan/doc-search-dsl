pub use doc_search_dsl_macro::r;
use lazy_regex::regex::Regex;
use rayon::prelude::*;

#[derive(Clone)]
pub enum R {
    One(&'static Regex),
    Sequence(Vec<&'static Regex>),
    And(Vec<R>),
    Or(Vec<R>),
}

pub trait Matcher {
    fn search(&self, page: &[String]) -> usize;
}

impl Matcher for R {
    fn search(&self, page: &[String]) -> usize {
        match self {
            R::One(r) => page
                .par_iter()
                .filter(|line| r.is_match(line.trim()))
                .count(),
            R::Sequence(patterns) => {
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
            R::And(patterns) => patterns.iter().map(|p| p.search(page)).min().unwrap_or(0),
            R::Or(patterns) => patterns.iter().map(|p| p.search(page)).sum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_regex::regex;

    #[test]
    fn test_regex_pattern() {
        let page: Vec<String> = vec![
            "BER.TA ACARA PENGGELEDAHAN BADAN".to_string(),
            //"Some other line".to_string(),
            "BER.TA ACARA".to_string(),
            "PENGGELEDAHAN".to_string(),
            "BER.TA ACARA PENG".to_string(),
            "Berdasarkan Surat Perintah Penggeledahan Badan".to_string(),
        ];

        let pattern = r!(
            all {
                any {
                    "^BER.TA ACARA PENGGELEDAHAN BADAN",
                    "^BER.TA ACARA$",
                    "^BER.TA ACARA PENG"
                },
                sequence {
                    "^BER.TA ACARA$",
                    ".*PENGGELEDAHAN.*"
                },
                sequence {
                    "^BER.TA ACARA PENG",
                    "Berdasarkan Surat Perintah Penggeledahan Badan"
                }
            }
        );

        assert_eq!(pattern.search(&page), 1);
    }
}
