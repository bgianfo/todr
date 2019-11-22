//! This module implements the todr configuration logic.
use std::env;


pub enum Configuration { }


impl Configuration {
    /// Obtain Todoist authentication token.
    pub fn api_token() -> String {
        env::var("TODR_AUTHTOKEN").expect("TODR_AUTHTOKEN is not set")
    }
}
