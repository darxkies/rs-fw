#![macro_use] 

#[macro_export]
macro_rules! Getter {
  ($name:ident, $method:ident, $interface:ident) => {
    pub trait $name {
      fn $method(&self) -> crate::models::Result<std::sync::Arc<dyn $interface + Send + Sync>>;
    }
  }
}

#[macro_export]
macro_rules! component {
  ($name:ident . $method:ident -> $interface:ident $body:tt) => {
    pub trait $name {
      fn $method(&self) -> crate::models::Result<std::sync::Arc<dyn $interface + Send + Sync>>;
    }

    #[async_trait::async_trait]
    pub trait $interface 
      $body
  }
}

#[macro_export]
macro_rules! container {
  ($container_name:ident, $($getter_interface:ident, $method:ident, $interface:ident, $component:ident)*) => {
    #[derive(Default)]
    struct $container_name {
      $(
        $method: Option<Arc<dyn $interface + Send + Sync>>,
      )*
    }

    impl $container_name {
      pub fn new() -> crate::models::Result<Arc<std::sync::Mutex<$container_name>>> {
        let container = Arc::new(std::sync::Mutex::new($container_name::default()));

        $(
          container
            .lock()
            .map_err(|_| Error::Lock(stringify!($method).to_string()))?
            .$method = Some($component::new(container.clone())?);
        )*

        Ok(container)
      }
    }

    $(
      impl $getter_interface for Arc<std::sync::Mutex<$container_name>> {
        fn $method(&self) -> crate::models::Result<Arc<dyn $interface + Send + Sync>> {
          Ok(self
            .lock()
            .map_err(|_| Error::Lock(stringify!($method).to_string()))?
            .$method
            .as_ref()
            .clone()
            .ok_or_else(|| Error::Option(stringify!($method).to_string()))
            .unwrap()
            .clone())
        }
      }
    )*
  }
}

