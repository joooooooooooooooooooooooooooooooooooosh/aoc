pub fn part1(input: String) -> Option<usize> {
    let mut fish: Vec<u8> = Vec::new();
    input.split(',')
        .filter_map(|x|
            x.parse::<u8>().ok())
        .for_each(|x|
            fish.push(x));

    for _ in 0..80 {
        let mut new_fish = 0;
        fish.iter_mut().for_each(|x| 
            if *x == 0 {
                new_fish += 1;
                *x = 6;
            } else {
                *x -= 1;
            }
        );

        for _ in 0..new_fish {
            fish.push(8);
        }
    }

    Some(fish.len())
}

pub fn part2(input: String) -> Option<usize> {
    let mut days: Vec<usize> = vec![0; 10];
    input.split(',')
        .filter_map(|x|
            x.parse::<usize>().ok())
        .for_each(|x|
            days[x] += 1);

    for _ in 0..256 {
        // create new fish
        days[9] = days[0];
        // reset timers
        days[7] += days[0];

        for day in 1..=9 {
            days[day - 1] = days[day];
        }

        days[9] = 0;
    }

    let total = days.into_iter().sum();
    Some(total)
}

