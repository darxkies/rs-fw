use crate::Getter;

// Getter Trait, Getter Method, Component Trait
Getter!(GetLogger, log, Logger);

pub trait Logger {
    fn info(&self, message: String);
    fn error(&self, message: String);
    fn debug(&self, message: String);
}

