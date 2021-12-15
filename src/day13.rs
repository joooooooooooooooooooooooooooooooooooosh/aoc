const GRID_SIZE: usize = 2000;

pub fn part1(input: String) -> Option<usize> {
    let mut dots: Vec<Vec<bool>> = vec![vec![false; GRID_SIZE]; GRID_SIZE];
    let mut input = input.lines();

    while let Some(line) = input.next() {
        if line == "" {
            break;
        }
        let point: Vec<usize> = line.split(",")
                              .map(|x| x.parse().unwrap())
                              .collect();
        dots[point[1]][point[0]] = true;
    }

    let mut line = input.next()?;
    line = line.split(' ').skip(2).next()?;
    let direction = line.chars().next()?;
    let fold: usize = line.split('=').skip(1).next()?.parse().unwrap();

    for (i, row) in dots.clone().iter().enumerate() {
        for (j, dot) in row.iter().enumerate() {
            if *dot {
                if i > fold && direction == 'y' {
                    dots[i][j] = false;
                    dots[(fold - (i - fold))][j] = true;
                } else if j > fold && direction == 'x' {
                    dots[i][j] = false;
                    dots[i][(fold - (j - fold))] = true;
                }
            }
        }
    }

    Some(dots.iter()
            .map(|row|
                    row.iter()
                        .filter(|x| **x)
                        .count())
            .sum::<_>())
}

pub fn part2(input: String) -> Option<u64> {
    let mut dots: Vec<Vec<bool>> = vec![vec![false; GRID_SIZE]; GRID_SIZE];
    let mut input = input.lines();

    while let Some(line) = input.next() {
        if line == "" {
            break;
        }
        let point: Vec<usize> = line.split(",")
                              .map(|x| x.parse().unwrap())
                              .collect();
        dots[point[1]][point[0]] = true;
    }

    while let Some(mut line) = input.next() {
        line = line.split(' ').skip(2).next()?;
        let direction = line.chars().next()?;
        let fold: usize = line.split('=').skip(1).next()?.parse().unwrap();

        for (i, row) in dots.clone().iter().enumerate() {
            for (j, dot) in row.iter().enumerate() {
                if *dot {
                    if i > fold && direction == 'y' {
                        dots[i][j] = false;
                        dots[(fold - (i - fold))][j] = true;
                    } else if j > fold && direction == 'x' {
                        dots[i][j] = false;
                        dots[i][(fold - (j - fold))] = true;
                    }
                }
            }
        }
    }

    // no way i'm gonna bother parsing this when i have eyes
    for row in dots.iter().take(6) {
        for dot in row.into_iter().take(40) {
            print!("{}", if *dot { '#' } else { '.' });
        }
        println!();
    }

    None
}

