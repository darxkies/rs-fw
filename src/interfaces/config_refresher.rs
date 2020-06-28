use crate::models::*;

// Getter Trait, Getter Method, Component Trait
Getter!(GetConfigRefresher, config_refresher, ConfigRefresher);

pub trait ConfigRefresher {
    fn load(&self) -> Result<Config>;
}

