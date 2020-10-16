//! Module to manage nomad jobs
//!

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug, Eq, PartialEq, Hash, Deserialize, Serialize)]
pub enum RunningStatus {
    Queued,
    Complete,
    Failed,
    Running,
    Starting,
    Lost,
    Pending,
    Dead,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct Job {
    pub ID: String,
    pub ParentID: String,
    pub Namespace: String,
    pub Datacenters: Vec<String>,
    pub MultiRegion: Option<String>,
    pub Type: String,
    pub Priority: isize,
    pub Periodic: bool,
    pub ParameterizedJob: bool,
    pub Stop: bool,
    pub Status: String,
    pub StatusDescription: String,
    pub JobSummary: JobSummary,
    pub CreateIndex: u16,
    pub ModifyIndex: u16,
    pub JobModifyIndex: u16,
    pub SubmitTime: u128,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize)]
pub struct JobSummary {
    pub JobID: String,
    pub Namespace: String,
    pub Summary: HashMap<String, HashMap<RunningStatus, isize>>,
    pub Children: HashMap<RunningStatus, u16>,
    pub CreateIndex: u16,
    pub ModifyIndex: u16,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs::File;

    #[test]
    fn test_job_from_str() {
        let file = File::open("./tests/jobs_response.json").unwrap();

        let job: Job = serde_json::from_reader(file).unwrap();

        // Test select attributes
        assert_eq!(job.ID, "example".to_string());
        assert_eq!(job.Datacenters, ["dc1".to_string()]);
        assert_eq!(job.Priority, 50);
        assert_eq!(job.ParameterizedJob, false);

        assert_eq!(
            job.JobSummary
                .Summary
                .get("cache")
                .unwrap()
                .get(&RunningStatus::Running)
                .unwrap(),
            &1
        );
    }
}
