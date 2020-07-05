use crate::models::Result;

component!(GetExternalIP.external_ip -> ExternalIP {
  async fn get(&self) -> Result<String>;
});
