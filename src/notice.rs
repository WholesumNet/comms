use serde::{Deserialize, Serialize};

use crate::compute;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Notice {
    // need compute resources
    Compute(compute::NeedCompute),    

    // update me on my jobs
    StatusUpdate(u8),

    // need to verify some computation
    Verification(u8),                     

    // need to harvest residue objects: stdout, stderr, logs, output data, ...
    Harvest(u8),                       
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {

    // servers put benchmark + hw specs + $ rates into the offer
    ComputeOffer(compute::Offer),

    // job's status has been updated
    UpdateForJobs(Vec<compute::JobUpdate>),

    // benchmarks checks are passed, so get this compute job done
    ComputeJob(compute::ComputeDetails),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    //@ is this even necessary? because "timeout === offer decline"
    DeclinedOffer, 

    VerificationJob(compute::VerificationDetails),
}
