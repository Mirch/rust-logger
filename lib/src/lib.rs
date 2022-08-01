pub mod error;

use colored::{ColoredString, Colorize};
use core::fmt;
use error::*;
use std::{
    fs::{self, File, OpenOptions},
    io::Write,
};

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

pub trait Target {
    fn log(&self, message: &str, log_level: &LogLevel) -> Result<(), Error>;
}

pub struct ConsoleTarget {
    use_colors: bool,
}

impl Target for ConsoleTarget {
    fn log(&self, message: &str, log_level: &LogLevel) -> Result<(), Error> {
        let final_message = self.compute_colored_message(&message, log_level);

        match log_level {
            LogLevel::Error => eprintln!("{}", final_message),
            _ => println!("{}", final_message),
        };
        Ok(())
    }
}

impl ConsoleTarget {
    pub fn new(use_colors: bool) -> Box<ConsoleTarget> {
        Box::new(ConsoleTarget { use_colors })
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

pub struct FileTarget {
    file_path: String,
}

impl Target for FileTarget {
    fn log(&self, message: &str, log_level: &LogLevel) -> Result<(), Error> {
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.file_path);

        let error = Err(Error {
            message: String::from("Could not open and write to file."),
        });

        match file {
            Ok(mut result) => {
                if let Err(e) = writeln!(result, "{message}") {
                    return error;
                }
                Ok(())
            }
            Err(_) => {
                return error;
            }
        }
    }
}

impl FileTarget {
    pub fn new(file_path: &str) -> Box<FileTarget> {
        Box::new(FileTarget {
            file_path: String::from(file_path),
        })
    }
}

pub struct Logger {
    log_level: LogLevel,
    target: Box<dyn Target>,
}

impl Logger {
    pub fn new(log_level: LogLevel, target: Box<dyn Target>) -> Logger {
        Logger { log_level, target }
    }

    fn log(&self, log_level: &LogLevel, message: &str) -> Result<(), Error> {
        if !self.should_log(log_level) {
            return Ok(());
        }

        let level_name = log_level.to_string().to_uppercase();
        let formatted_message = format!("[{}] {}", level_name, message);

        self.target.log(&formatted_message, log_level)?;
        Ok(())
    }

    fn should_log(&self, log_level: &LogLevel) -> bool {
        if (*log_level as i32) > (self.log_level as i32) {
            return false;
        }
        true
    }

    pub fn log_message(&self, message: &str) -> Result<(), Error> {
        self.log(&LogLevel::Message, message)?;
        Ok(())
    }

    pub fn log_warning(&self, warning: &str) -> Result<(), Error> {
        self.log(&LogLevel::Warning, warning)?;
        Ok(())
    }

    pub fn log_error(&self, error: &str) -> Result<(), Error> {
        self.log(&LogLevel::Error, error)?;
        Ok(())
    }
}
