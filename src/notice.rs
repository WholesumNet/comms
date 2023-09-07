use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use crate::compute;

#[derive(Debug, TryFromPrimitive, IntoPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum Notice {
    Compute   = 1, // I need compute resources
    Verify    = 2, // I need to verify a computation
    JobStatus = 3,    // I need to get my job's status
}

// servers make requests
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    Compute(compute::Offer),
    Verify,
    JobResult(compute::JobResult),
}

// clients respond to server requests
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    DeclineOffer,
    Compute(compute::Job),
    Verify,
}