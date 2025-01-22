use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProveAndLiftDetails {    
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
pub struct JoinDetails {
    pub pairs: Vec<(String, String)>,

    // hints for provers to know what pairs to join
    // the Nth bit is 1 => pair-N is already proved
    pub progress_map: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Groth16Details {
    // stark proof's cid
    pub cid: String    
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobType {
    ProveAndLift(ProveAndLiftDetails),
    
    Join(JoinDetails),

    Groth16(Groth16Details),
}

// used by clients when gossiping about compute needs
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComputeJob {    
    // network-wide id of the job
    pub job_id: String,

    // whether it's prove, join, or groth16
    pub job_type: JobType,

    pub budget: u32
}

#[derive(Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum Need {
    // need compute resources
    Compute(ComputeJob),

    // update me on my jobs
    UpdateMe(u8),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProofType {
    // param: segment id
    ProveAndLift(u32),

    // param: left & right proof cids
    Join(String, String),

    Groth16
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Proof {
    pub job_id: String,

    pub proof_type: ProofType,

    pub cid: String
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Request {
    // job's status has been updated
    ProofIsReady(Vec<Proof>),
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Response {
    Unknown, 
}
