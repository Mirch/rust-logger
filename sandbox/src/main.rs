use rust_logger::{console_logger::*, log_level::*};

fn main() {
    let logger = ConsoleLogger::new(LogLevel::Message, true);

    logger.log_message("This is a message!");
    logger.log_warning("This is a warning!");
    logger.log_error("This is an error!");
}
