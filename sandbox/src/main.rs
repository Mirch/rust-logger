use rust_logger::*;

fn main() -> Result<(), error::Error> {
    let console_target = ConsoleTarget::new(true);
    let file_target = FileTarget::new("./test.txt");

    let console_logger = Logger::new(LogLevel::Message, console_target);
    let file_logger = Logger::new(LogLevel::Message, file_target);
    let loggers = vec![console_logger, file_logger];

    for logger in loggers {
        logger.log_message("This is a message!")?;
        logger.log_warning("This is a warning!")?;
        logger.log_error("This is an error!")?;
    }
    Ok(())
}
