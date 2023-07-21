use std::cmp;
// e Away: There are three types of edits that can be performed on strings: insert a character,
// remove a character, or replace a character. Given two strings, write a function to check if they are
// one edit (or zero edits) away.
// EXAMPLE
// pale, ple -> true
// pales, pale -> true
// pale, bale -> true
// pale, bake -> false
fn is_one_edit_away(a: &str, b: &str) -> bool
{
    if ((a.len() as i32) - (b.len() as i32)).abs() > 1 {
        return false;
    }
    let mut change_found: bool = false;
    let insert: bool = (a.len() as i32) < (b.len() as i32);
    let remove: bool = (a.len() as i32) > (b.len() as i32);
    let mut idx1 = 0;
    let mut idx2 = 0;
    while idx1 < cmp::min(a.len(), b.len()) {
        let char1 = a.get(idx1..=idx1).unwrap();
        let char2 = b.get(idx2..=idx2).unwrap();
        if char1 != char2 {
            if change_found {
                return false;
            }
            if insert {
                idx2 += 1;
                if char1 != b.get(idx2..=idx2).unwrap() {
                    return false;
                }
            }
            if remove {
                idx1 += 1;
                if char2 != a.get(idx1..=idx1).unwrap() {
                    return false;
                }
            }
            change_found = true;
        }
        idx1 += 1;
        idx2 += 1;
    }
    true
}

fn same_length(a: &str, b: &str) -> bool
{
    let mut change_found: bool = false;
    for idx in 0..a.len() {
        if a.get(idx..=idx).unwrap() != b.get(idx..=idx).unwrap() {
            if change_found {
                return false;
            }
            change_found = true;
        }
    }
    true
}

fn off_by_one(a: &str, b: &str) -> bool
{
    let mut change_found: bool = false;
    let mut idx2 = 0;
    for idx1 in 0.. cmp::min(a.len(), b.len()) {
        let char1 = a.get(idx1..=idx1).unwrap();
        if char1 != b.get(idx2..=idx2).unwrap() {
            if change_found {
                return false;
            }
            idx2 += 1;
            if char1 != b.get(idx2..=idx2).unwrap() {
                return false;
            }
            change_found = true;
        }
        idx2 += 1;
    }
    true
}

fn is_one_edit_away_2(a: &str, b: &str) -> bool
{
    if ((a.len() as i32) - (b.len() as i32)).abs() > 1 {
        return false;
    }
    let insert: bool = (a.len() as i32) < (b.len() as i32);
    let remove: bool = (a.len() as i32) > (b.len() as i32);
    // insert and remove a character are complimentary operations
    if insert {
        off_by_one(a, b)
    } else if remove {
        off_by_one(b, a)
    } else {
        same_length(a, b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_one_edit_away() {
        assert_eq!(is_one_edit_away("taco", "taco"), true);
        assert_eq!(is_one_edit_away("taco", "teco"), true);
        assert_eq!(is_one_edit_away("taco", "tacos"), true);
        assert_eq!(is_one_edit_away("taco", "tasco"), true);
        assert_eq!(is_one_edit_away("taco", "tesco"), false);
        assert_eq!(is_one_edit_away("taco", "tco"), true);
        assert_eq!(is_one_edit_away("taco", "tac"), true);
        assert_eq!(is_one_edit_away("taco", "tec"), false);
        assert_eq!(is_one_edit_away("taco", "taca"), true);
        assert_eq!(is_one_edit_away("taco", "tcao"), false);
        //
        assert_eq!(is_one_edit_away_2("taco", "taco"), true);
        assert_eq!(is_one_edit_away_2("taco", "teco"), true);
        assert_eq!(is_one_edit_away_2("taco", "tacos"), true);
        assert_eq!(is_one_edit_away_2("taco", "tasco"), true);
        assert_eq!(is_one_edit_away_2("taco", "tesco"), false);
        assert_eq!(is_one_edit_away_2("taco", "tco"), true);
        assert_eq!(is_one_edit_away_2("taco", "tac"), true);
        assert_eq!(is_one_edit_away_2("taco", "tec"), false);
        assert_eq!(is_one_edit_away_2("taco", "taca"), true);
        assert_eq!(is_one_edit_away_2("taco", "tcao"), false);
    }
}

fn main() {
    //
}