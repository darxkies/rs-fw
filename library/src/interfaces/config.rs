use crate::models::Config;
use std::sync::Arc;

macros::component!(GetConfig.config -> Configer {
  fn get(&self) -> Arc<Config>;
});
