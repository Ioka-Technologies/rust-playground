use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone, Copy)]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

pub trait Logger {
    fn out(&self, message: String);
}

struct LoggingService {
    level: LogLevel,
    loggers: Vec<Rc<dyn Logger>>,
}

impl LoggingService {
    fn new(level: LogLevel) -> LoggingService {
        return LoggingService {
            level,
            loggers: Vec::new(),
        };
    }

    fn set_level(&'_ mut self, level: LogLevel) -> &'_ mut Self {
        self.level = level;
        self
    }

    fn add_logger(&'_ mut self, logger: Rc<dyn Logger>) -> &'_ mut Self {
        self.loggers.push(logger);
        self
    }

    fn log(&self, level: LogLevel, message: &str) {
        if level as i32 >= self.level as i32 {
            for logger in &self.loggers {
                logger.out(format!("{:?}: {}", level, message).to_string());
            }
        }
    }
}

struct TestLogger {
    current_message: RefCell<String>
}

impl TestLogger {
    fn new() -> TestLogger {
        TestLogger { current_message: RefCell::new("".to_string()) }
    }
}

impl Logger for TestLogger {
    fn out(&self, message: String) {
        *self.current_message.borrow_mut() = message;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger() {
        let test_logger = Rc::new(TestLogger::new());

        let mut logging_service = LoggingService::new(LogLevel::Debug);

        logging_service.add_logger(test_logger.clone()).set_level(LogLevel::Debug);

        logging_service.log(LogLevel::Debug, "This is a debug message");

        assert_eq!(test_logger.current_message.borrow().as_str(), "Debug: This is a debug message");

        logging_service.set_level(LogLevel::Info);

        logging_service.log(LogLevel::Info, "This is an info message");

        assert_eq!(test_logger.current_message.borrow().as_str(), "Info: This is an info message");

        logging_service.log(LogLevel::Warn, "This is a warning message");

        assert_eq!(test_logger.current_message.borrow().as_str(), "Warn: This is a warning message");

        logging_service.log(LogLevel::Error, "This is an error message");

        assert_eq!(test_logger.current_message.borrow().as_str(), "Error: This is an error message");
    }
}
