use std::fs;
use std::sync::Arc;
use anyhow::Context;
use crate::interfaces::*;
use crate::models::*;

#[derive(Clone)]
pub struct FileConfigSaver {
  container: Arc<dyn Container + Send + Sync>,
}

impl FileConfigSaver {
  pub fn new(container: Arc<dyn Container + Send + Sync>) -> Result<Arc<Self>> {
    Ok(Arc::new(Self {
      container: container,
    }))
  }
}

impl ConfigSaver for FileConfigSaver {
    fn save(&self, config: &Config) -> VoidResult {
        let filename = self.container.config_filename()?.get();

        let content = serde_yaml::to_string(config)
          .with_context(|| Error::YamlSerializeFile(filename.clone()))?;

        fs::write(&filename, &content)
          .with_context(|| Error::WriteFile(filename.clone()))?;

        Ok(())
    }
}
