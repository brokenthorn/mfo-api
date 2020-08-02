//! Utility functions.

/// If RUST_LOG environment variable is not found, this function sets it to `RUST_LOG="warn,mfo_api=debug"`.
pub fn fix_rust_log_if_not_set() {
    if std::env::var("RUST_LOG").is_err() {
        tide::log::warn!("RUST_LOG was not set so logging might not be configured as expected!");
        std::env::set_var("RUST_LOG", "mfo_api=debug,tiberius=error,warn");
    }
}
