use std::fs;
use std::sync::Arc;
use crate::interfaces::*;
use crate::models::Config;
use crate::models::VoidResult;
use crate::models::Result;
use crate::models::ErrorKind;

#[allow(dead_code)]
#[derive(Clone)]
pub struct FileConfigSaver<T: GetConfigFilename> {
  container: T,
}

impl<T: GetConfigFilename> FileConfigSaver<T> {
  pub fn new(container: T) -> Result<Arc<Self>> {
    Ok(Arc::new(Self {
      container: container,
    }))
  }
}

impl<T: GetConfigFilename> ConfigSaver for FileConfigSaver<T> {
    fn save(&self, config: &Config) -> VoidResult {
        let filename = self.container.config_filename().get();

        let content = serde_yaml::to_string(config)
          .map_err(|error| ErrorKind::Wrapper(format!("Could not serialize '{:?}'", config), error.to_string()))?;

        fs::write(&filename, &content)
          .map_err(|error| ErrorKind::Wrapper(format!("Could not write to '{}'", &filename), error.to_string()))?;

        Ok(())
    }
}
