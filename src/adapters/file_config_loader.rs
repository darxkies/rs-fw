use crate::models::*;
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
        let filename = self.container.config_filename().get();

        let content = fs::read_to_string(&filename)
          .map_err(|error| ErrorKind::Wrapper(format!("Could not read from '{}'", &filename), error.to_string()))?;

        *config = serde_yaml::from_str(&content)
          .map_err(|error| ErrorKind::Wrapper(format!("Could not deserialize '{}'", &filename), error.to_string()))?;

        Ok(())
    }
}
