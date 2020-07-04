use crate::models::Result;
use crate::interfaces::*;
use std::sync::Arc;
use async_trait::async_trait;

#[allow(dead_code)]
#[derive(Clone)]
pub struct IpifyExternalIP<L: GetLogger + GetDownloader + Sync> {
  container: L,
}

impl<L: GetLogger + GetDownloader + Send + Sync> IpifyExternalIP<L> {
  pub fn new(container: L) -> Result<Arc<Self>> {
    Ok(Arc::new(Self {
      container: container,
    }))
  }
}

#[async_trait]
impl<L: GetLogger + GetDownloader + Send + Sync> ExternalIP for IpifyExternalIP<L> {
  async fn get(&self) -> Result<String> {
    self.container.downloader()?.get_string(&"https://api.ipify.org".to_string()).await 
  }
}

