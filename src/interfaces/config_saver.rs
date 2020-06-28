use crate::models::Config;
use crate::models::VoidResult;

// Getter Trait, Getter Method, Component Trait
Getter!(GetConfigSaver, config_saver, ConfigSaver);

pub trait ConfigSaver {
    fn save(&self, config: &Config) -> VoidResult;
}

