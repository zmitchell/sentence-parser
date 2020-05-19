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

    #[derive(Debug, PartialEq, Clone)]
    enum EnclosedDelimiter {
        OpenParen,
        CloseParen,
        OpenComma,
        CloseComma,
    }

    impl EnclosedDelimiter {
        fn to_str(&self) -> &str {
            match self {
                EnclosedDelimiter::OpenParen => "(",
                EnclosedDelimiter::CloseParen => ")",
                EnclosedDelimiter::OpenComma => ", ",
                EnclosedDelimiter::CloseComma => ","
            }
        }
    }

    impl Arbitrary for EnclosedDelimiter {
        type Parameters = ();
        type Strategy = BoxedStrategy<Self>;

        fn arbitrary_with(_: Self::Parameters) -> Self::Strategy {
            prop_oneof![
                Just(EnclosedDelimiter::OpenParen),
                Just(EnclosedDelimiter::CloseParen),
                Just(EnclosedDelimiter::OpenComma),
                Just(EnclosedDelimiter::CloseComma),
            ].boxed()
        }
    }

    fn words() -> impl Strategy<Value = String> {
        proptest::string::string_regex("[a-z]+( [a-z]+)*").unwrap()
    }

    fn opening_delimiter() -> impl Strategy<Value = EnclosedDelimiter> {
        prop_oneof![
            Just(EnclosedDelimiter::OpenParen),
            Just(EnclosedDelimiter::OpenComma)
        ]
    }

    fn valid_enclosed() -> impl Strategy<Value = String> {
        (opening_delimiter(), words()).prop_flat_map(|(open, ws)| {
            let close = match open {
                EnclosedDelimiter::OpenParen => Just(EnclosedDelimiter::CloseParen),
                EnclosedDelimiter::OpenComma => Just(EnclosedDelimiter::CloseComma),
                _ => unreachable!()
            };
            (Just(open), close, Just(ws))
        }).prop_map(|(open, close, ws)| {
            let mut enclosed = String::new();
            enclosed.push_str(open.to_str());
            enclosed.push_str(ws.as_str());
            enclosed.push_str(close.to_str());
            enclosed
        })
    }

    fn invalid_enclosed() -> impl Strategy<Value = String> {
        (any::<EnclosedDelimiter>(), words()).prop_flat_map(|(open, ws)| {
            let close = match open {
                EnclosedDelimiter::OpenParen => prop_oneof![
                    Just(EnclosedDelimiter::OpenParen),
                    Just(EnclosedDelimiter::OpenComma),
                    Just(EnclosedDelimiter::CloseComma)
                ].boxed(),
                EnclosedDelimiter::CloseParen => any::<EnclosedDelimiter>(),
                EnclosedDelimiter::OpenComma => prop_oneof![
                    Just(EnclosedDelimiter::OpenParen),
                    Just(EnclosedDelimiter::CloseParen),
                    // Just(EnclosedDelimiter::OpenComma),
                ].boxed(),
                EnclosedDelimiter::CloseComma => any::<EnclosedDelimiter>(),
            };
            (Just(open), close, Just(ws))
        }).prop_map(|(open, close, ws)| {
            let mut enc = String::new();
            enc.push_str(open.to_str());
            enc.push_str(ws.as_str());
            enc.push_str(close.to_str());
            enc
        })
    }

    fn missing_closing_delimiter() -> impl Strategy<Value = String> {
        (any::<EnclosedDelimiter>(), words()).prop_map(|(open, ws)| {
            let mut enc = String::new();
            enc.push_str(open.to_str());
            enc.push_str(ws.as_str());
            enc
        })
    }

    fn punctuation() -> impl Strategy<Value = String> {
        prop_oneof![
            Just(String::from(".")),
            Just(String::from("?")),
            Just(String::from("!")),
        ]
    }

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

        #[test]
        fn parses_valid_words(s in words()) {
            let parsed = SentenceParser::parse(Rule::words, s.as_str());
            prop_assert!(parsed.is_ok());
        }

        #[test]
        fn parses_valid_words_chunk(s in words()) {
            let parsed = SentenceParser::parse(Rule::chunk, s.as_str());
            prop_assert!(parsed.is_ok());
        }

        #[test]
        fn parses_valid_enclosed(enc in valid_enclosed()) {
            let parsed = SentenceParser::parse(Rule::enclosed, enc.as_str());
            prop_assert!(parsed.is_ok());
        }

        #[test]
        fn rejects_mismatched_enclosed_delimiters(enc in invalid_enclosed()) {
            let parsed = SentenceParser::parse(Rule::enclosed, enc.as_str());
            prop_assert!(parsed.is_err());
        }

        #[test]
        fn rejects_missing_closing_delimiter(enc in missing_closing_delimiter()) {
            let parsed = SentenceParser::parse(Rule::enclosed, enc.as_str());
            prop_assert!(parsed.is_err());
        }

        #[test]
        fn parses_valid_enclosed_chunk(enc in valid_enclosed()) {
            let parsed = SentenceParser::parse(Rule::chunk, enc.as_str());
            prop_assert!(parsed.is_ok());
        }

        #[test]
        fn parses_valid_punctuation(s in punctuation()) {
            let parsed = SentenceParser::parse(Rule::punctuation, s.as_str());
            prop_assert!(parsed.is_ok());
        }

        #[test]
        fn rejects_invalid_punctuation(s in "[^\\.\\?!]") {
            let parsed = SentenceParser::parse(Rule::punctuation, s.as_str());
            prop_assert!(parsed.is_err());
        }
    }
}
