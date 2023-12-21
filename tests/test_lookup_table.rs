// Desc: Tests for the lookup table module.

use mono_axis::core::lookup_table::M1DLookupTable;
use mono_axis::core::lookup_table::M1DLookupTableError;

#[test]
fn it_works() {
    let lt = M1DLookupTable::new(3, 3, 3, 0);
    let result = lt.get(2, 2, 2).unwrap();
    assert_eq!(*result, 0);
}

#[test]
fn it_returns_index_out_of_bounds_on_get_call() {
    let lt = M1DLookupTable::new(3, 3, 3, 0);
    let result = lt.get(3, 3, 3);
    assert_eq!(result, Err(M1DLookupTableError::IndexOutOfBounds));
}

#[test]
fn it_returns_index_out_of_bounds_on_set_call() {
    let mut lt = M1DLookupTable::new(3, 3, 3, 0);
    let result = lt.set(3, 3, 3, 1);
    assert_eq!(result, Err(M1DLookupTableError::IndexOutOfBounds));
}

#[test]
fn it_returns_index_out_of_bounds_on_set_call_with_correct_index() {
    let mut lt = M1DLookupTable::new(3, 3, 3, 0);
    let lt_address_before = &lt as *const _;
    let result = lt.set(2, 2, 2, 1);
    assert!(result.is_ok());
    let lt_address_after = &lt as *const _;
    assert_eq!(lt_address_before, lt_address_after);
}

#[test]
fn it_returns_correct_value_on_get_call() {
    let mut lt = M1DLookupTable::new(3, 3, 3, 0);
    _ = lt.set(2, 2, 2, 1);
    let result = lt.get(2, 2, 2);
    assert_eq!(result, Ok(&1));
}

#[test]
fn it_returns_correct_collection_size() {
    let lt = M1DLookupTable::new(3, 3, 3, 0);
    let result = lt.collection_size();
    assert_eq!(result, 27);
}

#[test]
fn it_returns_correct_indices() {
    let lt = M1DLookupTable::new(3, 3, 3, 0);
    let result = lt.iter_indices().collect::<Vec<(usize, usize, usize)>>();
    assert_eq!(
        result,
        vec![
            (0, 0, 0),
            (0, 0, 1),
            (0, 0, 2),
            (0, 1, 0),
            (0, 1, 1),
            (0, 1, 2),
            (0, 2, 0),
            (0, 2, 1),
            (0, 2, 2),
            (1, 0, 0),
            (1, 0, 1),
            (1, 0, 2),
            (1, 1, 0),
            (1, 1, 1),
            (1, 1, 2),
            (1, 2, 0),
            (1, 2, 1),
            (1, 2, 2),
            (2, 0, 0),
            (2, 0, 1),
            (2, 0, 2),
            (2, 1, 0),
            (2, 1, 1),
            (2, 1, 2),
            (2, 2, 0),
            (2, 2, 1),
            (2, 2, 2),
        ]
    );
}

#[test]
fn it_replaces_values_with_replace_values() {
    let mut lt = M1DLookupTable::new(3, 3, 3, 0);
    _ = lt.set(2, 2, 2, 1);
    _ = lt.replace_values(1, 2);
    let result = lt.get(2, 2, 2).unwrap();
    assert_eq!(*result, 2);
}

#[test]
fn it_finalizes() {
    let mut lt = M1DLookupTable::new(3, 3, 3, 0);
    _ = lt.set(2, 2, 2, 1);
    _ = lt.finalize(0);
    let result = lt.get(2, 2, 2).unwrap();
    assert_eq!(*result, 1);
}
