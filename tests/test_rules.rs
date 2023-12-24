// Desc: Tests for the TransitionRules struct

use mono_axis::core::rules::TransitionRules;

const JSON_DATA: &str = r#"
        {
          "name": "Sum",
          "num_states": 3,
          "rules": [
            {"neighborhood": [1, 0, 1], "next_state": 2},
            {"neighborhood": [1, 2, 1], "next_state": 1},
            {"neighborhood": [2, 1, 1], "next_state": 2},
            {"neighborhood": [1, 2, 0], "next_state": 0},
            {"neighborhood": [2, 1, 0], "next_state": 2}
          ]
        }
    "#;

#[test]
fn it_deserializes() {
    let rules: TransitionRules = serde_json::from_str(JSON_DATA).unwrap();
    let result = rules.rules[0].next_state;
    assert_eq!(result, 2i8);
}

#[test]
fn it_deserializes_from_string() {
    let result = TransitionRules::new_from_json_string(JSON_DATA);
    assert_eq!(result.is_ok(), true);
}

#[test]
fn it_deserializes_from_file() {
    let result = TransitionRules::new_from_json_file("examples/example_01/rules.json");
    assert_eq!(result.is_ok(), true);
}

#[test]
fn it_returns_error_on_invalid_json() {
    let result = TransitionRules::new_from_json_string("invalid json");
    assert_eq!(result.is_err(), true);
}

#[test]
fn it_returns_error_on_invalid_file() {
    let result = TransitionRules::new_from_json_file("invalid file");
    assert_eq!(result.is_err(), true);
}

#[test]
fn it_iterates_correctly() {
    let rules: TransitionRules = serde_json::from_str(JSON_DATA).unwrap();
    let result = rules.into_iter().collect::<Vec<([i8; 3], i8)>>();
    assert_eq!(
        result,
        vec![
            ([1, 0, 1], 2),
            ([1, 2, 1], 1),
            ([2, 1, 1], 2),
            ([1, 2, 0], 0),
            ([2, 1, 0], 2),
        ]
    );
}
