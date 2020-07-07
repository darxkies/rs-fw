use crate::models::Result;

macros::component!(GetExternalIP.external_ip -> ExternalIP {
  async fn get(&self) -> Result<String>;
});
