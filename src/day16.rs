pub type Ticket = Vec<i32>;

#[derive(Debug, Clone)]
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

fn assign_fields<'a>(fields: &[Field], tickets: &[Ticket]) -> Vec<(Field, usize)> {
    let mut assigned = Vec::<(Field, usize)>::new();
    let mut fields = fields.to_vec();
    let mut indexes = (0..tickets[0].len()).collect::<Vec<usize>>();
    assert_eq!(fields.len(), indexes.len());
    'outer: while !fields.is_empty() {
        for (i, field) in fields.iter().enumerate() {
            let candidate_indexes = indexes
                .iter()
                .filter(|&&index| tickets.iter().all(|ticket| field.is_valid(ticket[index])))
                .cloned()
                .collect::<Vec<_>>();
            if let [field_index] = candidate_indexes[..] {
                // only one candidate index for this field, assign it
                assigned.push((field.clone(), field_index));
                fields.remove(i);
                indexes.remove(indexes.iter().position(|&x| x == field_index).unwrap());
                continue 'outer;
            }
        }
        panic!("could not assign field!");
    }
    assigned
}

#[aoc(day16, part2)]
pub fn part2(input: &Input) -> i64 {
    let valid_tickets = input
        .nearby_tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|&value| input.fields.iter().any(|field| field.is_valid(value)))
        })
        .cloned()
        .collect::<Vec<_>>();

    let assignments = assign_fields(&input.fields, &valid_tickets);
    assignments
        .into_iter()
        .filter(|(field, _)| field.name.starts_with("departure"))
        .map(|(_, index)| input.my_ticket[index] as i64)
        .product()
}
