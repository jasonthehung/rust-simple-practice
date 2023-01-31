#![allow(unused)]

#[test]
fn test_lifetime() {
    let mut x = Box::new(1);

    let r = &x; // 'a

    if rand() > 0.5 {
        *x = 82;
    } else {
        println!("{}", r); // 'a
    }
}

#[test]
fn lifetime_can_have_holes() {
    let mut x = Box::new(2);
    let mut z = &x; // 'a

    for i in 0..100 {}
}

fn rand() -> f64 {
    0.1
}
