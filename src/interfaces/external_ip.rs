use crate::models::Result;
use crate::Getter;
use async_trait::async_trait;

// Getter Trait, Getter Method, Component Trait
Getter!(GetExternalIP, external_ip, ExternalIP);

#[async_trait]
pub trait ExternalIP {
  async fn get(&self) -> Result<String>;
}

