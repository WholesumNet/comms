use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use crate::compute;

#[derive(Debug, TryFromPrimitive, IntoPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum Notice {
    Compute = 0,    // I need compute resources
    Verification,         // I need to verify some computation
    JobStatus,      // I need to get updates on my job's status
}

// servers make requests
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    ComputeOffer(compute::Offer),              // I have compute resources
    VerificationOffer,                         // I would verify
    UpdateForJobs(Vec<compute::JobUpdate>),    // job's status has been updated
}

// clients respond to server requests
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    DeclinedOffer,
    ComputeJob(compute::ComputeDetails),
    VerificationJob(compute::VerificationDetails),
}
