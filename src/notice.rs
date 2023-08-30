use num_enum::{TryFromPrimitive, IntoPrimitive};
use serde::{Deserialize, Serialize};

use crate::compute;

#[derive(Debug, TryFromPrimitive, IntoPrimitive, Eq, PartialEq)]
#[repr(u8)]
pub enum NeedRequest {
    Computation  = 1, // I need compute resources
    Verification = 2, // I need to verify a computation
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    Compute(compute::Offer),
    Verify,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    DeclineOffer,
    Compute(compute::Job),
    Verify,
}