use crate::models::Result;
use crate::interfaces::*;
use std::sync::Arc;
use async_trait::async_trait;

#[allow(dead_code)]
#[derive(Clone)]
pub struct HttpDownloader<L: GetLogger + Sync> {
  container: L,
}

impl<L: GetLogger + Sync> HttpDownloader<L> {
  pub fn new(container: L) -> Result<Arc<Self>> {
    Ok(Arc::new(Self {
      container: container,
    }))
  }
}

#[async_trait]
impl<L: GetLogger + Sync> Downloader for HttpDownloader<L> {
  async fn get_string(&self, url: &String) -> Result<String> {
    let result = reqwest::get(url).await?
      .text().await?;

    self.container.log()?.info(format!("url: {} / response: {}", url, &result));

    Ok(result)
  }
}

