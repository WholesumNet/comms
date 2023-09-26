use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerSpecs {
    pub gflops: u16,       //@ fp64 * 10,000?
    pub ram_amount: u64,   // mega bytes   
    pub cpu_model: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Offer {
    pub hw_specs: ServerSpecs, // hardware specs    
    pub price: u8,             // $/secs of usage rate
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobDetails {
    pub docker_image: String,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobContract {
    pub id: String,
    pub details: JobDetails,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    // running, aka proving
    Running,

    // finished with error, stderr and stdout resp. as params
    ExecutionFailed(Option<String>, Option<String>),    

    // finished running, waiting to be verified, receipt cid as param
    ReadyForVerification(String),       

    // verification failed, error as param
    VerificationFailed(String),         

    // verified, waiting to be collected
    ReadyToHarvest,                     
}

// servers update clients about latest developments of jobs 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobUpdate {
    pub id: String,
    pub status: JobStatus,
}

