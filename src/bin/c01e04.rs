// Palindrome Permutation: Given a string, write a function to check if it is a permutation of a palin-
// drome. A palindrome is a word or phrase that is the same forwards and backwards. A permutation
// is a rearrangement of letters. The palindrome does not need to be limited to just dictionary words.
// EXAMPLE
// Input: Tact Coa
// Output: True (permutations: "taco cat", "atco cta", etc.
use std::collections::HashMap;
// palindrome permutation: each letter appears 2x times in the string
//                          if the string has odd length one letter appears 2x+1 times

fn is_palindrome_permutation(palindrome: &str) -> bool
{
    let mut characters_count: HashMap<char, i32> = HashMap::new();
    for c in palindrome.chars()
    {
        if  c != ' ' {
            let count = characters_count.entry(c).or_insert(0);
            *count += 1;
        }
    }
    let mut num_even: u8 = 0;
    for c in characters_count.keys()
    {
        let v1 = match characters_count.get(c) {
            Some(v1) => (*v1 % 2) == 0,
            None => false, // never happens
        };
        if !v1 {
            num_even += 1;
            if num_even > 1{
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_palindrome_permutation() {
        assert_eq!(is_palindrome_permutation(&String::from("taco cat")), true);
        assert_eq!(is_palindrome_permutation(&String::from("atco cta")), true);
        assert_eq!(is_palindrome_permutation(&String::from("atcao cta")), false);
        assert_eq!(is_palindrome_permutation(&String::from("tacoo cat")), true);
        assert_eq!(is_palindrome_permutation(&String::from("tacoo mcat")), true);
        assert_eq!(is_palindrome_permutation(&String::from("tacvoo mcat")), false);
    }
}

fn main() {
    //
}