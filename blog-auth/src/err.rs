use jsonwebtoken::errors::ErrorKind;

#[derive(Debug)]
pub enum Kind {
    Generate,
    InvalidToken,
    InvalidIssuer,
    Expired,
}

#[derive(Debug)]
pub struct Error {
    pub kind: Kind,
    pub message: String,
    pub cause: Option<Box<dyn std::error::Error>>,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl std::error::Error for Error {}

impl From<jsonwebtoken::errors::Error> for Error {
    fn from(e: jsonwebtoken::errors::Error) -> Self {
        Self {
            kind: match e.kind() {
                ErrorKind::InvalidToken => Kind::InvalidToken,
                ErrorKind::InvalidIssuer => Kind::InvalidIssuer,
                ErrorKind::ExpiredSignature => Kind::Expired,
                _ => Kind::Generate,
            },
            message: e.to_string(),
            cause: Some(Box::new(e)),
        }
    }
}
