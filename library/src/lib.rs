pub mod models;
pub mod interfaces;
pub mod adapters;
pub mod usecases;

pub use models::*;
pub use interfaces::*;
pub use usecases::*;
pub use adapters::*;

use std::sync::Arc;

macros::container!(MainContainer,
  //  Getter Method, Component Trait, Component Implementation
  log -> Logger = Log
  config_filename -> ConfigFilename = ProgramArguments
  config_loader -> ConfigLoader = FileConfigLoader
  config_saver -> ConfigSaver = FileConfigSaver
  config_refresher -> ConfigRefresher = FileConfigRefresher
  config -> Configer = FileConfig
  downloader -> Downloader = HttpDownloader
  external_ip -> ExternalIP = IpifyExternalIP
  web_service -> WebService = ActixWebService
);

