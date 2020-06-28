use chrono::Local;
use env_logger::Builder;
use log::info;
use log::error;
use log::debug;
use log::LevelFilter;
use env_logger::DEFAULT_FILTER_ENV;
use std::io::Write;
use crate::interfaces::Logger;
use std::sync::Arc;
use crate::models::Result;

#[derive(Copy, Clone)]
pub struct Log<T> {
  container: T
}

impl<T> Log<T> {
	pub fn new(container: T) -> Result<Arc<Self>> {
    if let Err(_) = std::env::var(DEFAULT_FILTER_ENV) {
      std::env::set_var(DEFAULT_FILTER_ENV, LevelFilter::Info.to_string());
    }

		Builder::from_default_env()
			.format(|buffer, record| {
				writeln!(
					buffer,
					"{} [{}] - {}",
					Local::now().format("%Y-%m-%dT%H:%M:%S"),
					record.level(),
					record.args()
				)
			})
			.init();

		Ok(Arc::new(Log {container: container}))
	}
}

impl<T> Logger for Log<T> {
	fn info(&self, message: String) {
		info!("{}", &message);
	}

	fn error(&self, message: String) {
		error!("{}", &message);
	}

	fn debug(&self, message: String) {
		debug!("{}", &message);
	}
}
