use crate::models::Config;
use crate::models::VoidResult;

Getter!(GetConfigLoader, config_loader, ConfigLoader);

pub trait ConfigLoader {
    fn load(&self, config: &mut Config) -> VoidResult;
}

