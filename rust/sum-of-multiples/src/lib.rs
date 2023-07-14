pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .into_iter()
        .filter(|n| factors.iter().any(|a| a > &0 && n % a == 0))
        .sum()
}
