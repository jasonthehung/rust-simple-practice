pub fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

pub fn better_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

#[test]
fn test_first_word() {
    let mut s = String::from("Hello world!");

    let word = first_word(&s);
    println!("First word: {}", word); // OK

    s.clear();

    println!("First word: {}", word); // Not GOOD
}

#[test]
fn test_better_first_word() {
    let mut s = String::from("Hello world!");

    let word = better_first_word(&s);
    println!("First word: {}", word);

    s.clear();
}
