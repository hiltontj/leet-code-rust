pub fn is_subsequence(s: &str, t: &str) -> bool {
    let mut s_chars = s.chars().peekable();
    let t_chars = t.chars();
    for t_c in t_chars {
        if let Some(s_c) = s_chars.peek() {
            if t_c == *s_c {
                s_chars.next();
            }
        } else {
            return true;
        }
    }
    s_chars.peek().is_none()
}

#[cfg(test)]
mod tests {
    use crate::is_subsequence::is_subsequence;

    #[test]
    fn ex_1() {
        assert!(is_subsequence("abc", "ahbgdc"));
    }

    #[test]
    fn ex_2() {
        assert!(!is_subsequence("axc", "ahbgdc"));
    }
}
