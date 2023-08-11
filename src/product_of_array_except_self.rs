//! This does not solve the follow up to solve this with O(1) extra space complexity
pub fn solution(nums: &[i32]) -> Vec<i32> {
    println!("nums: {nums:?}");
    let mut prefix_products: Vec<Option<i32>> = Vec::with_capacity(nums.len());
    for (i, _) in nums.iter().enumerate() {
        if i == 0 {
            prefix_products.push(None);
        } else if i == 1 {
            prefix_products.push(Some(nums[i - 1]));
        } else {
            prefix_products.push(Some(prefix_products[i - 1].unwrap() * nums[i - 1]));
        }
    }
    println!("prefix products: {prefix_products:?}");

    let mut suffix_products: Vec<Option<i32>> = Vec::with_capacity(nums.len());
    for (i, _) in nums.iter().rev().enumerate() {
        if i == 0 {
            suffix_products.push(None);
        } else if i == 1 {
            suffix_products.push(Some(nums[nums.len() - 1]));
        } else {
            suffix_products.push(Some(suffix_products[i - 1].unwrap() * nums[nums.len() - i]));
        }
    }
    println!("suffix products: {suffix_products:?}");

    let mut result = Vec::with_capacity(nums.len());
    for (pp, sp) in prefix_products
        .into_iter()
        .zip(suffix_products.into_iter().rev())
    {
        result.push(match (pp, sp) {
            (None, None) => panic!("should not have None in both prefix and suffix products"),
            (None, Some(r)) | (Some(r), None) => r,
            (Some(p), Some(s)) => p * s,
        });
    }
    result
}

#[cfg(test)]
mod tests {
    use crate::product_of_array_except_self::solution;

    #[test]
    fn ex_1() {
        assert_eq!(solution(&[1, 2, 3, 4]), vec![24, 12, 8, 6]);
    }

    #[test]
    fn ex_2() {
        assert_eq!(solution(&[-1, 1, 0, -3, 3]), vec![0, 0, 9, 0, 0]);
    }
}
