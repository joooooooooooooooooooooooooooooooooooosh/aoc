pub fn part1(input: String) -> Option<i32> {
    let mut crabs: Vec<i32> = Vec::new();
    input.split(',')
        .filter_map(|x|
            x.parse::<i32>().ok())
        .for_each(|x|
            crabs.push(x));

    let max = *crabs.iter().max().unwrap() as usize;
    let mut costs = vec![0; max];

    for pos in 0..max {
        let mut cost = 0;
        crabs.iter().for_each(|x| 
            cost += (*x - pos as i32).abs()
        );

        costs[pos] = cost;
    }

    Some(costs.into_iter().min().unwrap())
}

pub fn part2(input: String) -> Option<i32> {
    let mut crabs: Vec<i32> = Vec::new();
    input.split(',')
        .filter_map(|x|
            x.parse::<i32>().ok())
        .for_each(|x|
            crabs.push(x));

    let max = *crabs.iter().max().unwrap() as usize;
    let mut costs = vec![0; max];

    for pos in 0..max {
        let mut cost = 0;
        crabs.iter().for_each(|x| {
            let diff = (*x - pos as i32).abs();
            cost += (diff * (1 + diff))/2;
        });

        costs[pos] = cost;
    }

    Some(costs.into_iter().min().unwrap())
}
