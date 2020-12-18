#[derive(Copy, Clone)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug)]
pub enum Expression {
    Lit(i64),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
}

fn skip_whitespace(s: &str) -> &str {
    s.trim_start_matches(' ')
}

fn parse_number(mut s: &str) -> Option<(i64, &str)> {
    s = skip_whitespace(s);
    let mut len = 0;
    for char in s.chars() {
        if char.is_ascii_digit() {
            len += 1;
        } else {
            break;
        }
    }
    let (num, rest) = s.split_at(len);
    num.parse().ok().map(|num| (num, rest))
}

fn parse_term(mut s: &str, part: Part) -> Option<(Expression, &str)> {
    s = skip_whitespace(s);
    if s.is_empty() {
        return None;
    }
    if let Some(s) = s.strip_prefix('(') {
        let (expr, s) = parse_expression(s, part)?;
        let s = s.strip_prefix(')')?;
        Some((expr, s))
    } else {
        let (num, s) = parse_number(s)?;
        let expr = Expression::Lit(num);
        Some((expr, s))
    }
}

fn parse_expression_part1(mut s: &str) -> Option<(Expression, &str)> {
    s = skip_whitespace(s);
    if s.is_empty() {
        return None;
    }
    let (mut expr, mut s) = parse_term(s, Part::Part1)?;
    loop {
        s = skip_whitespace(s);
        if let Some(rest) = s.strip_prefix('+') {
            let (rhs_expr, rest) = parse_term(rest, Part::Part1)?;
            expr = Expression::Add(Box::new(expr), Box::new(rhs_expr));
            s = rest
        } else if let Some(rest) = s.strip_prefix('*') {
            let (rhs_expr, rest) = parse_term(rest, Part::Part1)?;
            expr = Expression::Mul(Box::new(expr), Box::new(rhs_expr));
            s = rest;
        } else {
            break;
        }
    }
    Some((expr, s))
}

fn parse_expression_part2(mut s: &str) -> Option<(Expression, &str)> {
    todo!()
}

fn parse_expression(s: &str, part: Part) -> Option<(Expression, &str)> {
    match part {
        Part::Part1 => parse_expression_part1(s),
        Part::Part2 => parse_expression_part2(s),
    }
}

fn eval_expression(expr: &Expression) -> i64 {
    match expr {
        Expression::Lit(num) => *num,
        Expression::Add(lhs, rhs) => eval_expression(lhs) + eval_expression(rhs),
        Expression::Mul(lhs, rhs) => eval_expression(lhs) * eval_expression(rhs),
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|line| line.to_owned()).collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[String]) -> i64 {
    input
        .iter()
        .map(|line| {
            let (expr, rest) = parse_expression(line, Part::Part1).unwrap();
            assert!(rest.is_empty());
            eval_expression(&expr)
        })
        .sum()
}

#[aoc(day18, part2)]
pub fn part2(input: &[String]) -> i64 {
    todo!()
}
