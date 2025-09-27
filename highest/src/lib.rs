#[derive(Debug,Clone)]
pub struct Numbers<'a> {
    numbers: &'a [u32],
}

impl<'a> Numbers<'a> {
    pub fn new(numbers: &'a [u32]) -> Self {
        Self { numbers }
    }

    pub fn list(&self) -> &[u32] {
        self.numbers
    }

    pub fn latest(&self) -> Option<u32> {
        self.numbers.to_vec().into_iter().min()
    }

    pub fn highest(&self) -> Option<u32> {
        self.numbers.to_vec().into_iter().max()
    }

    pub fn highest_three(&self) -> Vec<u32> {
        let mut sorted_numbers: Vec<u32> = self.numbers.iter().copied().collect();
        sorted_numbers.sort_by(|a, b| b.cmp(a));
        sorted_numbers.into_iter().take(3).collect()
    }
}
