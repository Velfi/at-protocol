// use once_cell::sync::Lazy;
// use regex::Regex;
// use crate::operation::Create;
// use base32::Alphabet;

/// A key referring to some DID
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct DidKey {
    key: String,
}

impl DidKey {
    pub fn new(method_name: &str, method_specific_id: &str) -> Result<Self, Error> {
        let key = format!("did:{}:{}", method_name, method_specific_id);

        validate_did_key(&key)?;

        Ok(DidKey { key })
    }

    pub fn as_bytes(&self) -> &[u8] {
        self.key.as_bytes()
    }
}

fn validate_did_key(key: &str) -> Result<(), Error> {
    let mut parts = key.split(':');

    // In practice, we don't need to check for this b/c we can only create a [DidKey] using the
    // new() method right now.
    if parts.next() != Some("did") {
        return Err(Error::must_start_with_did());
    }

    // Ensure that the method name is present and valid
    match parts.next() {
        Some(method_name) if !method_name.is_empty() => {
            method_name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
                .then(|| ())
                .ok_or(Error::invalid_method_name(method_name.to_owned()))?;
        }
        _ => return Err(Error::missing_method_name()),
    }

    // // Ensure that the method-specific ID is present and valid
    // match parts.next() {
    //     Some(method_specific_id) if !method_specific_id.is_empty() => {
    //         METHOD_SPECIFIC_ID_REGEX
    //             .is_match(method_specific_id)
    //             .then(|| ())
    //             .ok_or(Error::invalid_method_specific_id(method_specific_id.to_owned()))?;
    //     }
    //     _ => return Err(Error::missing_method_specific_id()),
    // }

    Ok(())
}

#[derive(thiserror::Error, Debug, PartialEq)]
#[error(transparent)]
pub struct Error(#[from] ErrorRepr);

impl Error {
    fn must_start_with_did() -> Self {
        Self(ErrorRepr::MustStartWithDid)
    }

    fn missing_method_name() -> Self {
        Self(ErrorRepr::MissingMethodName)
    }

    fn invalid_method_name(method_name: String) -> Self {
        Self(ErrorRepr::InvalidMethodName(method_name))
    }

    // fn missing_method_specific_id() -> Self {
    //     Self(ErrorRepr::MissingMethodSpecificId)
    // }

    // fn invalid_method_specific_id(method_specific_id: String) -> Self {
    //     Self(ErrorRepr::InvalidMethodSpecificId(method_specific_id))
    // }
}

#[derive(thiserror::Error, Debug)]
enum ErrorRepr {
    #[error("DID key must start with 'did:'")]
    MustStartWithDid,
    #[error("DID key must contain a method name")]
    MissingMethodName,
    #[error("DID key contains invalid method name '{0}'. Method name must only contain lowercase ASCII letters and digits.")]
    InvalidMethodName(String),
    // #[error("DID key must contain a method-specific ID")]
    // MissingMethodSpecificId,
    // #[error("DID key contains invalid method-specific ID '{0}'. Method-specific ID must only contain ASCII letters, digits, dots, dashes, underscores, and percent encoded chars.")]
    // InvalidMethodSpecificId(String),
}

impl PartialEq for ErrorRepr {
    fn eq(&self, other: &Self) -> bool {
        use ErrorRepr::*;
        match (self, other) {
            (MustStartWithDid, MustStartWithDid) => true,
            (MissingMethodName, MissingMethodName) => true,
            (InvalidMethodName(a), InvalidMethodName(b)) => a == b,
            // (MissingMethodSpecificId, MissingMethodSpecificId) => true,
            // (InvalidMethodSpecificId(a), InvalidMethodSpecificId(b)) => a == b,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DidKey, Error};

    #[test]
    fn test_did_key_with_percent_encoded_method_specific_id_is_valid() {
        let key = DidKey::new("example", "foo%20bar").unwrap();
        assert_eq!(key.as_bytes(), b"did:example:foo%20bar");
    }

    #[test]
    fn test_missing_method_name() {
        assert_eq!(
            DidKey::new("", "foo").unwrap_err(),
            Error::missing_method_name()
        );
    }

    #[test]
    fn test_invalid_method_name() {
        assert_eq!(
            DidKey::new("Foo", "foo").unwrap_err(),
            Error::invalid_method_name("Foo".to_owned())
        );
    }

    // #[test]
    // fn test_missing_method_specific_id() {
    //     assert_eq!(
    //         DidKey::new("foo", "").unwrap_err(),
    //         Error::missing_method_specific_id()
    //     );
    // }

    // #[test]
    // fn test_invalid_method_specific_id() {
    //     assert_eq!(
    //         DidKey::new("foo", "bar baz").unwrap_err(),
    //         Error::invalid_method_specific_id("bar baz".to_owned())
    //     );
    // }
}