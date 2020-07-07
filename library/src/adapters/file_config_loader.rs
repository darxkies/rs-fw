use crate::models::*;
use anyhow::Context;
use crate::interfaces::*;
use std::sync::Arc;
use std::fs;

#[allow(dead_code)]
#[derive(Clone)]
pub struct FileConfigLoader<T: GetConfigFilename> {
  container: T,
}

impl<T: GetConfigFilename> FileConfigLoader <T>{
  pub fn new(container: T) -> Result<Arc<Self>> {
    Ok(Arc::new(Self {
      container: container,
    }))
  }
}

impl<T: GetConfigFilename> ConfigLoader for FileConfigLoader<T> {
    fn load(&self, config: &mut Config) -> VoidResult {
        let filename = self.container.config_filename()?.get();

        let content = fs::read_to_string(&filename)
          .with_context(|| Error::ReadFile(filename.clone()))?;

        *config = serde_yaml::from_str(&content)
          .with_context(|| Error::YamlDeserializeFile(filename.clone()))?;

        Ok(())
    }
}
