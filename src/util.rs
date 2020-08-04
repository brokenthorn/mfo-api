//! Utility functions.

/// If RUST_LOG environment variable is not found, this function sets it to `RUST_LOG="warn,mfo_api=debug"`.
pub fn fix_rust_log_if_not_set() {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "off,mfo_api=info");
    }
}
