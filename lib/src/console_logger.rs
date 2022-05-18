use colored::{ColoredString, Colorize};

use crate::log_level::{LogLevel, Logger};

pub struct ConsoleLogger {
    log_level: LogLevel,
    use_colors: bool,
}

impl ConsoleLogger {
    pub fn new(log_level: LogLevel, use_colors: bool) -> ConsoleLogger {
        ConsoleLogger {
            log_level,
            use_colors,
        }
    }

    fn should_log(&self, log_level: &LogLevel) -> bool {
        if (*log_level as i32) > (self.log_level as i32) {
            return false;
        }
        true
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

impl Logger for ConsoleLogger {
    fn log(&self, log_level: &LogLevel, message: &str) {
        if !self.should_log(log_level) {
            return;
        }
        let level_name = log_level.to_string().to_uppercase();

        let formatted_message = format!("[{}] {}", level_name, message);
        let final_message = self.compute_colored_message(&formatted_message, log_level);

        match log_level {
            LogLevel::Error => eprintln!("{}", final_message),
            _ => println!("{}", final_message),
        }
    }

    fn log_message(&self, message: &str) {
        self.log(&LogLevel::Message, message);
    }

    fn log_warning(&self, warning: &str) {
        self.log(&LogLevel::Warning, warning);
    }

    fn log_error(&self, error: &str) {
        self.log(&LogLevel::Error, error);
    }
}