pub fn part1(input: String) -> Option<u16> {
    let mut lines = input.split("\n");

    let mut prev_value: u16 = lines.next()?.parse().unwrap();
    let mut increases = 0;
    for line in lines {
        let value: u16 = match line.parse() {
            Ok(v) => v,
            Err(_) => 0,
        };

        if value > prev_value {
            increases += 1;
        }
        prev_value = value;
    }

    Some(increases)
}

pub fn part2(input: String) -> Option<u16> {
    let lines: Vec<&str> = input.split("\n").collect();
    let mut iter = lines.windows(3);

    let mut prev_value: u16 = get_sum(iter.next()?);
    let mut increases = 0;

    for line in iter {
        let value: u16 = get_sum(line);

        if value > prev_value {
            increases += 1;
        }
        prev_value = value;
    }

    Some(increases)
}

fn get_sum(window: &[&str]) -> u16 {
    let mut sum: u16 = 0;
    for val in window {
        let value: u16 = match val.parse() {
            Ok(v) => v,
            Err(_) => 0,
        };

        sum += value;
    }

    sum
}
