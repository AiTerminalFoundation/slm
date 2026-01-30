use std::cmp::Ordering;

#[derive(Eq, PartialEq)]
pub struct PairFrequency {
    pub pair: [u32; 2],
    pub frequency: u32,
}

impl Ord for PairFrequency {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

impl PartialOrd for PairFrequency {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
