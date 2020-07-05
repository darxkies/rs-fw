use crate::models::*;

component!(GetConfigLoader.config_loader -> ConfigLoader {
    fn load(&self, config: &mut Config) -> VoidResult;
});
