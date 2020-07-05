use crate::models::*;

component!(GetConfigRefresher.config_refresher -> ConfigRefresher {
    fn load(&self) -> Result<Config>;
});

