fn main() {
    let mut b = Box::new(80);
    replace_with_84(&mut b);
}

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
