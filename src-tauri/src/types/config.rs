use serde::{Deserialize, Serialize};

// the user-defined configuration for the trin node,
// this is passed from the frontend
#[derive(Serialize, Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct TrinConfig {
    // args received from the frontend must be camelCase
    // there might be a better way to handle these cases though
    pub httpPort: usize,
    pub storage: usize,
    pub trustedBlockRoot: String,
}
