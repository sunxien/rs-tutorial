#[cfg(test)]
pub mod logs_test_cases {
    use std::env;

    use env_logger::{Builder, Env};
    use log::{debug, error, info, Level, log, trace, warn};

    /// Rust Doc: https://course.rs/logs/log.html
    #[test]
    pub fn test_log() {
        //
        // println!("[test_log] env::args={:#?}", env::args());
        // println!("[test_log] env::vars={:#?}", env::vars());

        unsafe { env::set_var("RUST_LOG", "warn"); }

        env_logger::init();

        debug!("this is a debug {}", "message");
        error!("this is a error {} that printed by default", "message");
        info!("this is a info {}", "message");
        trace!("this is a trace {}", "message");
        warn!("this is a warn {}", "message");

        log!(target: "rs-tutorial", Level::Info, "Hello {}", "Eric");

        // TODO How to format log content?
        // TODO How to redirect stderr & stdout?
        // TODO How to set log output path? (file_name, compress and etc)
    }
}

/// No `main` function found in crate `logs` [EO601]
fn main() {}