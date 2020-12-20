use std::{collections::BTreeMap, fmt};

use lazy_static::lazy_static;

type RuleRef = usize;

#[derive(Debug, Clone)]
enum Rule<'a> {
    Text(&'a str),
    Refs(&'a str),
}

fn parse_rule(s: &str) -> Option<(RuleRef, Rule<'_>)> {
    use regex::Regex;

    lazy_static! {
        static ref CHAR_RE: Regex = Regex::new(r#"^(?P<index>\d+): "(?P<text>.+)"$"#).unwrap();
        static ref REFS_RE: Regex = Regex::new(r#"^(?P<index>\d+): (?P<refs>.+)$"#).unwrap();
    }

    if let Some(caps) = CHAR_RE.captures(s) {
        let index = caps.name("index").unwrap().as_str().parse().unwrap();
        let text = caps.name("text").unwrap().as_str();
        Some((index, Rule::Text(text)))
    } else if let Some(caps) = REFS_RE.captures(s) {
        let index = caps.name("index").unwrap().as_str().parse().unwrap();
        let refs = caps.name("refs").unwrap().as_str();
        Some((index, Rule::Refs(refs)))
    } else {
        None
    }
}

type RuleMap<'a> = BTreeMap<RuleRef, Rule<'a>>;

fn parse_input(input: &str) -> RuleMap {
    input.lines().filter_map(parse_rule).collect()
}

fn create_validator_pattern(rules: &RuleMap) -> Result<String, fmt::Error> {
    use fmt::Write;
    let mut pattern = String::new();
    write!(&mut pattern, "(?(DEFINE)")?;
    writeln!(&mut pattern)?;
    for (rule_ref, rule) in rules {
        write!(&mut pattern, "  (?P<r{}>", rule_ref)?;
        match rule {
            Rule::Text(text) => {
                write!(&mut pattern, "{}", regex::escape(text))?;
            }
            Rule::Refs(refs) => {
                for (index, rule_refs) in refs.split(" | ").enumerate() {
                    if index > 0 {
                        write!(&mut pattern, "|")?;
                    }
                    for rule_ref in rule_refs.split(' ') {
                        write!(&mut pattern, "(?P>r{})", rule_ref)?;
                    }
                }
            }
        }
        write!(&mut pattern, ")")?;
        writeln!(&mut pattern)?;
    }
    write!(&mut pattern, ")")?;
    writeln!(&mut pattern)?;
    write!(&mut pattern, "^(?P>r0)$")?;
    // println!("{}", &pattern);
    Ok(pattern)
}

fn create_validator<'a>(rules: &RuleMap<'a>) -> impl Fn(&str) -> bool + 'a {
    use pcre2::bytes::RegexBuilder;

    let pattern = create_validator_pattern(rules).unwrap();

    let regex = RegexBuilder::new()
        .jit_if_available(true)
        .extended(true)
        .build(&pattern)
        .unwrap();

    move |s| regex.is_match(s.as_bytes()).unwrap()
}

pub fn part1(input: &str) -> usize {
    let rules = parse_input(input);

    let validate = create_validator(&rules);

    input.lines().filter(|line| validate(*line)).count()
}

pub fn part2(input: &str) -> usize {
    let mut rules = parse_input(input);

    rules.extend(parse_rule("8: 42 | 42 8"));
    rules.extend(parse_rule("11: 42 31 | 42 11 31"));

    let validate = create_validator(&rules);

    input.lines().filter(|line| validate(*line)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT_0: &str = include_str!("test_input_0.txt");
    static TEST_INPUT_1: &str = include_str!("test_input_1.txt");
    static INPUT: &str = include_str!("input.txt");

    #[test]
    fn part1_works() {
        assert_eq!(part1(TEST_INPUT_0), 2);
        assert_eq!(part1(TEST_INPUT_1), 3);
        assert_eq!(part1(INPUT), 195);
    }

    #[test]
    fn part2_works() {
        assert_eq!(part2(TEST_INPUT_1), 12);
        assert_eq!(part2(INPUT), 309);
    }
}
