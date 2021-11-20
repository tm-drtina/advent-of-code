use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;

enum Rule {
    Ref { i: i32 },
    Val { v: char },
}

fn rule_to_regex(rules: &HashMap<i32, Vec<Vec<Rule>>>, index: i32) -> String {
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
        regexes.into_iter().next().unwrap()
    } else {
        format!(
            "({})",
            regexes.into_iter().map(|s| format!("({})", s)).join("|")
        )
    }
}

pub fn run(input: &str) -> usize {
    let mut lines = input.lines();
    let mut rules: HashMap<i32, Vec<Vec<Rule>>> = HashMap::new();
    loop {
        let line = lines.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (index, r) = line.split_once(": ").unwrap();
        let ruleset = r
            .split(" | ")
            .map(|r2| {
                r2.split(' ')
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
    let regex_str = format!("^{}$", rule_to_regex(&rules, 0));
    let re = Regex::from_str(&regex_str).unwrap();

    lines.filter(|line| re.is_match(line)).count()
}
