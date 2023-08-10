pub fn solution(s: &str) -> String {
    let mut vowels = Vec::with_capacity(s.len());

    for c in s.chars() {
        if is_vowel(c) {
            vowels.push(c);
        }
    }

    let mut result = Vec::with_capacity(s.len());

    for c in s.chars() {
        if is_vowel(c) {
            result.push(vowels.pop().unwrap());
        } else {
            result.push(c);
        }
    }

    result.into_iter().collect()
}

fn is_vowel(c: char) -> bool {
    match c.to_ascii_lowercase() {
        'a' | 'e' | 'i' | 'o' | 'u' => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::reverse_vowels_of_a_string::solution;

    #[test]
    fn ex_1() {
        assert_eq!(solution("hello"), "holle");
    }

    #[test]
    fn ex_2() {
        assert_eq!(solution("leetcode"), "leotcede");
    }
}
