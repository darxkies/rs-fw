use std::fs;
use std::sync::Arc;
use anyhow::Context;
use crate::interfaces::*;
use crate::models::*;

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
          .with_context(|| Error::YamlSerializeFile(filename.clone()))?;

        fs::write(&filename, &content)
          .with_context(|| Error::WriteFile(filename.clone()))?;

        Ok(())
    }
}
