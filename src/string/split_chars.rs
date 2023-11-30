/// Split the input string into an array of characters.
///
///
/// # Arguments
///
/// * `s` - The input string to split into characters.
///
/// # Returns
///
/// Returns a vector containing the characters extracted from the input string.
///
/// # Example
///
/// ```rust
/// use ruf::string;
///
/// let chars1 = string::split_chars("hello");
/// assert_eq!(chars1, vec!["h", "e", "l", "l", "o"]);
///
/// let chars2 = string::split_chars("你好");
/// assert_eq!(chars2, vec!["你", "好"]);
///
/// ```

pub fn split_chars(s: &str) -> Vec<&str> {
    if s.is_empty() {
        return vec![""];
    }

    s.split_terminator("").skip(1).collect::<Vec<_>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_chars() {
        assert_eq!(vec![""], split_chars(""));
        assert_eq!(vec!["h", "e", "l", "l", "o"], split_chars("hello"));
        assert_eq!(vec!["S", "z", "e", "ś", "ć"], split_chars("Sześć"));
        assert_eq!(vec!["你", "好"], split_chars("你好"));
    }
}
