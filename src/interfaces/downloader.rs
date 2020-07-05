use crate::models::Result;

component!(GetDownloader.downloader -> Downloader {
  async fn get_string(&self, url: &String) -> Result<String>;
});

