use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProveDetails {    
    // a typical segment cid: bafybeiccjcinml5w2meuhcnzu7gwlbkioy2dtyskulrspxoys6gikrrzae/segment-0

    // all segments are in a directory pointed to by the base cid
    pub segments_base_cid: String,

    // "segment-" in the ^ example 
    pub segment_prefix_str: String,

    // po2 limit of the segment, e.g. 19 requires 4gb memory to prove
    pub po2: u8,

    // number of segments, used by provers to access any segment(0 to num_segments - 1)
    pub num_segments: u32,

    // hints for provers to know what segments to prove
    // the Nth bit is 1 => segment-N is already proved
    pub progress_map: Vec<u8>
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JoinType {
    // when there is less than 32 pairs ~3.2kb
    Inline(Vec<(String, String)>),

    // cid of the json file that contains longer pair data
    // json file:
    //    [
    //      (l1, r1),
    //      (l2, r2),
    //      .
    //      .
    //      (lN, rN)
    //    ]
    Cid(String)
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JoinDetails {    
    // number of pairs to be joined
    pub num_joins: u32,

    pub join_type: JoinType,

    // hints for provers to know what pairs to join
    // the Nth bit is 1 => pair-N is already proved
    pub joined_map: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Groth16Details {
    // stark proof's cid
    pub cid: String    
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerificationDetails {
    // SuccinctReceipt's cid
    pub cid: String    
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ComputeType {
    ProveAndLift(ProveDetails),
    
    Join(JoinDetails),

    Groth16(Groth16Details),

    //@ reserved for future
    Verification(VerificationDetails),
}

// used by clients when gossiping about compute needs
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct NeedCompute {    
    // network-wide id of the job
    pub job_id: String,

    // whether it's prove, join, or snark
    pub compute_type: ComputeType,    

    pub budget: u32
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobStatus {    
    Running,

    // error message as param
    ExecutionFailed(Option<String>),    

    // receipt cid as param
    ExecutionSucceeded(String) 
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Item {
    // param: segment id
    Prove(u32),

    // param: left & right proof cids
    Join(String, String),

    Snark, 

    Verification
}

// servers update clients about latest developments of jobs 
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct JobUpdate {
    // job_id
    pub id: String,

    pub item: Item,

    pub status: JobStatus
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Notice {
    // need compute resources
    Compute(NeedCompute),    

    // update me on my job
    UpdateMe(u8),                     
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    // job's status has been updated
    Update(Vec<JobUpdate>),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    Unknown, 
}