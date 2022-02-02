use std::fmt;

use colored::*;

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

pub struct ConsoleLogger {
    log_level: LogLevel,
    use_colors: bool
}

impl ConsoleLogger {
    pub fn new(log_level: LogLevel, use_colors: bool) -> ConsoleLogger{
        ConsoleLogger { 
            log_level, 
            use_colors
        }
    }

    pub fn log_message(&self, message: &str) {
        self.log(&LogLevel::Message, message);
    }

    pub fn log_warning(&self, warning: &str) {
        self.log(&LogLevel::Warning, warning);
    }

    pub fn log_error(&self, error: &str) {
        self.log(&LogLevel::Error, error);
    }

    fn should_log(&self, log_level: &LogLevel) -> bool{
        if (*log_level as i32) > (self.log_level as i32) {
            return false;
        }
        true
    }

    fn log(&self, log_level: &LogLevel, message: &str) {
        if !self.should_log(log_level) {
            return;
        }
        let level_name = log_level.to_string().to_uppercase();
        
        let formatted_message = format!("[{}] {}", level_name, message);
        let final_message = self.compute_colored_message(&formatted_message, log_level);
        
        match log_level {
            LogLevel::Error => eprintln!("{}", final_message),
            _ =>  println!("{}", final_message)
        }
    }

    fn compute_colored_message(&self, message: &str, log_level: &LogLevel) -> ColoredString {
        if !self.use_colors {
            return message.white();
        }

        match log_level {
            LogLevel::Warning => message.yellow(),
            LogLevel::Error => message.red(),
            _ => message.white(),
        }
    }
}