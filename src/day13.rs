use modinverse::modinverse;

type Input = (i32, Vec<Option<i32>>);

#[aoc_generator(day13)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();
    let earliest_time = lines.next().unwrap().parse::<i32>().unwrap();
    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .map(|bus| {
            if bus == "x" {
                None
            } else {
                Some(bus.parse().unwrap())
            }
        })
        .collect();
    (earliest_time, buses)
}

#[aoc(day13, part1)]
pub fn part1((earliest_time, buses): &Input) -> i32 {
    let (next_bus, next_waiting_time) = buses
        .iter()
        .filter_map(|bus| *bus)
        .map(|bus| {
            let rem = earliest_time % bus;
            let waiting_time = if rem == 0 { 0 } else { bus - rem };
            (bus, waiting_time)
        })
        .min_by_key(|(_, waiting_time)| *waiting_time)
        .unwrap();
    next_bus * next_waiting_time
}

#[aoc(day13, part2)]
pub fn part2((_, buses): &Input) -> i64 {
    // Solve equation system: x % bus[i] = waiting_time[i] (for all i)
    let bus_times: Vec<(i64, i64)> = buses
        .iter()
        .enumerate()
        .filter_map(|(offset, bus)| {
            bus.map(|bus| {
                // All bus IDs are prime numbers, and therefore also co-prime
                debug_assert!(primal::is_prime(bus as u64));
                let bus = bus as i64;
                let rem = (offset as i64) % bus;
                let waiting_time = if rem == 0 { 0 } else { bus - rem };
                (bus, waiting_time)
            })
        })
        .collect();
    // Product of all bus[i]
    let big_n = bus_times.iter().map(|(bus, _)| *bus).product::<i64>();
    // Solve using Chinese remainder theorem
    let earliest_time = bus_times
        .into_iter()
        .map(|(n, rem)| {
            let m = big_n / n;
            let m_inv = modinverse(m, n).unwrap();
            rem * m_inv * m
        })
        .sum::<i64>()
        % big_n;
    earliest_time
}
