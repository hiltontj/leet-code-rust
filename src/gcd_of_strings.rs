/// Find the Greatest Common Divisor (GCD) of strings
///
/// For two strings `s1` and `s2`, find the largest string that
/// divides them both.
pub fn gcd_of_strings(s1: &str, s2: &str) -> String {
    // TODO - make loop index non-linear
    let end = s1.len().min(s2.len());
    println!("end: {end}");
    for i in DivisorRange::new(end).rev() {
        let (left, _) = s1.split_at(i);
        println!("left: {left}");
        if left.divides(s2) && left.divides(s1) {
            return left.to_string();
        }
        let (left, _) = s2.split_at(i);
        println!("left: {left}");
        if left.divides(s1) && left.divides(s2) {
            return left.to_string();
        }
    }
    Default::default()
}

#[derive(Debug)]
struct DivisorRange {
    number: usize,
    current: usize,
}

impl DivisorRange {
    fn new(number: usize) -> Self {
        Self { number, current: 1 }
    }
}

impl Iterator for DivisorRange {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let v = self.current;
        while self.current <= self.number {
            self.current += 1;
            if self.number % self.current == 0 || v == 1 {
                return Some(v);
            }
        }
        None
    }
}

impl DoubleEndedIterator for DivisorRange {
    fn next_back(&mut self) -> Option<Self::Item> {
        println!("next_back: {self:?}");
        let v = self.number / self.current;
        while self.current <= self.number {
            self.current += 1;
            if self.number % self.current == 0 || v == 1 {
                return Some(v);
            }
        }
        None
    }
}

trait Divisor<Rhs = Self> {
    fn divides(self, other: Rhs) -> bool;
}

impl<'a> Divisor for &'a str {
    fn divides(self, other: Self) -> bool {
        if self == other {
            return true;
        }
        if other.len() % self.len() != 0 {
            return false;
        }
        let mut check = String::new();
        for _ in 0..(other.len() / self.len()) {
            check.push_str(self);
        }
        check.as_str() == other
    }
}

#[cfg(test)]
mod tests {
    use crate::gcd_of_strings::gcd_of_strings;

    #[test]
    fn ex_1() {
        let s1 = "ABCABC";
        let s2 = "ABC";
        assert_eq!(gcd_of_strings(s1, s2), "ABC");
    }

    #[test]
    fn ex_2() {
        let s1 = "ABABAB";
        let s2 = "ABAB";
        assert_eq!(gcd_of_strings(s1, s2), "AB");
    }

    #[test]
    fn ex_3() {
        let s1 = "LEET";
        let s2 = "CODE";
        // More idiomatic to return None, but in this case this is what leet code wants:
        assert_eq!(gcd_of_strings(s1, s2), "");
    }

    #[test]
    fn case_1() {
        let s1 = "TAUXXTAUXXTAUXXTAUXXTAUXX";
        let s2 = "TAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXXTAUXX";
        assert_eq!(gcd_of_strings(s1, s2), "TAUXX");
    }

    #[test]
    fn case_2() {
        assert_eq!(gcd_of_strings("AA", "A"), "A");
    }
}
