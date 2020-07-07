use crate::models::*;

macros::component!(GetWebService.web_service -> WebService {
  fn run(&self) -> VoidResult;
});
