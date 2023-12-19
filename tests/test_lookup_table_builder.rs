//

use mono_axis::core::lookup_table_builder::LookupTableBuilder;

#[test]
fn it_works() {
    let ltb = LookupTableBuilder::new();
    let lt = ltb.build("examples/example_01/rules.json").unwrap();
    let result = lt.get(1, 0, 1).unwrap();

    println!("{}", lt);
    assert_eq!(*result, 2);
}
