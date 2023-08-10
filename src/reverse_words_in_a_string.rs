pub fn solution(s: &str) -> String {
    let reversed: Vec<&str> = s.split_whitespace().rev().collect();
    reversed.join(" ")
}

#[cfg(test)]
mod tests {
    use crate::reverse_words_in_a_string::solution;

    #[test]
    fn ex_1() {
        assert_eq!(solution("the sky is blue"), "blue is sky the");
    }

    #[test]
    fn ex_2() {
        assert_eq!(solution("  hello world  "), "world hello");
    }

    #[test]
    fn ex_3() {
        assert_eq!(solution("a good   example"), "example good a");
    }
}
