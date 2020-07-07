use crate::models::Result;

macros::component!(GetDownloader.downloader -> Downloader {
  async fn get_string(&self, url: &String) -> Result<String>;
});

