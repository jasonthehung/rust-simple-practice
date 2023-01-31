#[allow(unused)]
pub fn calculate_length(s: &String) -> usize {
    s.len()
}

pub fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

#[test]
fn test_calculate_length() {
    let mut s1 = String::from("hello");

    change(&mut s1);

    let length = calculate_length(&s1);

    println!("Here is my length: {}, and the string is '{}'", length, s1);
}
