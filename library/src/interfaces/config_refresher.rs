use crate::models::*;

macros::component!(GetConfigRefresher.config_refresher -> ConfigRefresher {
    fn load(&self) -> Result<Config>;
});

