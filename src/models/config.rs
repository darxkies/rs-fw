use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub web_listener: String,
}

impl Config {
	pub fn default() -> Config {
		let config = Config{
      web_listener: "0.0.0.0:8080".to_string(),
		};

		config
	}
}
