pub fn part1(input: String) -> Option<u64> {
    let input: Vec<&str> = input.lines().collect();
    let len = input.len();

    // assume length of 12
    const BIT_LEN: usize = 12;
    const MASK: u64 = 0xfff;
    let mut counts = [0; BIT_LEN];

    for line in input {
        for i in 0..BIT_LEN {
            if line.chars().nth(i) == Some('1') {
                counts[i] += 1;
            }
        }
    }

    let mut gamma = 0;
    for i in 0..BIT_LEN {
        if counts[i] * 2 > len {
            gamma |= 1 << (BIT_LEN - (i + 1));
        }
    }

    let epsilon = (!gamma) & MASK;
    Some(epsilon * gamma)
}

pub fn part2(input: String) -> Option<u64> {
    let input: Vec<&str> = input.lines().collect();

    let generator = get_rating(input.clone(), true);
    let scrubber = get_rating(input.clone(), false);

    Some(generator * scrubber)
}

fn get_rating(mut input: Vec<&str>, most_common: bool)  -> u64 {
    let mut i = 0;
    while input.len() > 1 {
        let bit = get_char_at_index(input.clone(), i, most_common);
        input = input.into_iter().filter(|line| {
            line.chars().nth(i) == Some(bit)
        }).collect();

        i += 1;
    }

    u64::from_str_radix(input[0], 2).unwrap()
}

fn get_char_at_index(input: Vec<&str>, index: usize, most_common: bool) -> char {
    let len = input.len();
    let mut count = 0;
    input.into_iter().for_each(|line| {
        if line.chars().nth(index) == Some('1') {
            count += 1;
        }
    });

    if count * 2 == len {
        if most_common {
            return '1';
        } else {
            return '0';
        }
    }

    if !most_common {
        count = len - count;
    }

    if count * 2 < len {
        '0'
    } else {
        '1'
    }
}
