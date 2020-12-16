pub type Ticket = Vec<i32>;

#[derive(Debug)]
pub struct Field {
    name: String,
    ranges: Vec<(i32, i32)>,
}

impl Field {
    fn is_valid(&self, value: i32) -> bool {
        self.ranges
            .iter()
            .any(|&(start, end)| (start..=end).contains(&value))
    }
}

#[derive(Debug)]
pub struct Input {
    fields: Vec<Field>,
    my_ticket: Ticket,
    nearby_tickets: Vec<Ticket>,
}

fn parse_field(s: &str) -> Field {
    let mut parts = s.split(": ");
    let name = parts.next().unwrap().to_owned();
    let ranges = parts
        .next()
        .unwrap()
        .split(" or ")
        .map(parse_field_range)
        .collect();
    Field { name, ranges }
}

fn parse_field_range(s: &str) -> (i32, i32) {
    let mut parts = s.split('-');
    let start = parts.next().unwrap().parse().unwrap();
    let end = parts.next().unwrap().parse().unwrap();
    (start, end)
}

fn parse_ticket(s: &str) -> Ticket {
    s.split(',').map(|field| field.parse().unwrap()).collect()
}

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> Input {
    let mut lines = input.lines();

    let fields = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(parse_field)
        .collect();

    assert_eq!(lines.next().unwrap(), "your ticket:");
    let my_ticket = parse_ticket(lines.next().unwrap());
    assert_eq!(lines.next().unwrap(), "");

    assert_eq!(lines.next().unwrap(), "nearby tickets:");
    let nearby_tickets = lines.map(parse_ticket).collect();

    Input {
        fields,
        my_ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
pub fn part1(input: &Input) -> i32 {
    let invalid_values = input.nearby_tickets.iter().flat_map(|ticket| {
        ticket
            .iter()
            .filter(|&&value| !input.fields.iter().any(|field| field.is_valid(value)))
    });
    invalid_values.sum()
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> i32 {
    todo!()
}
