use std::collections::HashMap;

pub fn part1(input: String) -> Option<i32> {
    let mut polymer = input.lines().next()?.to_string();
    let mut rules = HashMap::new();
    input.lines().skip(2)
        .for_each(|rule| {
            rules.insert(String::from(rule.split_whitespace().nth(0).unwrap()),
                         rule.split_whitespace().nth(2).unwrap());
        });

    for _ in 0..10 {
        let aaaaa = polymer.chars().collect::<Vec<char>>();
        let iter = aaaaa.windows(2);
        for (i, pair) in iter.enumerate() {
            let pair = pair.iter().collect::<String>();
            polymer.insert(i * 2 + 1, rules.get(&pair).unwrap().chars().next()?);
        }
    }

    let mut counts = HashMap::new();
    input.lines().skip(2)
        .for_each(|rule| {
            counts.insert(rule.split_whitespace().nth(2).unwrap()
                              .chars().next().unwrap(),
                          0);
        });

    for elem in polymer.chars() {
        let count = counts.get_mut(&elem)?;
        *count = *count + 1;
    }

    let mut count_list: Vec<i32> = counts.values().map(|x| *x).collect();
    count_list.sort();

    Some(count_list.last()? - count_list[0])
}

pub fn part2(input: String) -> Option<u64> {
    let result = 0;
    Some(result)
}

