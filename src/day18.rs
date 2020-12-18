#[derive(Copy, Clone)]
enum Part {
    Part1,
    Part2,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Token {
    Lit(i64),
    Add,
    Mul,
    OpenParen,
    CloseParen,
}

#[derive(Debug)]
pub enum Expression {
    Lit(i64),
    Add(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
}

fn lex(s: &str) -> Vec<Token> {
    let mut result = vec![];
    let mut chars = s.chars().peekable();
    while let Some(char) = chars.next() {
        if char.is_ascii_whitespace() {
            continue;
        } else if char.is_ascii_digit() {
            let mut digits = String::new();
            digits.push(char);
            while let Some(char) = chars.peek() {
                if char.is_ascii_digit() {
                    digits.push(chars.next().unwrap())
                } else {
                    break;
                }
            }
            result.push(Token::Lit(digits.parse().unwrap()));
        } else {
            result.push(match char {
                '(' => Token::OpenParen,
                ')' => Token::CloseParen,
                '+' => Token::Add,
                '*' => Token::Mul,
                c => panic!("unknown token: {}", c),
            });
        }
    }
    result
}

fn consume_token(tokens: &[Token], expected: Token) -> &[Token] {
    let (first, rest) = tokens.split_first().unwrap();
    assert_eq!(first, &expected);
    rest
}

fn parse_number_or_parens(tokens: &[Token], part: Part) -> Option<(Expression, &[Token])> {
    match tokens.split_first() {
        Some((Token::OpenParen, rest)) => {
            let (expr, rest) = parse_expression(rest, part)?;
            let rest = consume_token(rest, Token::CloseParen);
            Some((expr, rest))
        }
        Some((Token::Lit(num), rest)) => {
            let expr = Expression::Lit(*num);
            Some((expr, rest))
        }
        _ => None,
    }
}

fn parse_expression_part1(tokens: &[Token]) -> Option<(Expression, &[Token])> {
    let (mut expr, mut tokens) = parse_number_or_parens(tokens, Part::Part1)?;
    loop {
        match tokens.split_first() {
            Some((Token::Add, rest)) => {
                let (rhs_expr, rest) = parse_number_or_parens(rest, Part::Part1)?;
                expr = Expression::Add(Box::new(expr), Box::new(rhs_expr));
                tokens = rest
            }
            Some((Token::Mul, rest)) => {
                let (rhs_expr, rest) = parse_number_or_parens(rest, Part::Part1)?;
                expr = Expression::Mul(Box::new(expr), Box::new(rhs_expr));
                tokens = rest;
            }
            _ => break,
        }
    }
    Some((expr, tokens))
}

fn parse_term_part2(tokens: &[Token]) -> Option<(Expression, &[Token])> {
    let (mut expr, mut tokens) = parse_number_or_parens(tokens, Part::Part2)?;
    loop {
        match tokens.split_first() {
            Some((Token::Add, rest)) => {
                let (rhs_expr, rest) = parse_number_or_parens(rest, Part::Part2)?;
                expr = Expression::Add(Box::new(expr), Box::new(rhs_expr));
                tokens = rest
            }
            _ => break,
        }
    }
    Some((expr, tokens))
}

fn parse_expression_part2(tokens: &[Token]) -> Option<(Expression, &[Token])> {
    let (mut expr, mut tokens) = parse_term_part2(tokens)?;
    loop {
        match tokens.split_first() {
            Some((Token::Mul, rest)) => {
                let (rhs_expr, rest) = parse_term_part2(rest)?;
                expr = Expression::Mul(Box::new(expr), Box::new(rhs_expr));
                tokens = rest;
            }
            _ => break,
        }
    }
    Some((expr, tokens))
}

fn parse_expression(tokens: &[Token], part: Part) -> Option<(Expression, &[Token])> {
    match part {
        Part::Part1 => parse_expression_part1(tokens),
        Part::Part2 => parse_expression_part2(tokens),
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
pub fn input_generator(input: &str) -> Vec<Vec<Token>> {
    input.lines().map(|line| lex(line)).collect()
}

#[aoc(day18, part1)]
pub fn part1(input: &[Vec<Token>]) -> i64 {
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
pub fn part2(input: &[Vec<Token>]) -> i64 {
    input
        .iter()
        .map(|line| {
            let (expr, rest) = parse_expression(line, Part::Part2).unwrap();
            assert!(rest.is_empty());
            eval_expression(&expr)
        })
        .sum()
}
