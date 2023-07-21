// ing Compression: Implement a method to perform basic string compression using the counts
// of repeated characters. For example, the string aabcccccaaa would become a2blc5a3. If the
// "compressed7'string would not become smaller than theoriginal string, your method should return
// the original string. You can assume the string has only uppercase and lowercase letters (a - z).





fn compress_string(a: &str) -> String
{
    // You can assume the string has only uppercase and lowercase letters (a - z).
    let mut last_char: char = '.';
    let mut chain_len = 1;
    let mut result = a.chars().fold(String::new(), |mut result, current_char| {
        if last_char == current_char {
            chain_len += 1;
        }else {
            if last_char != '.' {
                result.push_str(&format!("{}{}", last_char, chain_len));
            }
            chain_len = 1;
            last_char = current_char;
        }
        result
    });
    if chain_len > 0 {
        result.push_str(&format!("{}{}", last_char, chain_len));
    }
    if a.len() < result.len() {
        return String::from(a);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compress_string() {
        assert_eq!(compress_string("aabcccccaaa"), "a2b1c5a3");
        assert_eq!(compress_string("abcccccaaa"), "a1b1c5a3");
        assert_eq!(compress_string("abc"), "abc");
        assert_eq!(compress_string("abbcc"), "abbcc");
        assert_eq!(compress_string("aabbcc"), "a2b2c2");
    }
}

fn main() {
    //
}





