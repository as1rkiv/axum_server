use super::Error;

// validator
impl From<validator::ValidationErrors> for Error {
    fn from(error: validator::ValidationErrors) -> Self {
        Self::Tips(error.to_string())
    }
}

// axum
impl From<axum::Error> for Error {
    fn from(error: axum::Error) -> Self {
        tracing::error!("Axum 错误: {}", error);

        Self::Unavailable
    }
}

// MultipartError
impl From<axum::extract::multipart::MultipartError> for Error {
    fn from(error: axum::extract::multipart::MultipartError) -> Self {
        tracing::error!("Axum 错误: {}", error);

        Self::Unavailable
    }
}

// reqwest
impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        tracing::error!("Reqwest 错误: {}", error);

        Self::Unavailable
    }
}

// serde_json
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        tracing::error!("Serde 错误: {}", error);

        Self::Unavailable
    }
}

// uuid
impl From<uuid::Error> for Error {
    fn from(error: uuid::Error) -> Self {
        tracing::error!("UUID 错误: {}", error);

        Self::Unavailable
    }
}

// std::io::Error
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        tracing::error!("I/O 错误: {}", error);

        match error.kind() {
            std::io::ErrorKind::NotFound => Self::NotFound,
            std::io::ErrorKind::TimedOut => Self::TimedOut,
            _ => Self::Unavailable,
        }
    }
}
