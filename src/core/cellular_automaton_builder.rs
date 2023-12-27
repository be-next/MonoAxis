use crate::core::cellular_automaton::CA1D;
use crate::core::cellular_automaton_configuration::CA1DConfiguration;
use crate::core::cellular_automaton_configuration::CA1DConfigurationError;
use crate::core::lookup_table_builder::LookupTableBuilder;
use crate::core::lookup_table_builder::LookupTableBuilderError;

pub struct CellularAutomatonBuilder;

#[derive(Debug)]
pub enum CellularAutomatonBuilderError {
    ConfigurationError(CA1DConfigurationError),
    LookupTableBuilderError(LookupTableBuilderError),
}

impl From<CA1DConfigurationError> for CellularAutomatonBuilderError {
    fn from(err: CA1DConfigurationError) -> Self {
        CellularAutomatonBuilderError::ConfigurationError(err)
    }
}

impl From<LookupTableBuilderError> for CellularAutomatonBuilderError {
    fn from(err: LookupTableBuilderError) -> Self {
        CellularAutomatonBuilderError::LookupTableBuilderError(err)
    }
}

impl CellularAutomatonBuilder {
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self, configuration_file_name: &str) -> Result<CA1D, CellularAutomatonBuilderError> {
        let ca1d_configuration = CA1DConfiguration::new_from_json_file(configuration_file_name)?;

        let lookup_table_builder = LookupTableBuilder::new();
        let lookup_table = lookup_table_builder.build(ca1d_configuration.get_rules_file_name())?;

        let mut ca1d = CA1D::new(
            ca1d_configuration.get_num_states(),
            ca1d_configuration.get_num_cells(),
            lookup_table,
        );

        ca1d.set_world_initialisation(ca1d_configuration.get_world_initialisation());

        Ok(ca1d)
    }
}