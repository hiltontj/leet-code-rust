const SIZE: usize = 3;

pub fn solution(nums: &[i32]) -> bool {
    let mut stack = Vec::<&i32>::with_capacity(SIZE);
    for n in nums {
        println!("stack: {stack:?}");
        match stack.partition_point(|e| *e < n) {
            SIZE => return true,
            i if i == stack.len() => stack.push(n),
            i => stack[i] = n,
        }
    }
    stack.len() == SIZE
}

#[cfg(test)]
mod tests {
    use crate::increasing_triplet_subsequence::solution;

    #[test]
    fn ex_1() {
        assert!(solution(&[1, 2, 3, 4, 5]));
    }

    #[test]
    fn ex_2() {
        assert!(!solution(&[5, 4, 3, 2, 1]));
    }

    #[test]
    fn ex_3() {
        assert!(solution(&[2, 1, 5, 0, 4, 6]));
    }

    #[test]
    fn other() {
        assert!(!solution(&[0, 0, 0, 0, 0, 0]));
        assert!(solution(&[0, 1, 0, 2, 0, 0]));
        assert!(solution(&[0, -1, 0, 3, 0, 0]));
        assert!(!solution(&[10, 20, 11, 0, 5, 0]));
        assert!(solution(&[0, 1, 0, 0, 2]));
    }
}
