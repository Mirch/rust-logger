mod console_logger;

use console_logger::{ConsoleLogger, LogLevel};

fn main() {
    let logger = ConsoleLogger::new(LogLevel::Error, true);

    logger.log_message("This is a message!");
    logger.log_warning("This is a warning!");
    logger.log_error("This is an error!");
}
