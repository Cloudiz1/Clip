use crate::arg::Type;

#[derive(Debug)]
pub enum Error {
    UnknownArgument(String),
    ExpectedParameter(String),
    TypeError(Type),
}

impl std::error::Error for Error {}

impl From<Type> for Error {
    fn from(t: Type) -> Self {
        Self::TypeError(t)
    }
}

// TODO: this needs SO much work
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::UnknownArgument(arg) => write!(f, "unrecognized argument: {arg}"),
            Error::ExpectedParameter(param) => write!(f, "expected param: {param}"),
            Error::TypeError(expected) => write!(f, "parameter must be of type {expected}"),
        }
    }
}

