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
pub struct ComputeDetails {
    pub job_id: String,
    pub docker_image: String,
    pub command: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationDetails {
    pub job_id: String,
    pub image_id: String,       // image_id(merkle root of) as in Risc0    
    pub receipt_cid: String,    // receipt to verify against as in Risc0
    pub pod_name: String,       // pod where the receipt is located in FairOS-dfs
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarvestDetails {
    pub fd12_cid: Option<String>,
    pub receipt_cid: Option<String>,
    //@ more fields TBD
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    // running, aka proving
    Running,

    // finished with error, cid of fd12(stdout - "unix fd 1" and stderr - "unix fd 2") as param
    ExecutionFailed(Option<String>),    

    // waiting to be verified, receipt cid as param
    ExecutionSucceeded(Option<String>),       

    // receipt_cid as param
    VerificationFailed(String),

    // receipt_cid as param
    VerificationSucceeded(String),

    // harvested, cid of fd12, logs, ... as params
    Harvested(HarvestDetails),  
}

// servers update clients about latest developments of jobs 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobUpdate {
    pub id: String,
    pub status: JobStatus,
}

