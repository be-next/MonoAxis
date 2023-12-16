// Desc: Tests for the TransitionRules struct

use mono_axis::core::rules::TransitionRules as TransitionRules;

const JSON_DATA : &str = r#"
        {
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
    assert_eq!(result, 2u8);
}
