/*
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
*/

use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
  #[error("Could not deserialize yaml file '{0}'")]
  YamlDeserializeFile(String),

  #[error("Could not serialize yaml file '{0}'")]
  YamlSerializeFile(String),

  #[error("Could not read from file '{0}'")]
  ReadFile(String),

  #[error("Could not write to file '{0}'")]
  WriteFile(String),

  #[error("Could not start server using '{0}'")]
  Bind(String),

  #[error("Could not lock '{0}'")]
  Lock(String),
}

pub type VoidResult = anyhow::Result<()>;
pub type Result<T> = anyhow::Result<T>;
