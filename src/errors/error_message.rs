use core::fmt;

#[derive(Debug, PartialEq)]
pub enum ErrorMessage {
    // Auth-related
    EmptyPassword,
    ExceededMaxPasswordLength(usize),
    InvalidToken,
    TokenNotProvided,
    UserNotAuthenticated,
    WrongCredentials,
    PermissionDenied,

    // User management
    EmailExist,
    UserNoLongerExist,

    // System
    HashingError,
    InvalidHashFormat,
    ServerError,
}

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            ErrorMessage::EmptyPassword => "Password is required",
            ErrorMessage::ExceededMaxPasswordLength(_) => "Password exceeds maximum length",
            ErrorMessage::InvalidHashFormat => "Invalid password hash format!",
            ErrorMessage::HashingError => "Failed to hash password",
            ErrorMessage::InvalidToken => "Invalid token",
            ErrorMessage::ServerError => "Internal server error",
            ErrorMessage::WrongCredentials => "Wrong email or password",
            ErrorMessage::EmailExist => "Email already exists",
            ErrorMessage::UserNoLongerExist => "User no longer exists",
            ErrorMessage::TokenNotProvided => "Token was not provided",
            ErrorMessage::PermissionDenied => "You do not have permission to perform this action",
            ErrorMessage::UserNotAuthenticated => "User is not authenticated",
        };
        write!(f, "{}", message)
    }
}
