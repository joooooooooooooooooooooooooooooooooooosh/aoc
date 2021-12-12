use std::collections::HashMap;

pub fn part1(input: String) -> Option<u64> {
    let mut brackets = HashMap::new();
    brackets.insert('(', ')');
    brackets.insert('[', ']');
    brackets.insert('{', '}');
    brackets.insert('<', '>');

    let mut scores = HashMap::new();
    scores.insert(')', 3);
    scores.insert(']', 57);
    scores.insert('}', 1197);
    scores.insert('>', 25137);

    let mut syntax_score = 0;
    let mut stack: Vec<char> = Vec::new();

    for line in input.lines() {
        for c in line.chars() {
            if brackets.contains_key(&c) {
                stack.push(c);
            } else {
                let opener = stack.pop().unwrap_or(c);
                if brackets[&opener] != c {
                    syntax_score += scores[&c];
                    break;
                }
            }
        }
    }

    Some(syntax_score)
}

pub fn part2(input: String) -> Option<u64> {
    let result = 0;
    Some(result)
}

