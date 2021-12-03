pub fn part1(input: String) -> Option<u64> {
    let input: Vec<&str> = input.lines().collect();
    let len = input.len();

    // assume length of 13
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

pub fn part2(input: String) -> Option<u16> {
    let result = 0;
    Some(result)
}

