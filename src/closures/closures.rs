#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    #[test]
    fn slow_closure_test() {
        let expansive_closure = |num: u32| -> u32 {
            println!("Slow....");
            thread::sleep(Duration::from_secs(2));
            num
        };

        expansive_closure(10);
    }

    #[test]
    fn must_use_it() {
        // ! Compiler doesn't know how the type, because there's no type annotation
        // let add_one_v4 = |x| x + 1;
    }

    #[test]
    fn compiler_infer_type() {
        // let example_closure = |x| x;

        // let s = example_closure(String::from("hello")); // String
        // let n = example_closure(5); // i32
    }

    #[test]
    fn immutable_closure() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // variable can bind to a closure definition
        let only_borrows = || println!("From closure: {:?}", list);

        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
    }

    #[test]
    fn mutable_closure() {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        let mut borrows_mutably = || list.push(7);
        // println!("After calling closure: {:?}", list); // ! Error
        borrows_mutably();
        println!("After calling closure: {:?}", list);
    }

    #[test]
    fn thread_move_closure() {
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();
    }
}
