use doc_search_dsl::{Matcher, R};
use doc_search_dsl_macro::r;
use lazy_regex::regex;

fn main() {
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
