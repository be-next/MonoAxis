use mono_axis::core::cellular_automaton_builder::CellularAutomatonBuilder;

#[test]
fn it_works_ex1() {
    let cab = CellularAutomatonBuilder::new();
    let mut ca = cab.build("examples/example_01/configuration.json").unwrap();

    for _ in 0..7 {
        ca.step();
    }

    let result = ca.get_current_world();

    assert_eq!(*result, vec![0, 0, 1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0]);
}

#[test]
fn it_works_ex2() {
    let cab = CellularAutomatonBuilder::new();

    let mut ca = cab.build("examples/example_02/configuration.json").unwrap();

    for _ in 0..40 {
        ca.step();
    }

    let result = ca.get_current_world();

    assert_eq!(*result, vec![0, 0, 1, 1, 1, 0, 1, 1, 1, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0]);
}