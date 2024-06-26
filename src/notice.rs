// use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use crate::compute;

// #[derive(Debug, TryFromPrimitive, IntoPrimitive, Eq, PartialEq)]
// #[repr(u8)]
#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Notice {
    Compute(compute::NeedCompute),    // need compute resources
    Verification,                     // need to verify some computation
    Harvest,                          // need to harvest residue objects: stdout, stderr, logs, output data, ...
    JobStatus,                        // need to get updates on my job's status
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    ComputeOffer(compute::Offer),              // I have compute resources
    VerificationOffer,                         // I would verify
    UpdateForJobs(Vec<compute::JobUpdate>),    // job's status has been updated
    ComputeJob(compute::ComputeDetails),       // Run this job for me hey degen server
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    //@ is this even necessary? timeout is equivalent to offer decline
    DeclinedOffer, 
    ComputeJob(compute::ComputeDetails),
    VerificationJob(compute::VerificationDetails),
}
