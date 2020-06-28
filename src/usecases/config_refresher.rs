use crate::models::*;
use crate::interfaces::*;
use std::sync::Arc;
use std::path::Path;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FileConfigRefresher<T: GetConfigFilename + GetConfigLoader + GetConfigSaver> {
  container: T,
}

impl<T: GetConfigFilename + GetConfigLoader + GetConfigSaver> FileConfigRefresher <T>{
  pub fn new(container: T) -> Result<Arc<Self>> {
    Ok(Arc::new(Self {
      container: container,
    }))
  }
}

impl<T: GetConfigFilename + GetConfigLoader + GetConfigSaver> ConfigRefresher for FileConfigRefresher<T> {
    fn load(&self) -> Result<Config> {
      let mut config = Config::default();

      let filename = self.container.config_filename().get();

      if Path::new(&filename).exists() {
        self.container.config_loader().load(&mut config)?;
      } else {
        self.container.config_saver().save(&config)?;
      }
       
      Ok(config)
    }
}
