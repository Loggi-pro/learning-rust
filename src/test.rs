//canot call test function from real app (like #if test_environment define)
#[test]
fn funny_name() {
    assert_eq!(1, 1);
}

//canot call test function from real app (like #if test_environment define)
#[test]
fn funny_name2() {
    assert_eq!(2, 34);
}

pub fn other_function() -> i32 {
    return -10;
}
