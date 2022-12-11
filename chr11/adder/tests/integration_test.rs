use adder;

#[test]
fn it_adds_two() {
    assert_eq!(4, adder::add_two(2));
}

#[test]
fn it_adds_three() {
    assert_eq!(5, adder::add_three(2));
}
