pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    UnknownError(String),
    FatalError(String),
    IoError(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::UnknownError(s) => s,
                Error::IoError(s) => s,
                Error::FatalError(s) => s,
            }
        )
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IoError(format!("{value}"))
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::UnknownError(value)
    }
}
