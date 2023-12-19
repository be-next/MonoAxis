use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TransitionRule {
    neighborhood: [u8; 3],
    pub next_state: u8,
}

#[derive(Serialize, Deserialize)]
pub struct TransitionRules {
    name: String,
    num_states: u8,
    pub rules: Vec<TransitionRule>,
}

#[derive(Debug)]
pub enum TransitionRulesError {
    JsonParseError(serde_json::Error),
    IoError(std::io::Error),
}

impl From<serde_json::Error> for TransitionRulesError {
    fn from(err: serde_json::Error) -> Self {
        TransitionRulesError::JsonParseError(err)
    }
}

impl From<std::io::Error> for TransitionRulesError {
    fn from(err: std::io::Error) -> Self {
        TransitionRulesError::IoError(err)
    }
}

impl TransitionRules {
    pub fn new_from_json_string(json_string: &str) -> Result<Self, TransitionRulesError> {
        let transition_rules: TransitionRules = serde_json::from_str(json_string)?;
        Ok(transition_rules)
    }

    pub fn new_from_json_file(file_path: &str) -> Result<Self, TransitionRulesError> {
        let json_string = std::fs::read_to_string(file_path)?;
        Self::new_from_json_string(&json_string)
    }

    pub fn get_num_states(&self) -> u8 {
        self.num_states
    }
}

// TransitionRules iterator implementation
pub struct TransitionRulesIter<'a> {
    transition_rules: &'a TransitionRules,
    index: usize,
}

impl<'a> IntoIterator for &'a TransitionRules {
    type Item = ([u8; 3], u8);
    type IntoIter = TransitionRulesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        TransitionRulesIter {
            transition_rules: self,
            index: 0,
        }
    }
}

impl<'a> Iterator for TransitionRulesIter<'a> {
    type Item = ([u8; 3], u8);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.transition_rules.rules.len() {
            let rule = &self.transition_rules.rules[self.index];
            self.index += 1;
            Some((rule.neighborhood, rule.next_state))
        } else {
            None
        }
    }
}
