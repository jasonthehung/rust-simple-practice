#[allow(unused)]

pub fn loop_label() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                println!("break 'counting_up' loop");
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
}

#[test]
fn test_loop_label() {
    loop_label()
}
