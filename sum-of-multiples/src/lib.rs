extern crate itertools;
use itertools::Itertools;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    factors
        .into_iter()
        .flat_map(|&x| (x..limit).into_iter().filter(move |y| y % x == 0))
        .unique()
        .sum()
}
