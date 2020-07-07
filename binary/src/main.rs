use log::error;
use library::*;

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
