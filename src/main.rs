#[allow(dead_code)]
pub mod models;
pub mod interfaces;
pub mod adapters;
pub mod usecases;

use log::error;
use models::*;
use interfaces::*;
use adapters::*;
use usecases::*;

use std::sync::*;

container!(Container, // Container Name
  // Getter Trait, Getter Method, Component Trait, Component Implementation
  GetLogger, log, Logger, Log
  GetConfigFilename, config_filename, ConfigFilename, ProgramArguments
  GetConfigLoader, config_loader, ConfigLoader, FileConfigLoader
  GetConfigSaver, config_saver, ConfigSaver, FileConfigSaver
  GetConfigRefresher, config_refresher, ConfigRefresher, FileConfigRefresher
  GetConfig, config, Configer, FileConfig
  GetDownloader, downloader, Downloader, HttpDownloader
  GetExternalIP, external_ip, ExternalIP, IpifyExternalIP
  GetWebService, web_service, WebService, ActixWebService
);

fn run() -> VoidResult {
  let _container = Container::new()?;

  _container.log()?.info(format!("{} {}", NAME, VERSION));

  _container.web_service()?.run()?;

  Ok(())
}

fn main() -> VoidResult {
	if let Err(error) = run() {
    error.chain().for_each(|cause| error!("{}", cause));

    std::process::exit(-1);
	}

  Ok(())
}
