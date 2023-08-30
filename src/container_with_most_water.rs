pub fn solution(height: Vec<i32>) -> i32 {
    let mut vol = 0;
    let mut i = 0;
    let mut j = height.len() - 1;
    loop {
        let h = height[i].min(height[j]);
        vol = vol.max(h * i32::try_from(j - i).unwrap());
        if j == i + 1 {
            break vol;
        } else if height[i] < height[j] {
            i += 1;
        } else if height[i] > height[j] {
            j -= 1;
        } else {
            i += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::container_with_most_water::solution;

    #[test]
    fn ex_1() {
        assert_eq!(solution(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }
}
