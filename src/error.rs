#[derive(Debug)]
pub enum Error {
    EmptyVec { field: &'static str, min: usize },
    InvalidLanguageCode(String),
    InvalidValue(&'static str),
    MissingField(&'static str),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyVec { field, min } => {
                write!(f, "Field '{}' must have at least {} elements", field, min)
            }
            Error::InvalidLanguageCode(code) => write!(f, "Invalid language code: {}", code),
            Error::InvalidValue(field) => write!(f, "Invalid value for field: {}", field),
            Error::MissingField(field) => write!(f, "Missing required field: {}", field),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
