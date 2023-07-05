// iterators2.rs
// In this module, you'll learn some of unique advantages that iterators can offer.
// As always, there are hints if you execute `rustlings hint iterators2`!

// I AM NOT DONE

// Step 1. Complete the `capitalize_first` function to pass the first two cases.
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
    }
}

// Step 2. Apply the `capitalize_first` function to a vector of strings.
//         Ensure that it returns a vector of strings as well.
pub fn capitalize_strings_vector(words: Vec<&str>) -> Vec<String> {
    words.into_iter().map(|s| capitalize_first(s)).collect()
}

// Step 3. Apply the `capitalize_first` function again to a list.
//         Try to ensure it returns a single string.
pub fn capitalize_list_to_string(words: Vec<&str>) -> String {
    capitalize_strings_vector(words).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Step 1.
    // Tests that verify your `capitalize_first` function implementation
    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    // Step 2.
    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        let capitalized_words: Vec<String> = capitalize_strings_vector(words);
        assert_eq!(capitalized_words, ["Hello", "World"]);
    }

    // Step 3.
    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        let capitalized_words = capitalize_list_to_string(words);
        assert_eq!(capitalized_words, "Hello World");
    }
}
