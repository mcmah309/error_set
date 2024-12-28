#[cfg(feature = "tracing")]
#[cfg(test)]
mod tracing {
    use err_trail::{ErrContext, ErrContextDisplay, NoneContext};
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
    fn test_consume_with_error() {
        let result: Result<(), &str> = Err("");
        let _ = result.consume_with_error(|_| "consumed with error");

        assert!(logs_contain("consumed with error"));
    }

    #[traced_test]
    #[test]
    fn test_consume_with_warn() {
        let result: Result<(), &str> = Err("");
        let _ = result.consume_with_warn(|_| "consumed with warn");

        assert!(logs_contain("consumed with warn"));
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

    #[traced_test]
    #[test]
    fn test_info_context() {
        let result: Result<(), &str> = Err("info");
        let _ = result.info_context("An info occurred");

        assert!(logs_contain("An info occurred"));
    }

    #[traced_test]
    #[test]
    fn test_debug_context() {
        let result: Result<(), &str> = Err("debug");
        let _ = result.debug_context("A debug occurred");

        assert!(logs_contain("A debug occurred"));
    }

    #[traced_test]
    #[test]
    fn test_trace_context() {
        let result: Result<(), &str> = Err("trace");
        let _ = result.trace_context("A trace occurred");

        assert!(logs_contain("A trace occurred"));
    }

    #[traced_test]
    #[test]
    fn test_with_info_context() {
        let result: Result<(), &str> = Err("info");
        let _ = result.with_info_context(|e| format!("An info occurred: `{}`", e));

        assert!(logs_contain("An info occurred: `info`"));
    }

    #[traced_test]
    #[test]
    fn test_with_debug_context() {
        let result: Result<(), &str> = Err("debug");
        let _ = result.with_debug_context(|e| format!("A debug occurred: `{}`", e));

        assert!(logs_contain("A debug occurred: `debug`"));
    }

    #[traced_test]
    #[test]
    fn test_with_trace_context() {
        let result: Result<(), &str> = Err("trace");
        let _ = result.with_trace_context(|e| format!("A trace occurred: `{}`", e));

        assert!(logs_contain("A trace occurred: `trace`"));
    }

    #[traced_test]
    #[test]
    fn test_option_info_context() {
        let option: Option<()> = None;
        let _ = option.info_context("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[traced_test]
    #[test]
    fn test_option_debug_context() {
        let option: Option<()> = None;
        let _ = option.debug_context("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[traced_test]
    #[test]
    fn test_option_trace_context() {
        let option: Option<()> = None;
        let _ = option.trace_context("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[traced_test]
    #[test]
    fn test_option_with_info_context() {
        let option: Option<()> = None;
        let _ = option.with_info_context(|| "Lazy info context");

        assert!(logs_contain("Lazy info context"));
    }

    #[traced_test]
    #[test]
    fn test_option_with_debug_context() {
        let option: Option<()> = None;
        let _ = option.with_debug_context(|| "Lazy debug context");

        assert!(logs_contain("Lazy debug context"));
    }

    #[traced_test]
    #[test]
    fn test_option_with_trace_context() {
        let option: Option<()> = None;
        let _ = option.with_trace_context(|| "Lazy trace context");

        assert!(logs_contain("Lazy trace context"));
    }
}

#[cfg(feature = "log")]
#[cfg(test)]
mod log {
    use err_trail::{ErrContext, ErrContextDisplay, NoneContext};
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
    fn test_consume_with_error() {
        clear_logs();
        let result: Result<(), &str> = Err("");
        let _ = result.consume_with_error(|_| "consumed with error");

        assert!(logs_contain("consumed with error"));
    }

    #[test]
    fn test_consume_with_warn() {
        clear_logs();
        let result: Result<(), &str> = Err("");
        let _ = result.consume_with_warn(|_| "consumed with warn");

        assert!(logs_contain("consumed with warn"));
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

    #[test]
    fn test_info_context() {
        clear_logs();
        let result: Result<(), &str> = Err("info");
        let _ = result.info_context("An info occurred");

        assert!(logs_contain("An info occurred"));
    }

    #[test]
    fn test_debug_context() {
        clear_logs();
        let result: Result<(), &str> = Err("debug");
        let _ = result.debug_context("A debug occurred");

        assert!(logs_contain("A debug occurred"));
    }

    #[test]
    fn test_trace_context() {
        clear_logs();
        let result: Result<(), &str> = Err("trace");
        let _ = result.trace_context("A trace occurred");

        assert!(logs_contain("A trace occurred"));
    }

    #[test]
    fn test_with_info_context() {
        clear_logs();
        let result: Result<(), &str> = Err("info");
        let _ = result.with_info_context(|e| format!("An info occurred: `{}`", e));

        assert!(logs_contain("An info occurred: `info`"));
    }

    #[test]
    fn test_with_debug_context() {
        clear_logs();
        let result: Result<(), &str> = Err("debug");
        let _ = result.with_debug_context(|e| format!("A debug occurred: `{}`", e));

        assert!(logs_contain("A debug occurred: `debug`"));
    }

    #[test]
    fn test_with_trace_context() {
        clear_logs();
        let result: Result<(), &str> = Err("trace");
        let _ = result.with_trace_context(|e| format!("A trace occurred: `{}`", e));

        assert!(logs_contain("A trace occurred: `trace`"));
    }

    #[test]
    fn test_option_info_context() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.info_context("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[test]
    fn test_option_debug_context() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.debug_context("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[test]
    fn test_option_trace_context() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.trace_context("Option was none");

        assert!(logs_contain("Option was none"));
    }

    #[test]
    fn test_option_with_info_context() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.with_info_context(|| "Lazy info context");

        assert!(logs_contain("Lazy info context"));
    }

    #[test]
    fn test_option_with_debug_context() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.with_debug_context(|| "Lazy debug context");

        assert!(logs_contain("Lazy debug context"));
    }

    #[test]
    fn test_option_with_trace_context() {
        clear_logs();
        let option: Option<()> = None;
        let _ = option.with_trace_context(|| "Lazy trace context");

        assert!(logs_contain("Lazy trace context"));
    }
}
