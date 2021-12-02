pub fn part1(input: String) -> Option<i32> {
    let mut horizontal = 0;
    let mut vertical = 0;

    let lines = input.lines();
    for line in lines {
        let movement = line.split(" ").nth(0)?;
        let change: &mut i32;
        let direction;
        match movement {
            "forward" => {
                change = &mut horizontal;
                direction = 1;
            },
            "down" => {
                change = &mut vertical;
                direction = 1;
            },
            "up" => {
                change = &mut vertical;
                direction = -1;
            },
            _ => unreachable!(),
        }

        let distance: u16 = line.split(" ").nth(1)?.parse().unwrap();
        *change += direction * distance as i32;
    }

    let result = horizontal * vertical;
    Some(result)
}

pub fn part2(input: String) -> Option<u16> {
    let result = 0;
    Some(result)
}
