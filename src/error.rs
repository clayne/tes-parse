use nom::error::{ErrorKind, ParseError};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// Malformed magic numbers, etc
    #[error("Corrupt or invalid file {0}")]
    CorruptOrInvalidFile(String),
    /// Malformed records, subrecords
    #[error("Corrupt or invalid record {0}")]
    CorruptOrInvalidRecord(String),
    /// Invalid flags
    #[error("Could not parse flags {0:010X}")]
    InvalidFlags(u32),
    /// Forward an IO Error from std
    #[error("IOError: {0}")]
    IoError(#[from] std::io::Error),
    /// Unknown parsing error
    #[error("Unknown error while parsing {0:?}")]
    NomError(nom::error::ErrorKind),
    /// Unconsumed bytes after file fully parsed
    #[error("{0} unconsumed bytes")]
    UnconsumedBytes(usize),
    /// Forward a Utf8Error from std
    #[error("FromUtf8Error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    /// Weird errors that should never happen
    #[error("Unexpected error")]
    Unexpected,
}

impl<I> ParseError<I> for Error {
    fn from_error_kind(_: I, kind: ErrorKind) -> Self {
        Error::NomError(kind)
    }

    fn append(_: I, _: ErrorKind, other: Self) -> Self {
        other
    }
}
