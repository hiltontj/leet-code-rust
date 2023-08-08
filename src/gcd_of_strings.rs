/// Find the Greatest Common Divisor (GCD) of strings
///
/// For two strings `s1` and `s2`, find the largest string that
/// divides them both.
pub fn gcd_of_strings(s1: &str, s2: &str) -> String {
    let size = s1.len().min(s2.len());
    for i in divisors_of(size).rev() {
        let (left, _) = s1.split_at(i);
        if left.divides(s2) && left.divides(s1) {
            return left.to_string();
        }
        let (left, _) = s2.split_at(i);
        if left.divides(s1) && left.divides(s2) {
            return left.to_string();
        }
    }
    Default::default()
}

/// Helper function for creating a new [`DivisorRange`]
fn divisors_of(number: usize) -> DivisorRange {
    DivisorRange::new(number)
}

/// Generate a range of divisors
///
/// Given a number, this will allow one to iterate over that number's even divisors.
///
/// # Usage
/// ```ignore
/// for i in divisors_of(12) {
///     print!("{i} ");
/// }
/// // prints: 1 2 3 4 6 12
/// ```
///
/// [`DivisorRange`] implements [`DoubleEndedIterator`] so that you can iterate over
/// divisors in reverse:
/// ```ignore
/// let number = 6;
/// for i in divisors_of(6).rev() {
///     print!("{i} ");
/// }
/// // prints: 6 3 2 1
/// ```
#[derive(Debug)]
struct DivisorRange {
    /// The number, the divisors of which will be iterated over
    number: usize,
    /// Used for tracking iteration location
    current: usize,
}

impl DivisorRange {
    /// Create a new `DivisorRange` for the given `number`
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

/// A trait to define if one things _divides_ another
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
        for Window(start, end) in SlidingWindow::new(0, other.len(), self.len()) {
            if &other[start..end] != self {
                return false;
            }
        }
        true
    }
}

/// A sliding window iterator
///
/// Used to generate indicies for a sliding window. A window, of `size`, starting from the index `start`,
/// is slid along a range of indices, ending at `end`.
#[derive(Debug)]
struct SlidingWindow {
    /// The starting index of the window
    start: usize,
    /// The ending index of the total range being considered
    end: usize,
    /// The size of the window
    size: usize,
}

impl SlidingWindow {
    /// Create a new [`SlidingWindow`]
    fn new(start: usize, end: usize, size: usize) -> Self {
        Self { start, end, size }
    }
}

impl Iterator for SlidingWindow {
    type Item = Window;

    fn next(&mut self) -> Option<Self::Item> {
        if self.start + self.size > self.end {
            None
        } else {
            let window = Window(self.start, self.start + self.size);
            self.start += self.size;
            Some(window)
        }
    }
}

/// The iterator item for [`SlidingWindow`]
struct Window(usize, usize);

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
