#[derive(Debug)]
pub enum ConversionError {
    Fs(std::io::Error),
    Json(serde_json::error::Error),
    Generic(String)
}

impl From<std::io::Error> for ConversionError {
    fn from(value: std::io::Error) -> Self {
        ConversionError::Fs(value)
    }
}

impl From<serde_json::error::Error> for ConversionError {
    fn from(value: serde_json::error::Error) -> Self {
        ConversionError::Json(value)
    }
}

impl From<String> for ConversionError {
    fn from(value: String) -> Self {
        ConversionError::Generic(value)
    }
}

impl std::fmt::Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ConversionError::Fs(e) => write!(f, "[FsError] {}", e),
            ConversionError::Json(e) => write!(f, "[JsonError] {}", e),
            ConversionError::Generic(e) => write!(f, "[GenericError] {}", e),
        }
    }
}
