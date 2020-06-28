use crate::models::Result;
use crate::Getter;
use async_trait::async_trait;

// Getter Trait, Getter Method, Component Trait
Getter!(GetDownloader, downloader, Downloader);

#[async_trait]
pub trait Downloader {
  async fn get_string(&self, url: &String) -> Result<String>;
}

