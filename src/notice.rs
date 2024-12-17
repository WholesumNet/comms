use serde::{Deserialize, Serialize};

use crate::compute;

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Notice {

    // need compute resources
    Compute(compute::NeedCompute),    

    // update me on my job
    UpdateMe(u8),                     
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {

    // job's status has been updated
    Update(Vec<compute::JobUpdate>),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {

    //@ is this even necessary? because "timeout === offer decline"
    DeclinedOffer, 
}
