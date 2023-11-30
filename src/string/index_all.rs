/// Searches a string and returns all the indexs of the occurrence of the specified searched substring.
///
/// # Arguments
///
/// * `s` - The input string where to search.
/// * `search` - The substring to search for.
/// * `position` - The position to start searching.
///
/// # Returns
///
/// Returns index of the first occurrence of searched substring.
///
/// # Examples
///
/// ```
/// use ruf::string;
///
/// assert_eq!(vec![2, 3], string::index_all("hello", "ll", 0));
///
///
/// ```

pub fn index_all(s: impl AsRef<str>, search: &str, position: usize) -> Vec<usize> {
    if s.as_ref().is_empty() || search.is_empty() || s.as_ref().chars().count() <= position {
        return vec![];
    }

    let mut result = Vec::new();
    let mut index = crate::string::index(&s, search, 0);

    while index != -1 {
        println!("index is {:?}", index);

        result.push(index as usize);
        index = crate::string::index(&s, search, (index + 1) as usize);
    }

    // let sub_string = &s.as_ref()[s.as_ref().char_indices().nth(position).unwrap().0..];

    // println!("sub_string is {:?}", sub_string);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index_all() {
        // assert_eq!(true, index_all("hello", "", 0).is_empty());
        // assert_eq!(true, index_all("hello", "", 10).is_empty());

        // assert_eq!(vec![2, 3], index_all("hello", "l", 0));
        // assert_eq!(vec![2], index_all("hello", "ll", 0));

        // assert_eq!(vec![3], index_all("hello", "l", 3));

        assert_eq!(vec![0, 1], index_all("你好你好!", "你好", 0));
        // assert_eq!(7, index_all("你好hello你好!", "你好", 2));
        // assert_eq!(-1, index_all("你好hello你好!", "你好", 9));
        // assert_eq!(-1, index_all("你好hello你好!", "你好", 10));
    }
}
