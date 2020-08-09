use crate::models::*;
use anyhow::Context;
use crate::interfaces::*;
use std::sync::Arc;
use std::fs;

#[derive(Clone)]
pub struct FileConfigLoader {
  container: Arc<dyn Container + Send + Sync>,
}

impl FileConfigLoader {
  pub fn new(container: Arc<dyn Container + Send + Sync>) -> Result<Arc<Self>> {
    Ok(Arc::new(Self {
      container: container,
    }))
  }
}

impl ConfigLoader for FileConfigLoader {
    fn load(&self, config: &mut Config) -> VoidResult {
        let filename = self.container.config_filename()?.get();

        let content = fs::read_to_string(&filename)
          .with_context(|| Error::ReadFile(filename.clone()))?;

        *config = serde_yaml::from_str(&content)
          .with_context(|| Error::YamlDeserializeFile(filename.clone()))?;

        Ok(())
    }
}
