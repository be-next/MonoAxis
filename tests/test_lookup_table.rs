// Desc: Tests for the lookup table module.

use mono_axis::core::lookup_table::LookupTable3d as LookupTable3d;
#[test]
fn it_works() {
    let lt = LookupTable3d::new(3, 3, 3, 0);
    let result = lt.get(2, 2, 2);
    assert_eq!(result, &0);
}

