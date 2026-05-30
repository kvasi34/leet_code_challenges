pub fn str_str(haystack: String, needle: String) -> i32 {
    if needle.len() > haystack.len() {
        return -1;
    }

    for i in 0..=(haystack.len() - needle.len()) {
        if haystack[i..i + needle.len()] == needle {
            return i as i32;
        }
    }

    -1
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    // LeetCode examples
    #[test]
    fn needle_found() {
        assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
    }

    #[test]
    fn needle_not_found() {
        assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
    }

    #[test]
    fn needle_at_end() {
        assert_eq!(str_str("hello".to_string(), "ll".to_string()), 2);
    }

    // Custom cases
    #[test]
    fn needle_equals_haystack() {
        assert_eq!(str_str("abc".to_string(), "abc".to_string()), 0);
    }

    #[test]
    fn needle_longer_than_haystack() {
        assert_eq!(str_str("ab".to_string(), "abc".to_string()), -1);
    }

    #[test]
    fn empty_needle() {
        assert_eq!(str_str("hello".to_string(), "".to_string()), 0);
    }
}
