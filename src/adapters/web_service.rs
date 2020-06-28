use crate::interfaces::*;
use crate::models::*;
use std::sync::Arc;
use actix_rt::System;
use actix_web::{middleware, web, App, HttpServer};
use actix_web;

#[allow(dead_code)]
#[derive(Clone)]
pub struct ActixWebService<L: GetLogger + GetConfig + GetExternalIP + Sync + Clone + Send + 'static> {
  container: L,
}

impl<L: GetLogger + GetConfig + GetExternalIP + Sync + Clone + Send + 'static> ActixWebService<L> {
  pub fn new(container: L) -> Result<Arc<Self>> {
    Ok(Arc::new(ActixWebService{container:  container}))
  }

  async fn index(_data: web::Data<Arc<L>>) -> actix_web::Result<String> {
      if let Ok(ip) = _data.external_ip().get().await {
        return Ok(format!("External IP: {}", ip));
      }

      Ok("No external IP found!".to_string())
  }
}

impl<L: GetLogger + GetConfig + GetExternalIP + Sync + Clone + Send + 'static> WebService for ActixWebService<L> {
  fn run(&self) -> VoidResult {
    let sys = System::new("web-service");

    let data: Arc<L>  = Arc::new(self.container.clone());

    let address = self.container.config().get().web_listener.clone();

    HttpServer::new(move || {
      App::new()
          .data(data.clone())
          .wrap(middleware::Logger::default())
          .route("/", web::get().to(ActixWebService::<L>::index))
    })
    .bind(&address)
          .map_err(|error| ErrorKind::Wrapper(format!("Could not start server using '{}'", &address), error.to_string()))?
    .run();

    sys.run()?;

    self.container.log().info("Server killed".to_string());

    Ok(())
  }
}

