use core::fmt;

#[derive(Copy, Clone, Debug)]
pub enum LogLevel {
    Nothing,
    Error,
    Warning,
    Message,
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub trait Logger {
    fn log(&self, log_level: &LogLevel, message: &str);
    fn log_message(&self, message: &str);
    fn log_warning(&self, warning: &str);
    fn log_error(&self, error: &str);
}