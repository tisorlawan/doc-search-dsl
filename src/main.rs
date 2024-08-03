use doc_search_dsl::{pat, Rule};

fn main() {
    let content = vec![
        "Once upon a midnight dreary, while I pondered, weak and weary,",
        "Over many a quaint and curious volume of forgotten lore—",
        "While I nodded, nearly napping, suddenly there came a tapping,",
        "As of some one gently rapping, rapping at my chamber door.",
        "''Tis some visitor,' I muttered, 'tapping at my chamber door—",
        "Only this and nothing more.'",
    ];

    let p = pat! {
        all {
            "nodded",
            "WHILE",
            "nothing"
        }
    };
    println!("{}", p.occurances(&content));
}
