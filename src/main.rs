use doc_search_dsl::{rule, Rule};

fn main() {
    let page: Vec<String> = vec![
        "Hi".to_string(),
        "This is Jack".to_string(),
        "I want you to know".to_string(),
        "That i will be back tomorrow".to_string(),
    ];

    let pattern = rule! {
        any {
            r"\bHi\b"i,
            all {
                "this is"i,
                "will be",
            }
        }
    };

    assert_eq!(pattern.search(&page), 2);
}
