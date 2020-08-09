use crate::interfaces::*;
use crate::models::*;
use std::sync::Arc;
use actix_rt::System;
use actix_web::{middleware, web, App, HttpServer};
use actix_web;
use anyhow::{Context};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};

#[derive(Debug)]
struct AnyhowError {
   error: anyhow::Error
}

impl actix_web::error::ResponseError for AnyhowError {
}

impl std::fmt::Display for AnyhowError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.error.to_string())
    }
}

impl From<anyhow::Error> for AnyhowError {
    fn from(error: anyhow::Error) -> AnyhowError {
        AnyhowError { error }
    }
}

#[derive(Clone)]
pub struct ActixWebService {
  container: Arc<dyn Container + Send + Sync>,
}

impl ActixWebService {
  pub fn new(container: Arc<dyn Container + Send + Sync>) -> Result<Arc<Self>> {
    Ok(Arc::new(ActixWebService{container:  container}))
  }

  async fn index(_data: web::Data<Arc<dyn Container + Send + Sync>>) -> actix_web::Result<String> {
    match _data.external_ip()
        .map_err(|error| AnyhowError{error})?
        .get().await {
      Ok(ip) => return Ok(format!("External IP: {}", ip)),
      Err(error) => {
        _data.log()
          .map_err(|error| AnyhowError{error})?
          .error(format!("Could not get IP: {}", error.to_string()));

        return Ok("No external IP found!".to_string());
      }
    }
  }
}

impl WebService for ActixWebService {
  fn run(&self) -> VoidResult {
    let sys = System::new("web-service");

    let data: Arc<dyn Container + Send + Sync>  = self.container.clone();

    let address = self.container.config()?.get().web_service_listener.clone();
    let ca_certificate = self.container.config()?.get().web_service_ssl_ca_certificate.clone();
    let server_certificate = self.container.config()?.get().web_service_ssl_server_certificate.clone();
    let server_key = self.container.config()?.get().web_service_ssl_server_key.clone();

    let mut builder = SslAcceptor::mozilla_modern(SslMethod::tls())
      .context("Could not create ssl acceptor for Actix")?;

    builder
        .set_ca_file(ca_certificate.clone())
        .with_context(|| Error::LoadFile(ca_certificate.clone()))?;

    builder
      .set_certificate_chain_file(server_certificate.clone())
      .with_context(|| Error::LoadFile(server_certificate.clone()))?;

    builder
      .set_private_key_file(server_key.clone(), SslFiletype::PEM)
      .with_context(|| Error::LoadFile(server_key.clone()))?;

    HttpServer::new(move || {
      App::new()
          .data(data.clone())
          .wrap(middleware::Logger::default())
          .route("/", web::get().to(ActixWebService::index))
    })
    .bind_openssl(&address, builder)
          .with_context(|| Error::Bind(address))?
    .run();

    sys.run()?;

    self.container.log()?.info("Server killed".to_string());

    Ok(())
  }
}

