use crate::Getter;

// Getter Trait, Getter Method, Component Trait
Getter!(GetConfigFilename, config_filename, ConfigFilename);

pub trait ConfigFilename {
  fn get(&self) -> String;
}

