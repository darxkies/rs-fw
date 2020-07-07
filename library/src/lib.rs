pub mod models;
pub mod interfaces;
pub mod adapters;
pub mod usecases;

pub use models::*;
pub use interfaces::*;
pub use usecases::*;
pub use adapters::*;

use std::sync::Arc;

macros::container!(Container, // Container Name
  // Getter Trait, Getter Method, Component Trait, Component Implementation
  GetLogger.log -> Logger = Log
  GetConfigFilename.config_filename -> ConfigFilename = ProgramArguments
  GetConfigLoader.config_loader -> ConfigLoader = FileConfigLoader
  GetConfigSaver.config_saver -> ConfigSaver = FileConfigSaver
  GetConfigRefresher.config_refresher -> ConfigRefresher = FileConfigRefresher
  GetConfig.config -> Configer = FileConfig
  GetDownloader.downloader -> Downloader = HttpDownloader
  GetExternalIP.external_ip -> ExternalIP = IpifyExternalIP
  GetWebService.web_service -> WebService = ActixWebService
);

