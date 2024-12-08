use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Operation {
    ADD,
    UPDATE,
    DELETE,
}
