#[derive(Debug)]
struct Person {
    name: String,
    gender: String,
    phone: String,
}

impl Person {
    pub fn new(name: String, gender: String, phone: String) -> Self {
        Self {
            name,
            gender,
            phone,
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_works() {
        let person = Person::new(
            String::from("John"),
            String::from("M"),
            String::from("0902131527"),
        );

        println!("{:#?}", person);
    }

    #[test]
    fn test_member_method() {
        let person = Person::new(
            String::from("John"),
            String::from("M"),
            String::from("0902131527"),
        );

        // This is a member function
        person.get_name();

        // This is a associated function
        // Person::new(...)
    }
}
