// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

impl FromStr for Person {
// Steps:
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        // 1. If the length of the provided string is 0, then return an error
        if s.len() == 0 {
            return Err(Self::Err::from("Zero length"));
        }
        // 2. Split the given string on the commas present in it
        let split: Vec<&str> = s.split(',').collect();
        if split.len() < 2 {
            return Err(Self::Err::from("No enough parts"));
        }
        // 3. Extract the first element from the split operation and use it as the name
        let name = split[0];
        // 4. If the name is empty, then return an error
        if name.is_empty() {
            return Err(Self::Err::from("Name must not be empty"));
        }
        // 5. Extract the other element from the split operation and parse it into a `usize` as the age
        //    with something like `"4".parse::<usize>()`.
        if let age  = split[1].parse::<usize>() {
            Ok(Person {name: name.to_string(), age: age.unwrap()})
        } else {
            Err(Self::Err::from("Parsing error"))
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        let p = "John,32".parse::<Person>();
        assert!(p.is_ok());
        let p = p.unwrap();
        assert_eq!(p.name, "John");
        assert_eq!(p.age, 32);
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John,".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn invalid_age() {
        "John,twenty".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_comma_and_age() {
        "John".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name() {
        ",1".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_age() {
        ",".parse::<Person>().unwrap();
    }

    #[test]
    #[should_panic]
    fn missing_name_and_invalid_age() {
        ",one".parse::<Person>().unwrap();
    }

}
