use crate::models::Result;
use crate::interfaces::*;
use std::sync::Arc;
use async_trait::async_trait;

#[derive(Clone)]
pub struct HttpDownloader {
  container: Arc<dyn Container + Send + Sync>,
}

impl HttpDownloader {
  pub fn new(container: Arc<dyn Container + Send + Sync>) -> Result<Arc<Self>> {
    Ok(Arc::new(Self {
      container: container,
    }))
  }
}

#[async_trait]
impl Downloader for HttpDownloader {
  async fn get_string(&self, url: &String) -> Result<String> {
    let result = reqwest::get(url).await?
      .text().await?;

    self.container.log()?.info(format!("url: {} / response: {}", url, &result));

    Ok(result)
  }
}

