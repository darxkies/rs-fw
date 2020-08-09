use crate::models::Result;
use crate::interfaces::*;
use std::sync::Arc;

use clap::{App, Arg};

pub const NAME: &'static str = env!("CARGO_PKG_NAME");
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
const CONFIG: &str = "config";
const DEFAULT_CONFIG_FILENAME: &str = "config.yaml";

#[derive(Clone)]
pub struct ProgramArguments {
  container: Arc<dyn Container + Send + Sync>,
  config_filename: String,
}

impl ProgramArguments {
  pub fn new(container: Arc<dyn Container + Send + Sync>) -> Result<Arc<Self>> {
    let matches = App::new(NAME)
        .version(VERSION)
        .author("Darx Kies <darxkies@gmail.com>")
        .about(&*format!("{}", NAME))
        .arg(
            Arg::with_name(CONFIG)
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .get_matches();

    let config_filename = matches.value_of(CONFIG).unwrap_or(DEFAULT_CONFIG_FILENAME);

    Ok(Arc::new(Self {
      container: container,
      config_filename: config_filename.to_string().clone(),
    }))
  }
}

impl ConfigFilename for ProgramArguments {
  fn get(&self) -> String {
    self.config_filename.clone()
  }
}

