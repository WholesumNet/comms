use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeType {
    // the compute is prove and lift
    ProveAndLift,
    
    // the compute is join
    Join,

    // the compute is snark extraction(r0 identity_p254 + compress)
    Snark,
}

// criteria for job seekers
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Criteria {
    // the actual type compute needed
    pub compute_type: ComputeType,
}

// used by clients when gossiping about compute needs
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeedCompute {
    pub criteria: Criteria,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ServerSpecs {
    pub gflops: u16,            //@ fp64 * 10,000?
    pub memory_capacity: u32,   // in GB
    pub cpu_model: String,
}

// the offer as server makes for a compute need
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Offer {
    pub compute_type: ComputeType,
    pub hw_specs: ServerSpecs,                  
    pub price: u8,                              
}


// input data for the job
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeJobInputType {
    // cid of the segment to be proved and lifted
    Prove(String),

    // cid of the left and right succinct receipts
    Join(String, String),
}

// the actual compute job 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeDetails {
    pub job_id: String,
    
    pub compute_type: ComputeType,
    
    pub input_type: ComputeJobInputType,
}

// 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationDetails {
    pub job_id: String,
    pub risc0_image_id: String,       // image_id(merkle root of) as in Risc0    
    pub receipt_cid: String,          // receipt to verify against as in Risc0
    //@ no need for pod_name here, could be hard-coded to 'receipt'
    pub pod_name: String,             // pod where the receipt is located in FairOS-dfs
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarvestDetails {
    pub fd12_cid: Option<String>,
    pub receipt_cid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {
    // running, aka proving
    Running,

    // finished with error, error message as param
    ExecutionFailed(Option<String>),    

    // waiting to be verified, receipt cid as param
    ExecutionSucceeded(String),  

    // harvested, cid of fd12, logs, ... as params
    Harvested(HarvestDetails),  
}

// servers update clients about latest developments of jobs 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobUpdate {
    pub id: String,
    pub compute_type: ComputeType,
    pub status: JobStatus,
}

