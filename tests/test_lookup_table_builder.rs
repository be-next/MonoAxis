//
use mono_axis::core::lookup_table::LookupTable3d;
use mono_axis::core::lookup_table_builder::LookupTableBuilder;

const LOOKUP_TABLE_EX1: &str = r#"
    {
        "data": [0,0,0,1,1,2,2,0,2,0,2,0,1,1,2,2,1,2,0,0,0,1,1,1,2,2,2],
        "left": 3,
        "center": 3,
        "right": 3
    }
"#;

#[test]
fn it_works() {
    let ltb = LookupTableBuilder::new();
    let lt = ltb.build("examples/example_01/rules.json").unwrap();
    let result = lt.get(1, 0, 1).unwrap();

    assert_eq!(*result, 2);
}

#[test]
fn it_works_with_example_01() {
    let ltb = LookupTableBuilder::new();
    let lt = ltb.build("examples/example_01/rules.json").unwrap();
    let result: LookupTable3d = serde_json::from_str(LOOKUP_TABLE_EX1).unwrap();

    assert_eq!(result, lt);
}