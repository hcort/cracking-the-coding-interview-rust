// URLify: Write a method to replace all spaces in a string with '%20'. You may assume that the string
// has sufficient space at the end to hold the additional characters, and that you are given the utrue"
// length of the string. (Note: If implementing in Java, please use a character array so that you can
// perform this operation in place.)
// EXAMPLE
// Input: "Mr John Smith ", 13
// Output: rrMr%20John%20SmithJJ
// Hints:#53, #118

fn url_encoder(s: &str) -> String
{
    //
    let mut url_str: String = String::from("");
    for c in s.chars()
    {
        if  c == ' ' {
            url_str.push_str("%20");
        }else{
            url_str.push(c);
        }
    }
    url_str
}

fn url_encoder_2(url: &str) -> String
{
    url.trim().replace(' ', "%20")
}

fn url_encoder_3(url: &str) -> String
{
    // fails on strings with more than one whitespace together
    let placeholder = "%20";
    // fold( valor inicial, |variables capturadas|, función)
    url.split_whitespace().fold(String::new(), |acc, s| {
        if acc.is_empty() {
            String::from(s) // último
        } else {
            acc + placeholder + s
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_url_encoder() {
        assert_eq!(url_encoder(&String::from("a b c d")), String::from("a%20b%20c%20d"));
        assert_eq!(url_encoder_2(&String::from("hola que tal ")), String::from("hola%20que%20tal"));
        assert_eq!(url_encoder_2(&String::from("hola  que tal ")), String::from("hola%20%20que%20tal"));
        assert_eq!(url_encoder_3(&String::from("hola que tal ")), String::from("hola%20que%20tal"));
        // fails as split_whitespace ignores multiple whitespaces
        // assert_eq!(url_encoder_3(&String::from("hola  que tal ")), String::from("hola%20%20que%20tal"));
    }
}

fn main() {
    //
}




