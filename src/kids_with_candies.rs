pub fn solution(candies: &[i32], extra_candies: i32) -> Vec<bool> {
    let mut result = Vec::with_capacity(candies.len());
    let greatest = candies.iter().max();
    if let Some(g) = greatest {
        for c in candies {
            result.push((*c + extra_candies) >= *g);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::kids_with_candies::solution;

    #[test]
    fn ex_1() {
        assert_eq!(
            solution(&[2, 3, 5, 1, 3], 3),
            [true, true, true, false, true],
        )
    }

    #[test]
    fn ex_2() {
        assert_eq!(
            solution(&[4, 2, 1, 1, 2], 1),
            [true, false, false, false, false],
        )
    }

    #[test]
    fn ex_3() {
        assert_eq!(solution(&[12, 1, 12], 10), [true, false, true],)
    }
}
