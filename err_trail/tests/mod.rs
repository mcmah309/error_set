#[cfg(feature = "tracing")]
#[cfg(test)]
mod tracing {
    use err_trail::{ErrContext, NoneContext};
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_error_context() {
        let result: Result<(), &str> = Err("error");
        let _ = result.error_context("An error occurred");

        assert!(logs_contain("An error occurred"));
    }

    #[traced_test]
    #[test]
    fn test_warn_context() {
        let result: Result<(), &str> = Err("warning");
        let _ = result.warn_context("A warning occurred");

        assert!(logs_contain("A warning occurred"));
    }

    #[traced_test]
    #[test]
    fn test_with_error_context() {
        let result: Result<(), &str> = Err("error");
        let _ = result.with_error_context(|e| format!("An error occurred: `{}`", e));

        assert!(logs_contain("An error occurred: `error`"));
    }

    #[traced_test]
    #[test]
    fn test_with_warn_context() {
        let result: Result<(), &str> = Err("warning");
        let _ = result.with_warn_context(|e| format!("A warning occurred: `{}`", e));

        assert!(logs_contain("A warning occurred: `warning`"));
    }

    #[traced_test]
    #[test]
    fn test_consume_as_error() {
        let result: Result<(), &str> = Err("consumed error");
        let _ = result.consume_as_error();

        assert!(logs_contain("consumed error"));
    }

    #[traced_test]
    #[test]
    fn test_consume_as_warn() {
        let result: Result<(), &str> = Err("consumed warning");
        let _ = result.consume_as_warn();

        assert!(logs_contain("consumed warning"));
    }

    #[traced_test]
    #[test]
    fn test_option_error_context() {
        let option: Option<()> = None;
        let _ = option.error_context("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[traced_test]
    #[test]
    fn test_option_warn_context() {
        let option: Option<()> = None;
        let _ = option.warn_context("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[traced_test]
    #[test]
    fn test_option_with_error_context() {
        let option: Option<()> = None;
        let _ = option.with_error_context(|| "Lazy error context");

        assert!(logs_contain("Lazy error context"));
    }

    #[traced_test]
    #[test]
    fn test_option_with_warn_context() {
        let option: Option<()> = None;
        let _ = option.with_warn_context(|| "Lazy warn context");

        assert!(logs_contain("Lazy warn context"));
    }
}

#[cfg(feature = "log")]
#[cfg(test)]
mod log {
    use err_trail::{ErrContext, NoneContext};
    use lazy_static::lazy_static;
    use log::{Level, Metadata, Record};
    use std::sync::{Arc, Mutex};

    struct TestLogger {
        logs: Arc<Mutex<Vec<String>>>,
    }

    impl log::Log for TestLogger {
        fn enabled(&self, metadata: &Metadata) -> bool {
            metadata.level() <= Level::Trace
        }

        fn log(&self, record: &Record) {
            if self.enabled(record.metadata()) {
                let mut logs = self.logs.lock().unwrap();
                logs.push(format!("{}", record.args()));
            }
        }

        fn flush(&self) {}
    }

    lazy_static! {
        static ref LOGS: Arc<Mutex<Vec<String>>> = {
            let logs = Arc::new(Mutex::new(Vec::new()));
            let test_logger = TestLogger { logs: logs.clone() };

            log::set_boxed_logger(Box::new(test_logger)).unwrap();
            log::set_max_level(log::LevelFilter::Trace);

            logs
        };
    }

    fn logs_contain(expected: &str) -> bool {
        let logs = LOGS.lock().unwrap();
        logs.iter().any(|log| log.contains(expected))
    }

    fn clear_logs() {
        let mut logs = LOGS.lock().unwrap();
        logs.clear();
    }

    #[test]
    fn test_error_context() {
        clear_logs();
        let result: Result<(), &str> = Err("error");
        let _ = result.error_context("An error occurred");

        assert!(logs_contain("An error occurred"));
    }

    #[test]
    fn test_warn_context() {
        clear_logs();
        let result: Result<(), &str> = Err("warning");
        let _ = result.warn_context("A warning occurred");

        assert!(logs_contain("A warning occurred"));
    }

    #[test]
    fn test_with_error_context() {
        clear_logs();
        let result: Result<(), &str> = Err("error");
        let _ = result.with_error_context(|e| format!("An error occurred: `{}`", e));

        assert!(logs_contain("An error occurred: `error`"));
    }

    #[test]
    fn test_with_warn_context() {
        clear_logs();
        let result: Result<(), &str> = Err("warning");
        let _ = result.with_warn_context(|e| format!("A warning occurred: `{}`", e));

        assert!(logs_contain("A warning occurred: `warning`"));
    }

    #[test]
    fn test_consume_as_error() {
        clear_logs();
        let result: Result<(), &str> = Err("consumed error");
        let _ = result.consume_as_error();

        assert!(logs_contain("consumed error"));
    }

    #[test]
    fn test_consume_as_warn() {
        clear_logs();
        let result: Result<(), &str> = Err("consumed warning");
        let _ = result.consume_as_warn();

        assert!(logs_contain("consumed warning"));
    }

    #[test]
    fn test_option_error_context() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.error_context("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[test]
    fn test_option_warn_context() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.warn_context("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[test]
    fn test_option_with_error_context() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.with_error_context(|| "Lazy error context");

        assert!(logs_contain("Lazy error context"));
    }

    #[test]
    fn test_option_with_warn_context() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.with_warn_context(|| "Lazy warn context");

        assert!(logs_contain("Lazy warn context"));
    }
}
