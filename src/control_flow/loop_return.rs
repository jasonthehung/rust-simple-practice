#[allow(unused)]

pub fn loop_return() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);
}

#[test]
fn test_loop_return() {
    loop_return();
}
