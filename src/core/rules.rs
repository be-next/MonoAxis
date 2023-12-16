use serde::{Deserialize, Serialize};
// use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct TransitionRule {
    neighborhood: [u8; 3],
    pub next_state: u8,
}

#[derive(Serialize, Deserialize)]
pub struct TransitionRules {
    num_states: u8,
    pub rules: Vec<TransitionRule>,
}