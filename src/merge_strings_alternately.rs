/// Combines two strings `word1` and `word2` by iterating over
/// characters in each, and zipping them together.
///
/// If one input is longer than the other, its characters will be
/// appended to the end of the resulting string
pub fn merge_strings_alternately(word1: &str, word2: &str) -> String {
    let mut result = Vec::with_capacity(word1.len() + word2.len());

    let mut iter1 = word1.chars();
    let mut iter2 = word2.chars();

    loop {
        match (iter1.next(), iter2.next()) {
            (None, None) => break,
            (None, Some(c)) | (Some(c), None) => result.push(c),
            (Some(c1), Some(c2)) => {
                result.push(c1);
                result.push(c2);
            }
        }
    }

    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::merge_strings_alternately::merge_strings_alternately;

    #[test]
    fn example_1() {
        let word1 = "abc";
        let word2 = "pqr";
        assert_eq!(merge_strings_alternately(word1, word2), "apbqcr");
    }

    #[test]
    fn example_2() {
        let word1 = "ab";
        let word2 = "pqrs";
        assert_eq!(merge_strings_alternately(word1, word2), "apbqrs");
    }

    #[test]
    fn example_3() {
        let word1 = "abcd";
        let word2 = "pq";
        assert_eq!(merge_strings_alternately(word1, word2), "apbqcd");
    }
}
