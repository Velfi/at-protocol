mod parser;

pub struct Abnf<T> {
    _marker: std::marker::PhantomData<T>,
}

impl<T> Abnf<T> {
    pub fn from_metasyntax(metasyntax: &str) -> Result<Self, Error> {
        let _ = metasyntax;
        todo!()
    }

    pub fn parse(&self, _input: &str) -> Result<T, Error> {
        todo!()
    }

    pub fn builder() -> Builder {
        Builder::default()
    }
}

#[derive(Default)]
pub struct Builder {}

// impl<T> Builder {
//     pub fn build(self) -> Result<Abnf<T>, Error> {
//         Ok(Abnf { inner: () })
//     }
// }

#[derive(thiserror::Error, Debug)]
#[error(transparent)]
pub struct Error(#[from] ErrorRepr);

impl Error {
    pub fn unexpected(message: &'static str) -> Self {
        Self(ErrorRepr::Unexpected(message))
    }
}

#[derive(thiserror::Error, Debug)]
enum ErrorRepr {
    #[error("unexpected error: {0}")]
    Unexpected(&'static str),
}

#[cfg(test)]
mod tests {
    // #[test]
    // fn test_nsid() {
    //     struct Nsid {
    //         namespace: String,
    //         name: String,
    //     };

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

    //     // let nsid =
    //     //     Abnf::from_metasyntax::<Nsid>(nsid_abnf, "nsid").expect("nsid_abnf is valid ABNF definition");

    //     fn rule(input: &str) {
    //         todo!();
    //     }

    //     let nsid = Abnf::builder()
    //         .rule("alpha", r#""a" / "b" / "c" / "d" / "e" / "f" / "g" / "h" / "i" / "j" / "k" / "l" / "m" / "n" / "o" / "p" / "q" / "r" / "s" / "t" / "u" / "v" / "w" / "x" / "y" / "z" / "A" / "B" / "C" / "D" / "E" / "F" / "G" / "H" / "I" / "J" / "K" / "L" / "M" / "N" / "O" / "P" / "Q" / "R" / "S" / "T" / "U" / "V" / "W" / "X" / "Y" / "Z""#)
    //         .rule("number", r#""1" / "2" / "3" / "4" / "5" / "6" / "7" / "8" / "9" / "0""#)
    //         .rule("delim", r#"".""#)
    //         .rule("segment", r#"alpha *( alpha / number / "-" )"#)
    //         .rule("authority", "segment *( delim segment )")
    //         .rule("name", "segment")
    //         .rule("nsid", "authority delim name ")
    //         .rule("nsid-ns", r#"authority delim "*""#)
    //         .build::<Nsid>()
    //         .expect("nsid_abnf is valid ABNF definition");

    //         assert!(nsid.parse("com.example.status").is_ok());
    //         assert!(nsid.parse("io.social.getFeed").is_ok());
    //         assert!(nsid.parse("net.users.bob.ping").is_ok());
    // }

    // #[test]
    // fn test_atp_uri() {
    //     struct AtpUri(String);

    //     const atp_uri_abnf: &str = r##"
    //         atp-url   = "at://" authority path [ "#" fragment ]
    //         authority = reg-name / did
    //         path      = [ "/" coll-nsid [ "/" record-id ] ]
    //         coll-nsid = nsid
    //         record-id = 1*pchar
    //     "##;

    //     // let atp_uri =
    //     //     Abnf::from_metasyntax::<AtpUri>(atp_uri_abnf, "atp-url").expect("atp_uri_abnf is valid ABNF definition");

    //     let atp_uri = Abnf::builder()
    //         .rule("atp-url", r##""at://" authority path [ "#" fragment ]"##)
    //         .rule("authority", "reg-name / did")
    //         .rule("path", r#"[ "/" coll-nsid [ "/" record-id ] ]"#)
    //         .rule("coll-nsid", "nsid")
    //         .rule("record-id", "1*pchar")
    //         .build::<AtpUri>()
    //         .expect("atp_uri_abnf is valid ABNF definition");

    //     assert!(atp_uri.parse("at://alice.host.com").is_ok());
    //     assert!(atp_uri.parse("at://did:plc:bv6ggog3tya2z3vxsub7hnal").is_ok());
    //     assert!(atp_uri.parse("at://alice.host.com/io.example.song").is_ok());
    //     assert!(atp_uri
    //         .parse("at://alice.host.com/io.example.song/3yI5-c1z-cc2p-1a")
    //         .is_ok());
    //     assert!(atp_uri
    //         .parse("at://alice.host.com/io.example.song/3yI5-c1z-cc2p-1a#/title")
    //         .is_ok());
    // }
}
