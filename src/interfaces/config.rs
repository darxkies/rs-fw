use crate::models::Config;
use crate::Getter;
use std::sync::Arc;

// Getter Trait, Getter Method, Component Trait
Getter!(GetConfig, config, Configer);

pub trait Configer {
  fn get(&self) -> Arc<Config>;
}

