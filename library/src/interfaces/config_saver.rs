use crate::models::Config;
use crate::models::VoidResult;

macros::component!(GetConfigSaver.config_saver -> ConfigSaver {
    fn save(&self, config: &Config) -> VoidResult;
});

