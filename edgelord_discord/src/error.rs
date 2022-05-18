#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    JsonValueError,
    WrongOptionType,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::JsonValueError => write!(f, "parsing json value is out of i64 or u64"),
            Error::WrongOptionType => write!(f, "wrong option type was passed"),
        }
    }
}
