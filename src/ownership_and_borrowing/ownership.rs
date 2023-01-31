#[allow(unused)]
pub fn calculate_length(s: String) -> (String, usize) {
    let len = s.len();

    (s, len)
}

#[test]
fn test_calculate_length() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}
