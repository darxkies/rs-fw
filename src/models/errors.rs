use error_chain::error_chain;

error_chain! {
    foreign_links {
        Io(::std::io::Error);
        Yaml(serde_yaml::Error);
        Json(serde_json::Error);
        SetLogger(log::SetLoggerError);
        Reqwest(reqwest::Error);
        ParseIntError(std::num::ParseIntError);
        Utf8Error(std::str::Utf8Error);
    }
    errors {
        Wrapper(message: String, error: String) {
            description("Wrapping error")
            display("{} ({})", message, error)
        }
        Message(message: String) {
            description("Text message")
            display("{}", message)
        }
    }
}

impl<T> From<std::sync::PoisonError<T>> for Error {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        Self::from_kind(ErrorKind::Wrapper("PoisonError".to_string(), err.to_string()))
    }
}

pub type VoidResult = Result<()>;
