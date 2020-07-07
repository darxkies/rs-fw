use crate::models::*;

macros::component!(GetConfigLoader.config_loader -> ConfigLoader {
    fn load(&self, config: &mut Config) -> VoidResult;
});
