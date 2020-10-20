//! # The client component
//!
//!

use crate::base::Connection;
use crate::jobs::Job;
use log::{debug, error, warn};
use reqwest;
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

pub struct RomadClient {
    base_url: String,
    client: Client,
}

impl RomadClient {
    /// New
    pub fn new(
        address: &'static str,
        port: &'static str,
        token: std::option::Option<&'static str>,
    ) -> Result<RomadClient, RomadClientError> {
        let connection: Connection = Connection {
            address,
            port,
            token,
            timeout: 0,
            version: "1",
        };

        RomadClient::from_connection(connection)
    }

    /// Create client from a connection object
    pub fn from_connection(connection: Connection) -> Result<RomadClient, RomadClientError> {
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

        Ok(RomadClient { base_url, client })
    }
    /// Get base url
    pub fn get_base_url(&self) -> &String {
        &self.base_url
    }

    /// Get the http client for then object
    pub fn get_client(&self) -> &Client {
        &self.client
    }

    async fn execute(
        &mut self,
        url: &String,
        prefix: &Option<&String>,
        namespace: &Option<&String>,
    ) -> Result<String, reqwest::Error> {
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

        let client = self.get_client();
        let response = client.get(url).send().await?;
        response.text().await
    }

    /// Method calls the [List Jobs](https://www.nomadproject.io/api-docs/jobs#list-jobs)
    /// endpoint
    /// TODO: Implement prefix and namespace filter
    pub async fn list_jobs(
        &mut self,
        prefix: Option<&String>,
        namespace: Option<&String>,
    ) -> Result<Vec<Job>, RomadClientError> {
        // Construct the url from th base
        let url: String = format!("{}/jobs", self.base_url);

        debug!("Listing jobs using url {}", url);
        match self.execute(&url, &namespace, &prefix).await {
            Ok(text) => {
                println!("List jobs response: {}", text);
                let jobs: Vec<Job> = match serde_json::from_str(&text) {
                    Ok(jobs) => jobs,
                    Err(e) => {
                        return Err(RomadClientError::new(format!(
                            "Unable to convert string to jobs {}",
                            e
                        )))
                    }
                };
                Ok(jobs)
            }
            Err(e) => return Err(RomadClientError::new(format!("Unable to list jobs {}", e))),
        }
    }

    pub async fn list_allocations(
        &mut self,
        prefix: Option<&String>,
        namespace: Option<&String>,
    ) -> Result<String, RomadClientError> {
        let url: String = format!("{}/allocations", self.base_url);

        debug!("Listing allocations using url {}", url);
        match self.execute(&url, &namespace, &prefix).await {
            Ok(text) => Ok(text),
            Err(e) => return Err(RomadClientError::new(format!("Unable to list jobs {}", e))),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_default_client() {
        let client: RomadClient = RomadClient::new("http://localhost", "4646", None).unwrap();

        assert_eq!(client.get_base_url(), "http://localhost:4646/v1");
    }
}
