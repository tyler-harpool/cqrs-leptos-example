#[derive(Debug, Clone)]
pub enum AppError {
    NotFound,
    DatabaseError(String),
    InvalidInput(String),
    Unauthorized,
    InternalServerError,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::NotFound => write!(f, "Resource not found"),
            AppError::DatabaseError(e) => write!(f, "Database error: {}", e),
            AppError::InvalidInput(e) => write!(f, "Invalid input: {}", e),
            AppError::Unauthorized => write!(f, "Unauthorized access"),
            AppError::InternalServerError => write!(f, "Internal server error"),
        }
    }
}

impl std::error::Error for AppError {}
