use serde::{Deserialize, Deserializer};
use std::str::FromStr;

#[derive(Debug, Clone)]
struct VecFromSpaceSeparatedString(Vec<i8>);

impl<'de> Deserialize<'de> for VecFromSpaceSeparatedString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let vec = s.split_whitespace()
            .filter_map(|num| i8::from_str(num).ok())
            .collect();
        Ok(VecFromSpaceSeparatedString(vec))
    }
}

impl Into<Vec<i8>> for VecFromSpaceSeparatedString {
    fn into(self) -> Vec<i8> {
        self.0.clone()
    }
}

#[derive(Debug)]
pub enum CA1DConfigurationError {
    JsonParseError(serde_json::Error),
    IoError(std::io::Error),
}

impl From<serde_json::Error> for CA1DConfigurationError {
    fn from(err: serde_json::Error) -> Self {
        CA1DConfigurationError::JsonParseError(err)
    }
}

impl From<std::io::Error> for CA1DConfigurationError {
    fn from(err: std::io::Error) -> Self {
        CA1DConfigurationError::IoError(err)
    }
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct CA1DConfiguration {
    num_states: u8,
    num_cells: usize,
    world_initialisation: VecFromSpaceSeparatedString,
    rules_file_name: String,
}

impl CA1DConfiguration {
    pub fn new_from_json_string(json_string: &str) -> Result<Self, CA1DConfigurationError> {
        let ca1d_configuration: CA1DConfiguration = serde_json::from_str(json_string)?;
        Ok(ca1d_configuration)
    }

    pub fn new_from_json_file(file_path: &str) -> Result<Self, CA1DConfigurationError> {
        let json_string = std::fs::read_to_string(file_path)?;
        Self::new_from_json_string(&json_string)
    }

    pub fn get_world_initialisation(&self) -> Vec<i8> {
        self.world_initialisation.clone().into()
    }

    pub fn get_rules_file_name(&self) -> &str {
        &self.rules_file_name
    }

    pub fn get_num_states(&self) -> u8 {
        self.num_states
    }

    pub fn get_num_cells(&self) -> usize {
        self.num_cells
    }
}