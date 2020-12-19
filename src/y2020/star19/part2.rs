use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

enum Rule {
    Ref { i: i32 },
    Val { v: char },
}

fn rule_to_regex(rules: &HashMap<i32, Vec<Vec<Rule>>>, index: i32) -> String {
    if index == 8 || index == 11 {
        panic!()
    }
    let ruleset = rules.get(&index).unwrap();
    let regexes: Vec<String> = ruleset
        .iter()
        .map(|rule| {
            rule.iter()
                .map(|r| match r {
                    Rule::Ref { i } => rule_to_regex(rules, *i),
                    Rule::Val { v } => String::from(*v),
                })
                .join("")
        })
        .collect();
    if regexes.len() == 1 {
        regexes[0].clone()
    } else {
        format!("({})", regexes.join("|"))
    }
}

fn shortest_match(rules: &HashMap<i32, Vec<Vec<Rule>>>, index: i32) -> usize {
    let ruleset = rules.get(&index).unwrap();
    ruleset
        .iter()
        .map(|rule| {
            rule.iter()
                .map(|r| match r {
                    Rule::Ref { i } => shortest_match(rules, *i),
                    Rule::Val { v: _ } => 1,
                })
                .sum()
        })
        .min()
        .unwrap()
}

pub fn run(input: &str) -> usize {
    let mut lines = input.lines();
    let mut rules: HashMap<i32, Vec<Vec<Rule>>> = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line == "" {
            break;
        }
        let (index, r) = line.split_once(": ").unwrap();
        let ruleset = r
            .split(" | ")
            .map(|r2| {
                r2.split(" ")
                    .map(|rule| match rule {
                        "\"a\"" => Rule::Val { v: 'a' },
                        "\"b\"" => Rule::Val { v: 'b' },
                        _ => Rule::Ref {
                            i: i32::from_str(rule).unwrap(),
                        },
                    })
                    .collect()
            })
            .collect();
        rules.insert(i32::from_str(index).unwrap(), ruleset);
    }

    let rule42 = rule_to_regex(&rules, 42);
    let rule31 = rule_to_regex(&rules, 31);

    let shortest_match_rule42 = shortest_match(&rules, 42);
    let shortest_match_rule31 = shortest_match(&rules, 31);

    const MAX_LINE_LENGTH: usize = 100;

    let re_second_part = (1..)
        .take_while(|i| {
            shortest_match_rule42 + i * (shortest_match_rule31 + shortest_match_rule42)
                < MAX_LINE_LENGTH
        })
        .map(|i| format!("({})", rule42.repeat(i) + &rule31.repeat(i)))
        .join("|");

    let re = Regex::from_str(&format!("^({})+({})$", rule42, re_second_part)).unwrap();

    lines.filter(|line| re.is_match(line)).count()
}
