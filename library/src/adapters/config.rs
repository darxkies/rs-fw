use crate::models::Result;
use crate::models::Config;
use crate::interfaces::*;
use std::sync::Arc;

#[derive(Clone)]
pub struct FileConfig {
  container: Arc<dyn Container + Send + Sync>,
  config: Arc<Config>,
}

impl FileConfig {
  pub fn new(container: Arc<dyn Container + Send + Sync>) -> Result<Arc<Self>> {
    let config = container.config_refresher()?.load()?;

    Ok(Arc::new(Self {
      container: container,
      config: Arc::new(config),
    }))
  }
}

impl Configer for FileConfig {
  fn get(&self) -> Arc<Config> {
    self.config.clone()
  }
}

