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

  #[error("Could not get option '{0}'")]
  Option(String),

  #[error("Could not load file '{0}'")]
  LoadFile(String),
}

pub type VoidResult = anyhow::Result<()>;
pub type Result<T> = anyhow::Result<T>;
