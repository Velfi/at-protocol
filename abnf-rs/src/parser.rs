// TODO remove this once this stuff starts getting used
#![allow(dead_code)]

use nom::branch::alt;
use nom::character::complete::{char, digit0, digit1, hex_digit1, line_ending, space1};
use nom::combinator::opt;
use nom::multi::{many0, many1};
use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::alpha1,
    combinator::recognize,
    sequence::{pair, tuple},
    IResult,
};
use std::fmt;
use tracing::{span, trace, Level};

// The ABNF for ABNFs!
// https://www.rfc-editor.org/rfc/rfc5234
//
//  rulelist       =  1*( rule / (*c-wsp c-nl) )
//  rule           =  rulename defined-as elements c-nl
//                         ; continues if next line starts
//                         ;  with white space
//  rulename       =  ALPHA *(ALPHA / DIGIT / "-")
//  defined-as     =  *c-wsp ("=" / "=/") *c-wsp
//                         ; basic rules definition and
//                         ;  incremental alternatives
//  elements       =  alternation *c-wsp
//  c-wsp          =  WSP / (c-nl WSP)
//  c-nl           =  comment / CRLF
//                         ; comment or newline
//  comment        =  ";" *(WSP / VCHAR) CRLF
//  alternation    =  concatenation
//                    *(*c-wsp "/" *c-wsp concatenation)
//  concatenation  =  repetition *(1*c-wsp repetition)
//  repetition     =  [repeat] element
//  repeat         =  1*DIGIT / (*DIGIT "*" *DIGIT)
//  element        =  rulename / group / option /
//                    char-val / num-val / prose-val
//  group          =  "(" *c-wsp alternation *c-wsp ")"
//  option         =  "[" *c-wsp alternation *c-wsp "]"
//  char-val       =  DQUOTE *(%x20-21 / %x23-7E) DQUOTE
//                         ; quoted string of SP and VCHAR
//                         ;  without DQUOTE
//  num-val        =  "%" (bin-val / dec-val / hex-val)
//  bin-val        =  "b" 1*BIT
//                    [ 1*("." 1*BIT) / ("-" 1*BIT) ]
//                         ; series of concatenated bit values
//                         ;  or single ONEOF range
//  dec-val        =  "d" 1*DIGIT
//                    [ 1*("." 1*DIGIT) / ("-" 1*DIGIT) ]
//  hex-val        =  "x" 1*HEXDIG
//                    [ 1*("." 1*HEXDIG) / ("-" 1*HEXDIG) ]
//  prose-val      =  "<" *(%x20-3D / %x3F-7E) ">"
//                         ; bracketed string of SP and VCHAR
//                         ;  without angles
//                         ; prose description, to be used as
//                         ;  last resort

fn rule_list(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "rule_list", input = i);
    let _enter = span.enter();
    recognize(many1(alt((rule, recognize(pair(many0(c_wsp), c_nl))))))(i)
}

fn rule(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "rule", input = i);
    let _enter = span.enter();

    let res = recognize(tuple((rule_name, defined_as, elements, c_nl)))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

// https://www.rfc-editor.org/rfc/rfc5234#section-2.1
fn rule_name(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "rule_name", input = i);
    let _enter = span.enter();
    let res = recognize(pair(
        alpha1,
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '-'),
    ))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn defined_as(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "defined_as", input = i);
    let _enter = span.enter();
    let res = recognize(tuple((
        many0(c_wsp),
        alt((tag("="), tag("=/"))),
        many0(c_wsp),
    )))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn elements(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "elements", input = i);
    let _enter = span.enter();
    let res = recognize(pair(alternation, many0(c_wsp)))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn c_wsp(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "c_wsp", input = i);
    let _enter = span.enter();
    let res = alt((tag(" "), recognize(pair(c_nl, char(' ')))))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn c_nl(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "c_nl", input = i);
    let _enter = span.enter();
    let res = alt((comment, line_ending))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn comment(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "comment", input = i);
    let _enter = span.enter();
    let res = recognize(tuple((
        char(';'),
        many0(alt((
            space1,
            take_while1(|c: char| c.is_ascii() && (0x21..=0x7E).contains(&(c as u8))),
        ))),
        line_ending,
    )))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

// https://www.rfc-editor.org/rfc/rfc5234#section-3.2
fn alternation(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "alternation", input = i);
    let _enter = span.enter();
    let res = recognize(pair(
        concatenation,
        many0(tuple((
            many0(c_wsp),
            char('/'),
            many0(c_wsp),
            concatenation,
        ))),
    ))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

// https://www.rfc-editor.org/rfc/rfc5234#section-3.1
fn concatenation(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "concatenation", input = i);
    let _enter = span.enter();
    let res = recognize(pair(repetition, many0(pair(many1(c_wsp), repetition))))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

// https://www.rfc-editor.org/rfc/rfc5234#section-3.6
// https://www.rfc-editor.org/rfc/rfc5234#section-3.7
fn repetition(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "repetition", input = i);
    let _enter = span.enter();
    let res = recognize(pair(opt(repeat), element))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn repeat(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "repeat", input = i);
    let _enter = span.enter();
    let res = alt((
        // Check the more specific case first, otherwise the less specific case will
        // always win out.
        recognize(tuple((digit0, char('*'), digit0))),
        digit1,
    ))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn element(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "element", input = i);
    let _enter = span.enter();
    let res = alt((rule_name, group, option, char_val, num_val, prose_val))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

// https://www.rfc-editor.org/rfc/rfc5234#section-3.5
fn group(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "group", input = i);
    let _enter = span.enter();
    // TODO: split out all the elements in the group
    let res = recognize(tuple((
        char('('),
        many0(c_wsp),
        alternation,
        many0(c_wsp),
        char(')'),
    )))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

// https://www.rfc-editor.org/rfc/rfc5234#section-3.8
fn option(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "option", input = i);
    let _enter = span.enter();
    // TODO: split out all the elements in the option
    let res = recognize(tuple((
        char('['),
        many0(c_wsp),
        alternation,
        many0(c_wsp),
        char(']'),
    )))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn char_val(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "char_val", input = i);
    let _enter = span.enter();
    let res = recognize(tuple((
        char('"'),
        take_while(|c: char| {
            // take while `c` is ASCII and is in the range (inclusive) of
            // 0x20 (space) to 0x21 (!) or 0x23 (#) to 0x7E (~)
            // _(This range excludes double quotes)_
            c.is_ascii()
                && ((0x20..=0x21).contains(&(c as u8)) || (0x23..=0x7E).contains(&(c as u8)))
        }),
        char('"'),
    )))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

// https://www.rfc-editor.org/rfc/rfc5234#section-2.3
fn num_val(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "num_val", input = i);
    let _enter = span.enter();
    let res = recognize(pair(char('%'), alt((bin_val, dec_val, hex_val))))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn bin_val(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "bin_val", input = i);
    let _enter = span.enter();
    // TODO convert this output to a number or something
    // TODO handle the weird concatenated version of these
    //    https://www.rfc-editor.org/rfc/rfc5234#section-2.3
    let res = recognize(pair(char('b'), take_while1(|c| c == '0' || c == '1')))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn dec_val(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "dec_val", input = i);
    let _enter = span.enter();
    // TODO convert this output to a number or something
    // TODO handle the weird concatenated version of these
    //    https://www.rfc-editor.org/rfc/rfc5234#section-2.3
    let res = recognize(pair(char('d'), digit1))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

fn hex_val(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "hex_val", input = i);
    let _enter = span.enter();
    // TODO convert this output to a number or something
    // TODO handle the weird concatenated version of these
    //    https://www.rfc-editor.org/rfc/rfc5234#section-2.3
    let res = recognize(pair(char('x'), hex_digit1))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

// https://www.rfc-editor.org/rfc/rfc5234#section-2.3
fn prose_val(i: &str) -> IResult<&str, &str> {
    let span = span!(Level::TRACE, "prose_val", input = i);
    let _enter = span.enter();
    let res = recognize(tuple((
        char('<'),
        take_while(|c: char| {
            c.is_ascii()
                && ((0x20..=0x3D).contains(&(c as u8)) || (0x3F..=0x7E).contains(&(c as u8)))
        }),
        char('>'),
    )))(i);

    if res.is_ok() {
        trace!("success")
    } else {
        trace!("failure")
    };

    res
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    Unexpected(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unexpected(message) => write!(f, "unexpected error: {}", message),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::sync::Once;

    static INIT_LOGGER: Once = Once::new();
    fn init_logger() {
        INIT_LOGGER.call_once(|| {
            tracing_subscriber::fmt::init();
        });
    }

    #[test]
    fn test_rule_parsing() {
        init_logger();

        let input = "elements       =  alternation *c-wsp\r\n";
        let (remaining, s) = rule(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        let input = "element        =  rulename / group / option \r\n                  char-val / num-val / prose-val\r\n";
        let (remaining, s) = rule(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_rule_name() {
        init_logger();

        let input = "rule-name";
        let (remaining, s) = rule_name(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        let input = "x100910912";
        let (remaining, s) = rule_name(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        let input = "x----";
        let (remaining, s) = rule_name(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_invalid_rule_name() {
        init_logger();

        let input = "1-rule-name";
        let err = rule_name(input).expect_err("rule name is invalid");
        assert_eq!(
            err.to_string(),
            "Parsing Error: Error { input: \"1-rule-name\", code: Alpha }"
        );

        let input = "-rule-name";
        let err = rule_name(input).expect_err("rule name is invalid");
        assert_eq!(
            err.to_string(),
            "Parsing Error: Error { input: \"-rule-name\", code: Alpha }"
        );
    }

    #[test]
    fn test_c_nl_parsing() {
        init_logger();

        let input = "\r\n";
        let (remaining, s) = c_nl(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        let input = "\n";
        let (remaining, s) = c_nl(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // Comments count as valid line endings too
        let input = "; some comment\r\n";
        let (remaining, s) = c_nl(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_comment_parsing() {
        init_logger();

        let input = "; this is a comment\r\n";
        let (remaining, s) = comment(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_alternation() {
        init_logger();

        let input = "a / b / c";
        let (remaining, s) = alternation(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_repetition_parsing() {
        init_logger();

        // Repetitions may not even have a repeat count
        let input = "\"word\"";
        let (remaining, s) = repetition(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // 0 or more repetitions of the string "word"
        let input = "*\"word\"";
        let (remaining, s) = repetition(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // 1 or more repetitions of the string "word"
        let input = "1*\"word\"";
        let (remaining, s) = repetition(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // 1 to 5 repetitions of the string "word"
        let input = "1*5\"word\"";
        let (remaining, s) = repetition(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_repeat_parsing() {
        init_logger();

        // 0 or more repeats
        let input = "*";
        let (remaining, s) = repeat(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // 0 or 1 repeats
        let input = "*1";
        let (remaining, s) = repeat(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // 1 or more repeats
        let input = "1*";
        let (remaining, s) = repeat(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // 1 to 5 repeats
        let input = "1*5";
        let (remaining, s) = repeat(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_element_parsing() {
        init_logger();

        // rulename
        let input = "a-rule-name-ending-with-123";
        let (remaining, s) = element(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // group
        let input = "(a group)";
        let (remaining, s) = element(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // option
        let input = "[repeat]";
        let (remaining, s) = element(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // char-val
        let input = "\"some string\"";
        let (remaining, s) = element(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // num-val
        let input = "%x1F";
        let (remaining, s) = element(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);

        // prose-val
        let input = "<prose-val>";
        let (remaining, s) = element(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_char_val() {
        init_logger();

        let input = r#""hello""#;
        let (remaining, s) = char_val(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_bin_val() {
        init_logger();

        let input = r#"%b1010"#;
        let (remaining, s) = num_val(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_dec_val() {
        init_logger();

        let input = r#"%d10"#;
        let (remaining, s) = num_val(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_hex_val() {
        init_logger();

        let input = r#"%x10"#;
        let (remaining, s) = num_val(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }

    #[test]
    fn test_prose_val() {
        init_logger();

        let input = r#"<hello>"#;
        let (remaining, s) = prose_val(input).unwrap();
        assert_eq!(remaining, "");
        assert_eq!(input, s);
    }
}

//     const nsid_abnf: &str = r#"
//         alpha     = "a" / "b" / "c" / "d" / "e" / "f" / "g" / "h" / "i" / "j" / "k" / "l" / "m" / "n" / "o" / "p" / "q" / "r" / "s" / "t" / "u" / "v" / "w" / "x" / "y" / "z" / "A" / "B" / "C" / "D" / "E" / "F" / "G" / "H" / "I" / "J" / "K" / "L" / "M" / "N" / "O" / "P" / "Q" / "R" / "S" / "T" / "U" / "V" / "W" / "X" / "Y" / "Z"
//         number    = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9" / "0"
//         delim     = "."
//         segment   = alpha *( alpha / number / "-" )
//         authority = segment *( delim segment )
//         name      = segment
//         nsid      = authority delim name
//         nsid-ns   = authority delim "*"
//     "#;
