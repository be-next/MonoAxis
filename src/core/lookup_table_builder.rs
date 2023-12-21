use crate::core::lookup_table::M1DLookupTable;
use crate::core::rules::{TransitionRules, TransitionRulesError};
pub struct LookupTableBuilder;

#[derive(Debug)]
pub enum LookupTableBuilderError {
    RulesError(TransitionRulesError),
}

impl From<TransitionRulesError> for LookupTableBuilderError {
    fn from(err: TransitionRulesError) -> Self {
        LookupTableBuilderError::RulesError(err)
    }
}

impl LookupTableBuilder {
    const DEFAULT_INITIAL_VALUE: i8 = -1;
    pub fn new() -> Self {
        Self {}
    }

    pub fn build(&self, rules_file_name: &str) -> Result<M1DLookupTable, LookupTableBuilderError> {
        let transition_rules = TransitionRules::new_from_json_file(rules_file_name)?;

        let mut lookup_table = M1DLookupTable::new(
            transition_rules.get_num_states() as usize,
            transition_rules.get_num_states() as usize,
            transition_rules.get_num_states() as usize,
            Self::DEFAULT_INITIAL_VALUE,
        );

        let _ = transition_rules
            .into_iter()
            .for_each(|(neighborhood, next_state)| {
                lookup_table
                    .set(
                        neighborhood[0] as usize,
                        neighborhood[1] as usize,
                        neighborhood[2] as usize,
                        next_state as i8,
                    )
                    .unwrap();
            });

        lookup_table.finalize(Self::DEFAULT_INITIAL_VALUE);

        Ok(lookup_table)
    }
}
