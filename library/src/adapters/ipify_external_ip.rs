use crate::models::Result;
use crate::interfaces::*;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Clone)]
pub struct IpifyExternalIP {
  container: Arc<dyn Container + Send + Sync>,
}

impl IpifyExternalIP {
  pub fn new(container: Arc<dyn Container + Send + Sync>) -> Result<Arc<Self>> {
    Ok(Arc::new(Self {
      container: container,
    }))
  }
}

#[async_trait]
impl ExternalIP for IpifyExternalIP {
  async fn get(&self) -> Result<String> {
    self.container.downloader()?.get_string(&"https://api.ipify.org".to_string()).await 
  }
}

