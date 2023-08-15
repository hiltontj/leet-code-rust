pub fn solution(nums: &mut Vec<i32>) {
    let mut i = 0;
    let mut j = nums.len() - 1;
    let mut swapping = false;
    while i < nums.len() {
        while j > i {
            match (nums[i], nums[j], swapping) {
                (0, _, _) => {
                    nums.swap(i, j);
                    swapping = true;
                }
                (_, 0, _) | (_, _, false) => (),
                (_, _, true) => nums.swap(i, j),
            }
            j -= 1;
        }
        swapping = false;
        i += 1;
        j = nums.len() - 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex_1() {
        let mut nums = vec![0, 1, 0, 3, 12];
        solution(&mut nums);
        assert_eq!(nums, [1, 3, 12, 0, 0]);
    }

    #[test]
    fn ex_2() {
        let mut nums = vec![0];
        solution(&mut nums);
        assert_eq!(nums, [0]);
    }

    #[test]
    fn ex_3() {
        let mut nums = vec![0, 0, 0, 1, 2, 3];
        solution(&mut nums);
        assert_eq!(nums, [1, 2, 3, 0, 0, 0]);
    }

    #[test]
    fn ex_4() {
        let mut nums = vec![1, 0, 0, 0, 0, 2];
        solution(&mut nums);
        assert_eq!(nums, [1, 2, 0, 0, 0, 0]);
    }
}
