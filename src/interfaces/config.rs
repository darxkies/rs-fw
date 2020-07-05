use crate::models::Config;
use std::sync::Arc;

component!(GetConfig.config -> Configer {
  fn get(&self) -> Arc<Config>;
});
