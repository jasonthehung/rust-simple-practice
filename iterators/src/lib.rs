#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let v1 = vec!["fear", "anger", "surprise", "heart"];
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&"fear"));
        assert_eq!(v1_iter.next(), Some(&"anger"));
        assert_eq!(v1_iter.next(), Some(&"surprise"));
        assert_eq!(v1_iter.next(), Some(&"heart"));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: u32 = v1_iter.sum();

        assert_eq!(total, 6);
    }

    #[test]
    fn iterator_map() {
        let v1 = vec![1, 2, 3];

        let x = v1
            .iter()
            .map(|x| x + 1)
            .filter(|x| *x > 1)
            .collect::<Vec<i32>>();

        for item in x {
            println!("item after map and filter: {}", item);
        }
    }
}
