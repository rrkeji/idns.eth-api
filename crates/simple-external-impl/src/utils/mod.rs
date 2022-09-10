use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Deserialize, Serialize)]
pub struct ObjectId {
    //$oid
    #[serde(rename(deserialize = "$oid"))]
    pub oid: String,
}
