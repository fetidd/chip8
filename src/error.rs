pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Unknown(String),
    Fatal(String),
    Io(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::Unknown(s) => s,
                Error::Io(s) => s,
                Error::Fatal(s) => s,
            }
        )
    }
}

impl std::error::Error for Error {}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::Io(format!("{value}"))
    }
}

impl From<String> for Error {
    fn from(value: String) -> Self {
        Error::Unknown(value)
    }
}
