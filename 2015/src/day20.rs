use rayon::prelude::{IntoParallelIterator, ParallelIterator};

pub(crate) fn part_1(input: &str) -> String {
    let target: u64 = input.trim().parse().unwrap();
    (target / 100..target / 10)
        .into_par_iter()
        .find_first(|num| sum_of_prime_factors(num, 1) * 10 >= target)
        .unwrap()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let target: u64 = input.trim().parse().unwrap();
    (target / 100..target / 10)
        .into_par_iter()
        .find_first(|num| sum_of_prime_factors(num, (num + 49) / 50) * 11 >= target)
        .unwrap()
        .to_string()
}

fn sum_of_prime_factors(num: &u64, start: u64) -> u64 {
    (start..=num / 2).filter(|i| num % i == 0).sum::<u64>() + num
}
