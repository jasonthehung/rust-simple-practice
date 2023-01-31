#[allow(unused)]

pub fn range() {
    for number in (0..10).rev() {
        println!("{}!", number);
    }
}

#[test]
fn test_range() {
    range();
}
