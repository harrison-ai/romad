//! # The base library components
//!
//!
use std::error::Error;
use std::fmt;

/// Base Error for Crate
#[derive(Debug, Clone)]
pub struct RomadError {
    msg: String,
}

impl RomadError {
    pub fn new(msg: String) -> RomadError {
        RomadError { msg: msg }
    }
}

impl fmt::Display for RomadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RomadError: {}", self.msg)
    }
}

impl Error for RomadError {}

/// Connection object
/// Default values http://localhost:4646/v1
pub struct Connection<'a> {
    pub address: &'a str,
    pub port: &'a str,
    pub token: Option<&'a str>, // TODO: Token authentication
    pub timeout: isize,
    pub version: &'a str,
}

impl Default for Connection<'_> {
    /// Default implementation for a localhost nomad instance
    /// ```
    /// use romad::base::Connection;
    ///
    /// let con: Connection = Default::default();
    /// ```
    fn default() -> Self {
        Connection {
            address: "http://localhost",
            port: "4646",
            token: None,
            timeout: 0,
            version: "1",
        }
    }
}

impl Connection<'_> {
    /// Build the base url for the connection
    /// ```
    /// use romad::base::Connection;
    ///
    /// let con: Connection = Default::default();
    /// let url = con.build_base_url();
    /// assert_eq!("http://localhost:4646/v1".to_string(), url);
    /// ```
    pub fn build_base_url(&self) -> String {
        format!("{}:{}/v{}", self.address, self.port, self.version)
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_build_base_url() {
        let con: Connection = Default::default();

        let expected_url = "http://localhost:4646/v1".to_string();

        assert_eq!(expected_url, con.build_base_url());
    }
}
