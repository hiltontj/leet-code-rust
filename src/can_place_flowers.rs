pub fn solution(flowerbed: Vec<i32>, n: i32) -> bool {
    let flowerbed = Flowerbed::from(flowerbed);

    let mut available: i32 = 0;
    for can_plant in flowerbed.scanner() {
        if can_plant {
            available += 1;
        }
    }

    n <= available
}

#[derive(Debug)]
struct Flowerbed(Vec<bool>);

// TODO - should be fallible to protect when empty vec or other
// constraints not met.
impl From<Vec<i32>> for Flowerbed {
    fn from(fb: Vec<i32>) -> Self {
        Self(fb.into_iter().map(|i| i > 0).collect())
    }
}

impl Flowerbed {
    fn scanner(self) -> FlowerbedScanner {
        FlowerbedScanner { bed: self, curr: 0 }
    }

    fn get(&self, index: usize) -> Option<bool> {
        self.0.get(index).copied()
    }
}

#[derive(Debug)]
struct FlowerbedScanner {
    bed: Flowerbed,
    curr: usize,
}

impl Iterator for FlowerbedScanner {
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        println!("next: {self:?}");
        let len = self.bed.0.len();
        if self.curr >= len {
            None
        } else if self.curr == 0 {
            let can_plant =
                !(self.bed.get(self.curr).unwrap() || self.bed.get(self.curr + 1).unwrap_or(false));
            self.bed.0[self.curr] = self.bed.0[self.curr] || can_plant;
            self.curr += 1;
            Some(can_plant)
        } else {
            let can_plant = !(self.bed.get(self.curr - 1).unwrap_or(false)
                || self.bed.get(self.curr).unwrap()
                || self.bed.get(self.curr + 1).unwrap_or(false));
            self.bed.0[self.curr] = self.bed.0[self.curr] || can_plant;
            self.curr += 1;
            Some(can_plant)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::can_place_flowers::solution;

    #[test]
    fn ex_1() {
        assert!(solution(vec![1, 0, 0, 0, 1], 1));
        assert!(!solution(vec![1, 0, 0, 0, 1], 2));
        assert!(!solution(vec![1, 0, 0, 0, 0, 1], 2));
        assert!(solution(vec![1, 0, 0, 0, 1, 0, 0], 2));
    }
}
