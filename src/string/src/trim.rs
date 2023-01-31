#[test]
fn it_works() {
    let a = "0x231fc";

    let b = a.trim_start_matches("0x");

    let c = i128::from_str_radix(b, 16).unwrap();

    println!("{}", c);
}
