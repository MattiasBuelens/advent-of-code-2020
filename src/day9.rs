use std::cmp::Ordering;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<i64> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn is_valid(num: i64, previous: &[i64]) -> bool {
    previous.iter().any(|&left| {
        let right = num - left;
        previous.contains(&right)
    })
}

fn find_first_invalid(seq: &[i64], preamble_len: usize) -> Option<i64> {
    seq.windows(preamble_len + 1).find_map(|window| {
        let (&num, previous) = window.split_last().unwrap();
        if is_valid(num, previous) {
            None
        } else {
            Some(num)
        }
    })
}

#[aoc(day9, part1)]
pub fn part1(input: &[i64]) -> i64 {
    find_first_invalid(input, 25).unwrap()
}

#[aoc(day9, part2)]
pub fn part2(input: &[i64]) -> i64 {
    let target = part1(input);
    let mut left = 0usize;
    let mut right = 0usize;
    let mut sum = 0i64;
    while right < input.len() {
        debug_assert!(left <= right);
        debug_assert_eq!(sum, input[left..right].iter().sum());
        match sum.cmp(&target) {
            Ordering::Equal => {
                // match!
                let range = &input[left..right];
                assert!(range.len() >= 2);
                return range.iter().min().unwrap() + range.iter().max().unwrap();
            }
            Ordering::Less => {
                // sum is too small, add the next number
                sum += input[right];
                right += 1;
            }
            Ordering::Greater => {
                // sum is too big, subtract the first number
                sum -= input[left];
                left += 1;
            }
        }
    }
    panic!("no contiguous range found");
}
