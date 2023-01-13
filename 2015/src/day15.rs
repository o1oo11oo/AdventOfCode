use itertools::Itertools;
use parse_display::FromStr;
use rayon::prelude::*;

pub(crate) fn part_1(input: &str) -> String {
    let ingredients = input
        .lines()
        .map(|l| l.parse::<Ingredient>().unwrap())
        .collect_vec();
    ingredients
        .iter()
        .skip(1)
        .map(|_| (0..=100))
        .multi_cartesian_product()
        .filter_map(|mut d| {
            let sum = d.iter().sum::<i64>();
            if sum <= 100 {
                d.push(100 - sum);
                Some(d)
            } else {
                None
            }
        })
        .collect_vec()
        .into_par_iter()
        .map(|distribution| {
            let scores = ingredients
                .iter()
                .zip(distribution)
                .map(|(ingredient, amount)| {
                    (
                        ingredient.capacity * amount,
                        ingredient.durability * amount,
                        ingredient.flavor * amount,
                        ingredient.texture * amount,
                    )
                })
                .fold((0, 0, 0, 0), |acc: (i64, i64, i64, i64), curr| {
                    (
                        acc.0 + curr.0,
                        acc.1 + curr.1,
                        acc.2 + curr.2,
                        acc.3 + curr.3,
                    )
                });
            scores.0.max(0) * scores.1.max(0) * scores.2.max(0) * scores.3.max(0)
        })
        .max()
        .unwrap()
        .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    let ingredients = input
        .lines()
        .map(|l| l.parse::<Ingredient>().unwrap())
        .collect_vec();
    ingredients
        .iter()
        .skip(1)
        .map(|_| (0..=100))
        .multi_cartesian_product()
        .filter_map(|mut d| {
            let sum = d.iter().sum::<i64>();
            if sum <= 100 {
                d.push(100 - sum);
                Some(d)
            } else {
                None
            }
        })
        .collect_vec()
        .into_par_iter()
        .filter_map(|distribution| {
            let scores = ingredients
                .iter()
                .zip(distribution)
                .map(|(ingredient, amount)| {
                    (
                        ingredient.capacity * amount,
                        ingredient.durability * amount,
                        ingredient.flavor * amount,
                        ingredient.texture * amount,
                        ingredient.calories * amount,
                    )
                })
                .fold((0, 0, 0, 0, 0), |acc: (i64, i64, i64, i64, i64), curr| {
                    (
                        acc.0 + curr.0,
                        acc.1 + curr.1,
                        acc.2 + curr.2,
                        acc.3 + curr.3,
                        acc.4 + curr.4,
                    )
                });
            (scores.4 == 500)
                .then_some(scores.0.max(0) * scores.1.max(0) * scores.2.max(0) * scores.3.max(0))
        })
        .max()
        .unwrap()
        .to_string()
}

#[derive(Debug, Clone, FromStr)]
#[display("{_name}: capacity {capacity}, durability {durability}, flavor {flavor}, texture {texture}, calories {calories}")]
struct Ingredient {
    _name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}
