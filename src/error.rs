#[derive(Debug)]
pub enum Error {
    EmptyVec { field: &'static str, min: usize },
    InvalidGeometry,
    InvalidLanguageCode(String),
    InvalidUri(String),
    InvalidValue(&'static str),
    MissingField(&'static str),
    MultiplePrimaryImages,
    NoPrimaryImages,
}

impl From<wkt::conversion::Error> for Error {
    fn from(_: wkt::conversion::Error) -> Self {
        Error::InvalidGeometry
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::EmptyVec { field, min } => {
                write!(f, "Field '{}' must have at least {} elements", field, min)
            }
            Error::InvalidGeometry => write!(
                f,
                "Invalid geometry, expecting a valid GeoJSON or WKT representation"
            ),
            Error::InvalidLanguageCode(code) => write!(f, "Invalid language code: {}", code),
            Error::InvalidUri(uri) => write!(f, "Invalid URI: {}", uri),
            Error::InvalidValue(field) => write!(f, "Invalid value for field: {}", field),
            Error::MissingField(field) => write!(f, "Missing required field: {}", field),
            Error::MultiplePrimaryImages => write!(f, "Multiple images marked as primary"),
            Error::NoPrimaryImages => write!(f, "No images marked as primary"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
