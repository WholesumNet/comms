use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifyDetails {
    pub job_id: String,
    pub image_id: Vec<u8>,    // image_id(merkle root of) as in risc0    
    pub receipt_cid: String,    
}