component!(GetLogger.log -> Logger {
    fn info(&self, message: String);
    fn error(&self, message: String);
    fn debug(&self, message: String);
});
