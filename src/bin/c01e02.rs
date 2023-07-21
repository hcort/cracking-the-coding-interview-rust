// Check Permutation: Given two strings, write a method to decide if one is a permutation of the
// other.


fn is_permutation(s1: &str, s2: &str) -> bool {
    if  s1.len() != s2.len()
    {
        return false;
    }
    use std::collections::HashMap;
    let mut characters_count_1: HashMap<char, i32> = HashMap::new();
    let mut characters_count_2: HashMap<char, i32> = HashMap::new();
    for c in s1.chars()
    {
        let count = characters_count_1.entry(c).or_insert(1);
        *count += 1;
    }
    for c in s2.chars()
    {
        let count = characters_count_2.entry(c).or_insert(1);
        *count += 1;
    }
    for c in characters_count_1.keys()
    {
        let v2 = characters_count_2.get(c).cloned().unwrap_or(-1);
        let v1 = match characters_count_1.get(c) {
            Some(v1) => *v1,
            None => -1, // never happens
        };
        if v1 != v2 {
            return false;
        }else {
            characters_count_2.remove(c);
        }
    }
    return characters_count_2.is_empty();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_permutation() {
        assert_eq!(
            is_permutation(&String::from("cat"), &String::from("tac")),
            true
        );
        assert_eq!(
            is_permutation(&String::from("cat"), &String::from("ttac")),
            false
        );
        assert_eq!(
            is_permutation(&String::from("catt"), &String::from("ttac")),
            true
        );
        assert_eq!(
            is_permutation(&String::from("cat"), &String::from("dog")),
            false
        );
    }
}

fn main() {
    //is_permutation(&String::from("cat"), &String::from("dog"));
}