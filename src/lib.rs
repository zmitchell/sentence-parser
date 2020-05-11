#[macro_use] extern crate pest_derive;
use pest::Parser;
#[cfg(test)]
use proptest::prelude::*;


#[derive(Parser)]
#[grammar = "sentence.pest"]
pub struct SentenceParser;

#[cfg(test)]
mod tests {
    use super::*;

    proptest!{
        #[test]
        fn parses_valid_word(s in "[a-zA-Z]+") {
            let parsed = SentenceParser::parse(Rule::word, s.as_str());
            prop_assert!(parsed.is_ok());
        }

        #[test]
        fn rejects_invalid_word(s in "[^a-zA-Z]*") {
            let parsed = SentenceParser::parse(Rule::word, s.as_str());
            prop_assert!(parsed.is_err());
        }
    }
}
