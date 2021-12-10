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

pub fn part2(input: String) -> Option<u64> {
    let result = 0;
    Some(result)
}

