use std::fmt::Debug;
use std::thread::sleep;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use tracing::{event, instrument, Level, span};
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

/// Rust Doc: https://course.rs/logs/tracing.html
#[allow(dead_code, unused)]
fn mock_io_operate_with_span() {
    let span = span!(Level::TRACE, "my_span");
    let _guard = span.enter(); // it will return a `RAII`, so what's `RAII`?
    sleep(Duration::from_secs(1));
    // TODO How to use span ?
}

/// Best Practice: Use EVENT in Span Context
/// Rust Doc: https://course.rs/logs/tracing.html
#[allow(dead_code, unused)]
fn mock_io_operate_with_event() {

    // Global `tracing_subscriber` can only be init once.
    // `tracing_subscriber` is a `Collector`, it will collect logs and output
    tracing_subscriber::registry().with(fmt::layer()).init();
    event!(Level::INFO, "this is my event outside my span...");

    let span = span!(Level::TRACE, "my_span");
    let _guard = span.enter(); // it will return a `RAII`, so what's `RAII`?
    sleep(Duration::from_secs(1));
    event!(Level::DEBUG, "this is my event define in my span");
}

/// Rust Doc: https://course.rs/logs/tracing.html
#[instrument]
#[allow(dead_code, unused)]
fn mock_io_operate_with_instrument() {
    event!(Level::INFO, "this is my event define in instrument...");
    sleep(Duration::from_secs(1));
    event!(Level::INFO, "this is my event define in instrument...");
}

#[instrument]
#[allow(dead_code, unused)]
pub fn call_parent() {
    event!(Level::INFO, "this is `call_parent` function...");
    sleep(Duration::from_secs(1));
    call_son();
}

#[instrument]
#[allow(dead_code, unused)]
pub fn call_son() {
    event!(Level::INFO, "this is `call_son` function...");
    sleep(Duration::from_secs(1));
    call_grandson();
}

#[instrument]
#[allow(dead_code, unused)]
pub fn call_grandson() {
    event!(Level::INFO, "this is `call_grandson` function...");
    sleep(Duration::from_secs(1));
}

#[allow(dead_code, unused)]
#[cfg(test)]
pub mod logs_test_cases {
    use std::env;

    use env_logger::{Builder, Env, Target};
    use log::{debug, error, info, Level, log, log_enabled, trace, warn};
    use log::Level::{Debug, Trace};
    use tracing::{event, info_span, span};
    use tracing_subscriber::fmt;
    use tracing_subscriber::layer::SubscriberExt;
    use tracing_subscriber::util::SubscriberInitExt;

    use crate::{call_parent, Emp, mock_io_operate_with_event, mock_io_operate_with_instrument, mock_io_operate_with_span};

    /// Rust Doc: https://course.rs/logs/log.html
    #[test]
    pub fn test_log() {
        /// Get arguments and variables from environment
        // println!("[test_log] env::args={:#?}", env::args());
        // println!("[test_log] env::vars={:#?}", env::vars());

        // Reset variables into environment
        unsafe { env::set_var("RUST_LOG", "trace"); }

        env_logger::init();

        trace!("this is a trace {}", "message");
        if log_enabled!(Trace) {
            trace!("if trace enabled, do trace log now....");
        }

        debug!("this is a debug {}", "message");
        if log_enabled!(target: "Global", Debug) {
            debug!("if debug enabled, do debug log now....");
        }

        info!("this is a info {}", "message");
        log!(target: "my_event", Level::Info, "this a info log for module: {}", "my_event");

        warn!("this is a warn {}", "message");
        error!("this is a error {} that printed by default", "message");
    }

    /// Rust Doc1: https://course.rs/logs/tracing.html
    /// Rust Doc2: https://course.rs/logs/tracing-logger.html
    #[test]
    pub fn test_custom_log() {
        let mut builder = Builder::from_default_env();
        builder.target(Target::Stdout); // redirect stderr to stdout
        builder.init();
        if cfg!(debug_assertions) {
            eprintln!("this operation[`eprintln!`] is redirect stderr to stdout");
        }


    }

    /// Tracing got three concepts: SPAN, EVENT, COLLECTOR
    #[test]
    pub fn test_trace() {
        // Global `tracing_subscriber` can only be init once.
        // `tracing_subscriber` is a `Collector`, it will collect logs and output
        tracing_subscriber::registry().with(fmt::layer()).init();
        log::info!("this log is generated from `log::info!` macro....");
        let salary = 100000i32;
        tracing::info!(salary, "this log is generated from `tracing::info!` macro....");
    }

    ///
    #[test]
    pub fn test_mock_io_operate() {
        mock_io_operate_with_span();
        mock_io_operate_with_event();
        mock_io_operate_with_instrument();

        // Use span `in_scope` in external lib/operate. e.g:
        // FIXME Doesn't work????
        let json_str = "
            {
                \"id\": 121001,
                \"name\": \"xien.sxe\"
            }
        ";
        let json_obj: Emp = info_span!("json.parse").in_scope(|| serde_json::from_str(json_str).unwrap());
        println!("JSON toObject: {:?}", json_obj);

        // call parent -> son -> grandson
        call_parent();

        // Avoid conflict with `log::Level`
        let span = span!(tracing::Level::INFO, "multi_span");
        event!(parent: &span, tracing::Level::INFO, "this is child log, and it's parent is multi_span");

        // TODO File Appender Doc: https://github.com/tokio-rs/tracing/tree/master/tracing-appender
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct Emp<'a> {
    id: u32,
    name: &'a str,
}

/// No `main` function found in crate `logs` [EO601]
fn main() {}