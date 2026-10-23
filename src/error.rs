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
        Self::InvalidGeometry
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyVec { field, min } => {
                write!(f, "Field '{field}' must have at least {min} elements")
            }
            Self::InvalidGeometry => write!(
                f,
                "Invalid geometry, expecting a valid GeoJSON or WKT representation"
            ),
            Self::InvalidLanguageCode(code) => write!(f, "Invalid language code: {code}"),
            Self::InvalidUri(uri) => write!(f, "Invalid URI: {uri}"),
            Self::InvalidValue(field) => write!(f, "Invalid value for field: {field}"),
            Self::MissingField(field) => write!(f, "Missing required field: {field}"),
            Self::MultiplePrimaryImages => write!(f, "Multiple images marked as primary"),
            Self::NoPrimaryImages => write!(f, "No images marked as primary"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
