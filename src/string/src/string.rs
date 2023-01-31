#[cfg(test)]
mod test {
    use std::mem;

    use super::*;

    #[test]
    fn test_basic_operations_1() {
        // FILL in the blanks and FIX errors
        // 1. Don't use `to_string()`
        // 2. Don't add/remove any code line

        let mut s: String = String::from("hello, ");
        s.push_str("world");
        s.push('!');

        move_ownership(&s);

        assert_eq!(s, "hello, world!");

        println!("Success!");
    }

    #[test]
    fn test_basic_operations_2() {
        let mut s = String::from("hello, world");

        let slice1: &str = s.as_str(); // In two ways
        assert_eq!(slice1, "hello, world");

        let slice2 = &s[..5];
        assert_eq!(slice2, "hello");

        let slice3: &mut String = &mut s;
        slice3.push('!');
        assert_eq!(slice3, "hello, world!");

        println!("Success!");
    }

    #[test]
    fn test_basic_operations_3() {
        // Create a String type based on `&str`
        // The type of string literals is `&str`
        let s = String::from("hello, world!");

        // Create a slice point to String `s`
        let slice: &str = &s;

        // Create a String type based on the recently created slice
        let s: String = slice.to_string();

        assert_eq!(s, "hello, world!");

        println!("Success!");

        //
        let story = "Once upon a time...";

        let ptr = story.as_ptr();
        let len = story.len();
    }

    #[test]
    fn test_basic_operations_4() {
        let s = String::from("hello, 世界");
        let slice1: &str = &s[0..1]; //tips: `h` only takes 1 byte in UTF8 format
        assert_eq!(slice1, "h");

        let slice2 = &s[7..10]; // Tips: `中`  takes 3 bytes in UTF8 format
        assert_eq!(slice2, "世");

        // Iterate through all chars in s
        for (i, c) in s.chars().enumerate() {
            if i == 7 {
                assert_eq!(c, '世')
            }
        }

        println!("Success!");
    }

    #[test]
    fn test_basic_operations_5() {
        let mut s = String::new();
        s.push_str("hello");

        // Some bytes, in a vector
        let v = vec![104, 101, 108, 108, 111];

        // Turn a byte's vector into a String
        let s1 = String::from_utf8(v).unwrap();

        assert_eq!(s, s1);

        println!("Success!");
    }

    #[test]
    fn test_basic_operations_6() {
        // Modify the code below to print out:
        // 25
        // 25
        // 25
        // Here, there’s no need to allocate more memory inside the loop.

        let mut s = String::with_capacity(25);

        println!("{}", s.capacity());

        for _ in 0..2 {
            s.push_str("hello");
            println!("{}", s.capacity());
        }

        println!("Success!");
    }

    #[test]
    fn test_basic_operations_7() {
        let story = String::from("Rust By Practice");

        // Prevent automatically dropping of the String's data
        let mut story = mem::ManuallyDrop::new(story);

        let ptr = story.as_mut_ptr();
        let len = story.len();
        let capacity = story.capacity();

        assert_eq!(16, len);

        // We can rebuild a String out of ptr, len, and capacity. This is all
        // unsafe because we are responsible for making sure the components are
        // valid:
        let s = unsafe { String::from_raw_parts(ptr, len, capacity) };

        assert_eq!(*story, s);

        println!("{}", s);

        println!("Success!");
    }
}

fn move_ownership(s: &String) {
    println!("ownership of \"{}\" is moved here!", s)
}
