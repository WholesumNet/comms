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
pub struct Job {
    pub id: String,    //@ appears that cbor cannot serialize u128!
    pub details: JobDetails,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    Running,                     // running, aka proving
    FinishedWithError(String),   // finished with error
    ToBeVerified,                // finished running, waiting to be verified
    ToBeCollected,               // finished running and verification, waiting to be collected
    Unknown,                     // have no idea?
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobResult {
    pub id: String,
    pub status: JobStatus,
}

