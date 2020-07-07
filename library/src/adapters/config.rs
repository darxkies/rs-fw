use crate::models::Result;
use crate::models::Config;
use crate::interfaces::*;
use std::sync::Arc;

#[allow(dead_code)]
#[derive(Clone)]
pub struct FileConfig<L: GetConfigRefresher + GetLogger> {
  container: L,
  config: Arc<Config>,
}

impl<L: GetConfigRefresher + GetLogger> FileConfig<L> {
  pub fn new(container: L) -> Result<Arc<Self>> {
    let config = container.config_refresher()?.load()?;

    Ok(Arc::new(Self {
      container: container,
      config: Arc::new(config),
    }))
  }
}

impl<L: GetConfigRefresher + GetLogger> Configer for FileConfig<L> {
  fn get(&self) -> Arc<Config> {
    self.config.clone()
  }
}

