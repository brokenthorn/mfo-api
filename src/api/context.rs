/// Shared state for all HTTP requests.
///
/// Use it for things like database connection pools and other shared state.
#[derive(Debug, Clone)]
pub struct Context {
    pub db: String,
}
