#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<i32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(expenses: &[i32]) -> i32 {
    for x in expenses {
        for y in expenses {
            if (x + y) == 2020 {
                return x * y;
            }
        }
    }
    unreachable!()
}

#[aoc(day1, part2)]
pub fn part2(expenses: &[i32]) -> i32 {
    for x in expenses {
        for y in expenses {
            for z in expenses {
                if (x + y + z) == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    unreachable!()
}
