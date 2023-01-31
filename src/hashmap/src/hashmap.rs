// FILL in the blanks and FIX the errors
// ! https://practice.rs/collections/hashmap.html#basic-operations
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Person {
    name: String,
}

#[derive(Debug)]
struct PhoneNumber {
    country_code: u8,
    area_code: u8,
    digits: u8,
}

#[cfg(test)]
mod test {
    use std::vec;

    use super::*;

    #[test]
    fn test_basic_operation_1() {
        let mut scores = HashMap::new();
        scores.insert("Sunface", 98);
        scores.insert("Daniel", 95);
        scores.insert("Ashley", 69);
        scores.insert("Katie", 58);

        // Get returns an Option<&V>
        let score = scores.get("Sunface");
        assert_eq!(score, Some(&98));

        if scores.contains_key("Daniel") {
            // Indexing returns a value V
            let score = scores["Daniel"];
            assert_eq!(score, 95);
            scores.remove("Daniel");
        }

        assert_eq!(scores.len(), 3);

        for (name, score) in scores {
            println!("The score of {} is {}", name, score);
        }
    }

    #[test]
    fn test_basic_operation_2() {
        let teams = [
            ("Chinese Team", 100),
            ("American Team", 10),
            ("France Team", 50),
        ];

        let mut teams_map1 = HashMap::new();
        for team in &teams {
            teams_map1.insert(team.0, team.1);
        }

        // IMPLEMENT team_map2 in two ways
        // Tips: one of the approaches is to use `collect` method
        let teams_map2: HashMap<_, _> = teams.into_iter().collect();

        assert_eq!(teams_map1, teams_map2);

        println!("Success!");
    }

    #[test]
    fn test_basic_operation_3() {
        // Type inference lets us omit an explicit type signature (which
        // would be `HashMap<&str, u8>` in this example).
        let mut player_stats = HashMap::new();

        // Insert a key only if it doesn't already exist
        player_stats.entry("health").or_insert(100);

        assert_eq!(player_stats["health"], 100);

        // Insert a key using a function that provides a new value only if it
        // doesn't already exist
        player_stats.entry("health").or_insert_with(|| 1);
        assert_eq!(player_stats["health"], 100);

        // Ensures a value is in the entry by inserting the default if empty, and returns
        // a mutable reference to the value in the entry.
        let health = player_stats.entry("health").or_insert(50);
        assert_eq!(health, &100);
        *health -= 50;
        assert_eq!(*health, 50);

        println!("Success!");
    }

    #[test]
    fn test_basic_operation_4() {
        // Use a HashMap to store the vikings' health points.
        let vikings = HashMap::from([
            (Viking::new("Einar", "Norway"), 25),
            (Viking::new("Olaf", "Denmark"), 24),
            (Viking::new("Harald", "Iceland"), 12),
        ]);

        // Use derived implementation to print the status of the vikings.
        for (viking, health) in &vikings {
            println!("{:?} has {} hp", viking, health);
        }

        println!("Key: {:?}", vikings.keys());
    }

    #[test]
    fn test_basic_operation_5() {
        let teams = vec![
            String::from("Blue"),
            String::from("Red"),
            String::from("Green"),
        ];

        let initial_score = vec![10, 50, 60];

        // * 這也是一種產生HashMap的方式
        let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_score.into_iter()).collect();
    }

    #[test]
    fn test_basic_operation_6() {
        let text = "hello world wonderful world";

        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
    }

    #[test]
    fn use_custom_struct_as_key() {
        let mut map = HashMap::new();
        let jason = Person {
            name: String::from("Jason"),
        };

        let kevin = Person {
            name: String::from("kevin"),
        };

        map.insert(
            jason,
            PhoneNumber {
                country_code: 1,
                area_code: 99,
                digits: 131,
            },
        );

        map.insert(
            kevin,
            PhoneNumber {
                country_code: 1,
                area_code: 99,
                digits: 131,
            },
        );

        for (key, value) in &map {
            println!("{:?}: {:?}", key, value);
        }
    }

    #[test]
    fn test_ownership() {
        let v1 = 10;
        let mut m1 = HashMap::new();
        m1.insert(v1, v1); // Copy
        println!("v1 is still usable after inserting to hashmap : {}", v1); // v1 Live

        let v2 = "hello".to_string();
        let mut m2 = HashMap::new();

        let v2_str_slice = "jason";
        // @ &str implements Copy trait
        // m2.insert(v2, v1); // v2 Ownership moved here
        m2.insert(v2_str_slice, v1);
    }
}

#[derive(Hash, Debug, Eq, PartialEq)]
struct Viking {
    name: String,
    country: String,
}

impl Viking {
    /// Creates a new Viking.
    fn new(name: &str, country: &str) -> Viking {
        Viking {
            name: name.to_string(),
            country: country.to_string(),
        }
    }
}
