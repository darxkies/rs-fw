use crate::models::*;
use crate::Getter;

// Getter Trait, Getter Method, Component Trait
Getter!(GetWebService, web_service, WebService);

pub trait WebService {
  fn run(&self) -> VoidResult;
}

