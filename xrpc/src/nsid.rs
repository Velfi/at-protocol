use convert_case::{Case, Casing};
use once_cell::sync::Lazy;
use regex::Regex;
use std::{fmt, str::FromStr};

// I wrote tests for this regex and I'm still not confident it's correct. 😅
static NSID_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([[[:alnum:]]\-]+)\.([[[:alnum:]]\-]+)\.([[[:alnum:]]\-]+\.?)+$").unwrap()
});

/// https://atproto.com/specs/nsid
///
/// ## Grammar
///
/// alpha     = "a" / "b" / "c" / "d" / "e" / "f" / "g" / "h" / "i" / "j" / "k" / "l" / "m" / "n" / "o" / "p" / "q" / "r" / "s" / "t" / "u" / "v" / "w" / "x" / "y" / "z" / "A" / "B" / "C" / "D" / "E" / "F" / "G" / "H" / "I" / "J" / "K" / "L" / "M" / "N" / "O" / "P" / "Q" / "R" / "S" / "T" / "U" / "V" / "W" / "X" / "Y" / "Z"
/// number    = "1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9" / "0"
/// delim     = "."
/// segment   = alpha *( alpha / number / "-" )
/// authority = segment *( delim segment )
/// name      = segment
/// nsid      = authority delim name
/// nsid-ns   = authority delim "*"
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nsid {
    inner: String,
}

impl Nsid {
    pub fn new(inner: impl ToString) -> Result<Self, Error> {
        let inner = inner.to_string();
        if inner.is_empty() {
            return Err(Error(ErrorRepr::Invalid));
        }

        if !NSID_REGEX.is_match(&inner) {
            return Err(Error(ErrorRepr::Invalid));
        }

        Ok(Self { inner })
    }

    pub fn authority(&self) -> String {
        // split the nsid into segments
        let mut parts = self
            .inner
            .split('.')
            // reverse the order of the segments
            .rev();
        // remove the last segment
        let _last = parts.next();
        // rejoin the segments, separated by a dot
        let mut authority = parts.fold(String::new(), |mut acc, part| {
            acc.push_str(part);
            acc.push('.');
            acc
        });
        // remove trailing dot
        let _ = authority.pop();

        authority
    }

    pub fn as_crate_name(&self) -> String {
        self.authority().replace('.', "-")
    }

    pub fn as_struct_name(&self) -> String {
        self.inner.split('.').last().unwrap().to_case(Case::Pascal)
    }
}

impl FromStr for Nsid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

impl fmt::Display for Nsid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner)
    }
}

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] ErrorRepr);

impl Error {
    // Accessors for anything we do want to expose publicly.
    pub fn invalid() -> Self {
        Self(ErrorRepr::Invalid)
    }
}

#[derive(thiserror::Error, Debug)]
enum ErrorRepr {
    Invalid,
}

impl fmt::Display for ErrorRepr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorRepr::Invalid => write!(f, "invalid nsid"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, Nsid, NSID_REGEX};
    use std::str::FromStr;

    #[test]
    fn test_empty() {
        let expected = Error::invalid().to_string();
        let actual = Nsid::new("")
            .expect_err("empty strings are invalid")
            .to_string();
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_authority() {
        let nsid = Nsid::new("com.atproto.test".to_string()).unwrap();
        assert_eq!(nsid.authority(), "atproto.com");
    }

    #[test]
    fn test_struct_name() {
        let nsid = Nsid::new("com.atproto.test".to_string()).unwrap();
        assert_eq!(nsid.as_struct_name(), "Test");
    }

    #[test]
    fn test_crate_name() {
        let nsid = Nsid::new("com.atproto.test".to_string()).unwrap();
        assert_eq!(nsid.as_crate_name(), "atproto-com");
    }

    #[test]
    fn test_from_str() {
        let nsid = Nsid::from_str("com.atproto.test").unwrap();
        assert_eq!(nsid.authority(), "atproto.com");
        assert_eq!(nsid.as_struct_name(), "Test");
    }

    #[test]
    fn test_regex() {
        assert!(NSID_REGEX.is_match("com.atproto.test"));
        assert!(NSID_REGEX.is_match("dashes.are.a-ok"));
        assert!(NSID_REGEX.is_match("com.atproto.testCamelCase"));
        assert!(NSID_REGEX.is_match("com.atproto.test.long.name.with.many.segments"));
        assert!(NSID_REGEX.is_match("99.numbers.are.ok.2"));
        assert!(!NSID_REGEX.is_match("symbols.are.not.ok.$#{!^["));
        assert!(!NSID_REGEX.is_match("spaces are not ok.at.all"));
        assert!(!NSID_REGEX.is_match("underscorse_are_not_ok.at.all"));
        assert!(!NSID_REGEX.is_match("multiple.dots.in.a.row...are.not.ok"));
    }
}
