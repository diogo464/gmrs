#[derive(Debug)]
pub enum Error {
    CustomMessage(String),
    Generic(Box<dyn std::error::Error + Send>),
}
impl<E: std::error::Error + Send + 'static> From<E> for Error {
    fn from(e: E) -> Self {
        Self::Generic(Box::new(e))
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CustomMessage(msg) => write!(f, "{}", msg),
            Self::Generic(err) => write!(f, "{}", err),
        }
    }
}

pub type Result<T, E = Error> = std::result::Result<T, E>;
