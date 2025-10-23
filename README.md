# CodeWars-Count-characters-in-your-string-6-kyu---Passed-
The main idea is to count all the occurring characters in a string. If you have a string like aba, then the result should be {'a': 2, 'b': 1}.  What if the string is empty? Then the result should be empty object literal, {}.


// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use rand::Rng;
    use super::*;
    
    fn solution(input: &str) -> HashMap<char, i32> {  
        let mut char_map: HashMap<char, i32> = HashMap::new();

        for letter in input.chars() {
            let count = char_map.entry(letter).or_insert(0);
            *count += 1;
        }

        char_map
    }

   const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn test_empty_string() {
        let test_input = "";
        let expected: HashMap<char, i32> = HashMap::new();
        
        assert_eq!(count(test_input), expected, "{ERR_MSG} with input: \"{test_input}\"");
    }
    
    #[test]
    fn test_string_with_two_equal_letters() {
        let test_input = "aa";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);
        
        assert_eq!(count(test_input), expected, "{ERR_MSG} with input: \"{test_input}\"");
    }
        
    #[test]
    fn test_string_with_different_letters() {
        let test_input = "aabb";
        let mut expected: HashMap<char, i32> = HashMap::new();
        expected.insert('a', 2);
        expected.insert('b', 2);
        
        assert_eq!(count(test_input), expected, "{ERR_MSG} with input: \"{test_input}\"");
    }
        
    #[test]
    fn test_random_strings() {
        let mut rng = rand::thread_rng();
        
        for _ in 0..100 {
            let rand_input_length = rng.gen_range(5u8..255u8) as i32;
            let test_input = random_string(rand_input_length);
            
            assert_eq!(count(&test_input), solution(&test_input), "{ERR_MSG} with input: \"{test_input}\"");
        }
    }
    
    fn random_string(length: i32) -> String {
        let mut rng = rand::thread_rng();
        let str: String = (0..length).map(|_| rng.gen_range('a'..'z') as char).collect();
        str
    }
}
