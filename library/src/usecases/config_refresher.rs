use crate::models::*;
use crate::interfaces::*;
use std::sync::Arc;
use std::path::Path;

#[allow(dead_code)]
#[derive(Clone)]
pub struct FileConfigRefresher {
  container: Arc<dyn Container + Send + Sync>,
}

impl FileConfigRefresher {
  pub fn new(container: Arc<dyn Container + Send + Sync>) -> Result<Arc<Self>> {
    Ok(Arc::new(Self {
      container: container,
    }))
  }
}

impl ConfigRefresher for FileConfigRefresher {
    fn load(&self) -> Result<Config> {
      let mut config = Config::default();

      let filename = self.container.config_filename()?.get();

      if Path::new(&filename).exists() {
        self.container.config_loader()?.load(&mut config)?;
      } else {
        self.container.config_saver()?.save(&config)?;
      }
       
      Ok(config)
    }
}
