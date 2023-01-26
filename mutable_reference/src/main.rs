fn main() {}

fn replace_with_84(s: &mut Box<i32>) {
    // ! You can't do this, because s will be empty.
    // let was = *s;
    println!("s before taken: {}", s);
    let was = std::mem::take(s);

    println!("s after taken: {}", s);

    println!("was: {}", was);

    // *s = was;
    // println!("give it bakc to s: {}", s);
}

#[test]
fn test_replace_with_84() {
    let mut b = Box::new(80);
    replace_with_84(&mut b);
}

#[test]
fn test_mem_swap() {
    let mut x1 = Box::new(1);
    let mut x2 = Box::new(2);

    std::mem::swap(&mut x1, &mut x2);

    assert_eq!(*x1, 2);
    assert_eq!(*x2, 1);
}
