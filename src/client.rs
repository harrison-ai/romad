//! # The client component
//!
//!

use crate::base::Connection;
use crate::jobs::Job;
use log::{debug, error, warn};
use reqwest::Client;
use std::fmt;

/// Client Error
#[derive(Debug, Clone)]
pub struct RomadClientError {
    msg: String,
}

impl RomadClientError {
    pub fn new(msg: String) -> RomadClientError {
        RomadClientError { msg: msg }
    }
}

impl fmt::Display for RomadClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RomadClientError: {}", self.msg)
    }
}

pub struct RomadClient<'a> {
    connection: Connection<'a>,
    base_url: String,
    client: Client,
}

impl<'a> RomadClient<'a> {
    /// New
    pub fn new(
        address: &'a str,
        port: &'a str,
        token: Option<&'a str>,
        timeout: isize,
    ) -> Result<RomadClient<'a>, RomadClientError> {
        let connection: Connection = Connection {
            address,
            port,
            token,
            timeout,
            version: "1",
        };

        RomadClient::from_connection(connection)
    }

    /// Create client from a connection object
    pub fn from_connection(
        connection: Connection<'a>,
    ) -> Result<RomadClient<'a>, RomadClientError> {
        let base_url = connection.build_base_url();

        let client = match Client::builder().build() {
            Ok(client) => client,
            Err(e) => {
                return {
                    error!("Error building client: {}", e);
                    Err(RomadClientError::new(format!("Error building client")))
                }
            }
        };

        Ok(RomadClient {
            connection,
            base_url,
            client,
        })
    }
    /// Get base url
    pub fn get_base_url(&self) -> &String {
        &self.base_url
    }

    /// Get the http client for then object
    pub fn get_client(&self) -> &Client {
        &self.client
    }

    /// Function calls the [List Jobs](https://www.nomadproject.io/api-docs/jobs#list-jobs)
    /// endpoint
    /// TODO: Implement prefix and namespace filter
    pub async fn list_jobs(
        &mut self,
        prefix: Option<&String>,
        namespace: Option<&String>,
    ) -> Result<Vec<Job>, RomadClientError> {
        let mut jobs: Vec<Job> = Vec::new();

        let client = self.get_client();

        // Construct the url from th base
        let url: String = format!("{}/jobs", self.base_url);

        match namespace {
            Some(_) => {
                warn!("Listing jobs by namespace is not yet implemented, defaulting to no specified namespace")
            }
            None => {}
        }

        match prefix {
            Some(_) => {
                warn!("Listing jobs by prefix is not yet implemented, defaulting to no prefix")
            }
            None => {}
        }

        debug!("Listing jobs using url {}", url);

        let response = match client.get(&url).send().await {
            Ok(resp) => resp,
            Err(e) => return Err(RomadClientError::new(format!("{}", e))),
        };

        debug!("List jobs response headers: {:?}", response);
        match response.text().await {
            Ok(text) => {
                println!("List jobs response: {}", text);
                let jobs: Vec<Job> = serde_json::from_str(&text).unwrap();
                Ok(jobs)
            }
            Err(e) => return Err(RomadClientError::new(format!("{}", e))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_client() {
        let client: RomadClient = RomadClient::new("localhost", "4646", None, 0).unwrap();

        assert_eq!(client.get_base_url(), "http://localhost:4646/v1");
    }
}
