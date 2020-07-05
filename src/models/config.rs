use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize, Clone)]
pub struct Config {
    pub web_service_listener: String,
    pub web_service_ssl_ca_certificate: String,
    pub web_service_ssl_server_key: String,
    pub web_service_ssl_server_certificate: String,
}

impl Config {
	pub fn default() -> Config {
		let config = Config{
      web_service_listener: "0.0.0.0:8080".to_string(),
      web_service_ssl_ca_certificate: "data/ssl/ca-certificate.pem".to_string(),
      web_service_ssl_server_certificate: "data/ssl/server-certificate.pem".to_string(),
      web_service_ssl_server_key: "data/ssl/server-key.pem".to_string(),
		};

		config
	}
}
