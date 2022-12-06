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
    let mut brackets = HashMap::new();
    brackets.insert('(', ')');
    brackets.insert('[', ']');
    brackets.insert('{', '}');
    brackets.insert('<', '>');

    let mut scores = HashMap::new();
    scores.insert(')', 1);
    scores.insert(']', 2);
    scores.insert('}', 3);
    scores.insert('>', 4);

    let mut lines: Vec<&str> = input.lines().collect();
    let mut stack: Vec<char> = Vec::new();

    for line in input.lines() {
        for c in line.chars() {
            if brackets.contains_key(&c) {
                stack.push(c);
            } else {
                let opener = stack.pop().unwrap_or(c);
                if brackets[&opener] != c {
                    lines.retain(|&x| x != line);
                }
            }
        }
    }

    // could probably merge these two loops into one, but eh
    let mut syntax_scores = vec![0; lines.len()];
    for (i, line) in lines.iter().enumerate() {
        stack.clear();
        for c in line.chars() {
            if brackets.contains_key(&c) {
                stack.push(c);
            } else {
                stack.pop();
            }
        }

        for unpaired in stack.iter().rev() {
            let pair = brackets[unpaired];
            syntax_scores[i] *= 5;
            syntax_scores[i] += scores[&pair];
        }
    }

    syntax_scores.sort();
    Some(syntax_scores[syntax_scores.len() / 2])
}
