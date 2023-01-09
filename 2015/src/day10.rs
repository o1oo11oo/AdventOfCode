pub(crate) fn part_1(input: &str) -> String {
    look_and_say_steps(
        input
            .trim()
            .as_bytes()
            .iter()
            .map(|c| c - b'0')
            .collect::<Vec<_>>(),
        40,
    )
    .len()
    .to_string()
}

pub(crate) fn part_2(input: &str) -> String {
    look_and_say_steps(
        input
            .trim()
            .as_bytes()
            .iter()
            .map(|c| c - b'0')
            .collect::<Vec<_>>(),
        50,
    )
    .len()
    .to_string()
}

fn look_and_say_steps(nums: Vec<u8>, steps: usize) -> Vec<u8> {
    itertools::iterate(nums, look_and_say_step)
        .skip(1)
        .take(steps)
        .last()
        .unwrap()
}

fn look_and_say_step(nums: &Vec<u8>) -> Vec<u8> {
    let mut new = Vec::with_capacity(nums.len() * 27 / 20);
    let mut idx = 0;
    while idx < nums.len() {
        if idx + 2 < nums.len() && nums[idx] == nums[idx + 1] && nums[idx + 1] == nums[idx + 2] {
            new.extend([3, nums[idx]]);
            idx += 3;
        } else if idx + 1 < nums.len() && nums[idx] == nums[idx + 1] {
            new.extend([2, nums[idx]]);
            idx += 2;
        } else {
            new.extend([1, nums[idx]]);
            idx += 1;
        }
    }
    new
}
