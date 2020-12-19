type Number = u64;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

#[derive(Debug, Clone, Copy)]
enum Token {
    Number(Number),
    Operator(Operator),
    LeftParenthesis,
    RightParenthesis,
}

fn tokens(input: &str) -> impl Iterator<Item = Token> + '_ {
    let mut it = input.char_indices().peekable();

    std::iter::from_fn(move || loop {
        match it.next()? {
            (start, c) if c.is_numeric() => {
                let mut end = start + 1;
                while let Some((i, c)) = it.peek() {
                    if c.is_numeric() {
                        it.next();
                    } else {
                        end = *i;
                        break;
                    }
                }
                let value = input[start..end].parse().unwrap();
                break Some(Token::Number(value));
            }
            (_, '+') => break Some(Token::Operator(Operator::Add)),
            (_, '*') => break Some(Token::Operator(Operator::Multiply)),
            (_, '(') => break Some(Token::LeftParenthesis),
            (_, ')') => break Some(Token::RightParenthesis),
            (_, ' ') => {}
            (_, c) => panic!("Unexpected value: {}", c),
        }
    })
}

fn rpn_tokens<'a>(
    tokens: impl IntoIterator<Item = Token> + 'a,
    operator_precedence: impl Fn(Operator) -> usize + 'a,
) -> impl Iterator<Item = Token> + 'a {
    let mut operators = Vec::new();
    let mut token_it = tokens.into_iter();

    enum State {
        Initial,
        Operator(Operator),
        RightParenthesis,
        Final,
    }

    let mut state = State::Initial;

    std::iter::from_fn(move || loop {
        match state {
            State::Initial => match token_it.next() {
                Some(token @ Token::Number(_)) => {
                    break Some(token);
                }
                Some(Token::Operator(operator)) => {
                    state = State::Operator(operator);
                }
                Some(token @ Token::LeftParenthesis) => {
                    operators.push(token);
                }
                Some(Token::RightParenthesis) => {
                    state = State::RightParenthesis;
                }
                None => {
                    state = State::Final;
                }
            },
            State::Operator(operator) => {
                if operators
                    .last()
                    .filter(|other_token| match other_token {
                        Token::Operator(other_operator) => {
                            operator_precedence(*other_operator) >= operator_precedence(operator)
                        }
                        _ => false,
                    })
                    .is_some()
                {
                    break Some(operators.pop().unwrap());
                } else {
                    operators.push(Token::Operator(operator));
                    state = State::Initial;
                }
            }
            State::RightParenthesis => {
                if operators
                    .last()
                    .filter(|token| !matches!(token, Token::LeftParenthesis))
                    .is_some()
                {
                    break Some(operators.pop().unwrap());
                } else {
                    if let Some(Token::LeftParenthesis) = operators.last() {
                        operators.pop().unwrap();
                    }
                    state = State::Initial;
                }
            }
            State::Final => {
                break operators.pop();
            }
        }
    })
}

fn evaluate(input: &str, operator_precedence: impl Fn(Operator) -> usize) -> Number {
    let mut stack = Vec::new();

    for rpn_token in rpn_tokens(tokens(input), operator_precedence) {
        match rpn_token {
            Token::Number(value) => {
                stack.push(value);
            }
            Token::Operator(operator) => {
                let lhs = stack.pop().unwrap();
                let rhs = stack.pop().unwrap();
                stack.push(match operator {
                    Operator::Add => lhs + rhs,
                    Operator::Multiply => lhs * rhs,
                });
            }
            _ => panic!("Unexpected token: {:?}", rpn_token),
        }
    }

    stack.pop().unwrap()
}

pub fn part1(input: &str) -> Number {
    let operator_precedence = |operator| match operator {
        Operator::Add => 1,
        Operator::Multiply => 1,
    };

    input
        .lines()
        .map(|line| evaluate(line, operator_precedence))
        .sum()
}

pub fn part2(input: &str) -> Number {
    let operator_precedence = |operator| match operator {
        Operator::Add => 2,
        Operator::Multiply => 1,
    };

    input
        .lines()
        .map(|line| evaluate(line, operator_precedence))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1("1 + 2 * 3 + 4 * 5 + 6"), 71);
        assert_eq!(part1("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part1("2 * 3 + (4 * 5)"), 26);
        assert_eq!(part1("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 437);
        assert_eq!(part1("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 12_240);
        assert_eq!(
            part1("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            13_632
        );
        assert_eq!(part1(INPUT), 3_348_222_486_398);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2("1 + 2 * 3 + 4 * 5 + 6"), 231);
        assert_eq!(part2("1 + (2 * 3) + (4 * (5 + 6))"), 51);
        assert_eq!(part2("2 * 3 + (4 * 5)"), 46);
        assert_eq!(part2("5 + (8 * 3 + 9 + 3 * 4 * 3)"), 1445);
        assert_eq!(part2("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"), 669_060);
        assert_eq!(
            part2("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"),
            23_340
        );
        assert_eq!(part2(INPUT), 43_423_343_619_505);
    }
}
