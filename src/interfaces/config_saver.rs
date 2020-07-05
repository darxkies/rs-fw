use crate::models::Config;
use crate::models::VoidResult;

component!(GetConfigSaver.config_saver -> ConfigSaver {
    fn save(&self, config: &Config) -> VoidResult;
});

