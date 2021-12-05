use std::mem::swap;

const GRID_SIZE: usize = 1000;

pub fn part1(input: String) -> Option<usize> {
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    for line in input.lines() {
        let mut words = line.split(',');
        let mut x1 = words.next()?.parse::<usize>().ok()?;
        let mut inner = words.next()?.split_whitespace();
        let mut y1 = inner.next()?.parse::<usize>().ok()?;
        let mut x2 = inner.skip(1).next()?.parse::<usize>().ok()?;
        let mut y2 = words.next()?.parse::<usize>().ok()?;

        // i should really find a better way to do all this
        if x1 != x2 && y1 != y2 {
            continue;
        }
        
        if x1 > x2 {
            swap(&mut x1, &mut x2);
        }

        if y1 > y2 {
            swap(&mut y1, &mut y2);
        }

        for x in x1..=x2 {
            for y in y1..=y2 {
                grid[y][x] += 1;
            }
        }
    }

    let mut result = 0;
    grid.into_iter().for_each(|row| {
        result += row.into_iter().filter(|point| {
            *point >= 2
        }).count();
    });

    Some(result)
}

pub fn part2(input: String) -> Option<u64> {
    let result = 0;
    Some(result)
}

