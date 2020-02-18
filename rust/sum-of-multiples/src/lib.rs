pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut all_factors: Vec<u32> = vec![];

    for item in factors {
        for i in 1..=limit {
            let cur = item * i;

            if cur < limit {
                all_factors.push(cur);
            } else {
                break;
            }
        }
    }

    all_factors.sort_unstable();
    all_factors.dedup();
    let sum: u32 = all_factors.iter().sum();

    sum
}
