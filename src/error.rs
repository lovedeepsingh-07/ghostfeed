use serde::ser::SerializeStruct;

macro_rules! define_error {
    ($( $variant:ident ),* $(,)?) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum Error { $( $variant(String) ),* }

        impl std::error::Error for Error {}

        impl Error {
            pub fn kind(&self) -> &str {
                match self { $( Error::$variant(_) => stringify!($variant) ),* }
            }
            pub fn message(&self) -> &str {
                match self { $( Error::$variant(s) => s.as_str() ),* }
            }
        }

        impl std::fmt::Display for Error {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}: {}", self.kind(), self.message())
            }
        }
    };
}

define_error!(
    NotFoundError,
    InvalidInputError,
    IOError,
    FSError,
    RequestError,
    ParseError,
    HtmlSelectorError,
    SerializeError,
    DeserializeError,
    BoaEngineError,
    AlreadyRunningError,
    DatabaseError,
    ChannelError,
    SerenityError,
);

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut s = serializer.serialize_struct("Error", 2)?;
        s.serialize_field("kind", self.kind())?;
        s.serialize_field("message", self.message())?;
        s.end()
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Error::IOError(value.to_string())
    }
}
impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Error::DeserializeError(value.to_string())
    }
}
impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Error::RequestError(value.to_string())
    }
}
impl<T> From<tokio::sync::mpsc::error::SendError<T>> for Error {
    fn from(value: tokio::sync::mpsc::error::SendError<T>) -> Self {
        Error::ChannelError(value.to_string())
    }
}
impl From<poise::serenity_prelude::Error> for Error {
    fn from(value: poise::serenity_prelude::Error) -> Self {
        Error::SerenityError(value.to_string())
    }
}
impl<T> From<tokio::sync::broadcast::error::SendError<T>> for Error {
    fn from(value: tokio::sync::broadcast::error::SendError<T>) -> Self {
        Error::ChannelError(value.to_string())
    }
}
