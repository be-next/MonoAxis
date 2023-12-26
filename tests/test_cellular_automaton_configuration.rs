use mono_axis::core::cellular_automaton_configuration::CA1DConfiguration;

#[test]
fn it_works() {
    let ca1d_configuration = CA1DConfiguration::new_from_json_file("examples/example_01/configuration.json").unwrap();
    let result = ca1d_configuration.get_world_initialisation();
    assert_eq!(result, vec![0, 0, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 0]);
}