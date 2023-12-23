use mono_axis::core::cellular_automaton::CA1D;
// use mono_axis::core::lookup_table::M1DLookupTable;
use mono_axis::core::lookup_table_builder::LookupTableBuilder;


#[test]
fn it_works() {
    let ltb = LookupTableBuilder::new();
    let lt = ltb.build("examples/example_01/rules.json").unwrap();
    let mut ca = CA1D::new(3, 30, lt);
    ca.step();
}

