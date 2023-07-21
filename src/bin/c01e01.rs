/*
    Chapter 1 | Arrays and Strings
    1 1 Implement an algorithm to determine if a string has all unique characters What if you
        can not use additional data structures?
 */

fn all_chars_unique_part_a(input_str: &str) -> bool {
    // let's asume ASCII
    if input_str.len() > 128{
        return false;
    }
    //let mut letters = vec![false; 128];
    let mut letters = [false; 128];
    for byt in input_str.as_bytes() {
        let c = *byt;
        if letters[c as usize]{
            return false;
        }
        letters[c as usize] = true;
    }
    return true;
}

fn all_chars_unique_part_a_hash(input_str: &str) -> bool {
    use std::collections::HashSet;
    let mut characters: HashSet<char> = HashSet::new();
    for c in input_str.chars()
    {
        if characters.contains(&c){
            return false;
        }
        characters.insert(c);
    }
    return true;
}

fn all_chars_unique_part_b(s: &str) -> bool {
    //// solution from repo, only works for a subset of ASCII
    let mut bitfield: i64 = 0;
    let a_int_char: i16 = 'a' as i16;

    for c in s.chars() {
        let mut int_char: i16 = c as i16;
        int_char -= a_int_char;

        if (1 << int_char) & bitfield != 0 {
            return false;
        }

        // set bit
        bitfield |= 1 << int_char;
    }

    true
}

fn all_chars_unique_part_b_128_bits(s: &str) -> bool {
    // What if you can not use additional data structures?
    // assume ASCII -> 128 characters
    let mut characters_as_bits: u128 = 0;
    for c in s.chars()
    {
        if (1 << c as u8) & characters_as_bits != 0 {
            return false;
        }
        characters_as_bits |= 1 << c as u8;
    }
    true
}

fn all_chars_unique_part_b_no_extra_data(s: &str) -> bool {
    // What if you can not use additional data structures?
    // O(n^2) loop
    let str_as_bytes = s.as_bytes();
    for (i, c) in str_as_bytes.iter().enumerate()
    {
        for j in i+1..str_as_bytes.len()
        {
            if *c == str_as_bytes[j]
            {
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
    fn test_part_a() {
        assert_eq!(all_chars_unique_part_a(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_part_a(&String::from("abcdefga")), false);
        // all_chars_unique_part_a_hash
        assert_eq!(all_chars_unique_part_a_hash(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_part_a_hash(&String::from("abcdefga")), false);
    }

    #[test]
    fn test_part_b() {
        // solutions from repo
        // chars < 'a' cause panic (1 << c - 'a') -> bit shift with negative value
        assert_eq!(all_chars_unique_part_b(&String::from("abcdefg")), true);
        //assert_eq!(all_chars_unique_part_b(&String::from("abcd!!fg")), false);
        //assert_eq!(all_chars_unique_part_b(&String::from("abcdefgA")), true);
        //assert_eq!(all_chars_unique_part_b(&String::from("abcdefgAA")), false);

        assert_eq!(all_chars_unique_part_b_128_bits(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_part_b_128_bits(&String::from("abcd!!fg")), false);
        assert_eq!(all_chars_unique_part_b_128_bits(&String::from("abcdefgA")), true);
        assert_eq!(all_chars_unique_part_b_128_bits(&String::from("abcdefgAA")), false);

        assert_eq!(all_chars_unique_part_b_no_extra_data(&String::from("abcdefg")), true);
        assert_eq!(all_chars_unique_part_b_no_extra_data(&String::from("abcd!!fg")), false);
        assert_eq!(all_chars_unique_part_b_no_extra_data(&String::from("abcdefgA")), true);
        assert_eq!(all_chars_unique_part_b_no_extra_data(&String::from("abcdefgAA")), false);
    }
}

fn main() {
    //all_chars_unique_part_a(&String::from("helloworld"));
    //all_chars_unique_part_b(&String::from("helloworld"));
}