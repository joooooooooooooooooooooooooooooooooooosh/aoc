const BASE: u32 = 10;

pub fn part1(input: String) -> Option<u32> {
    let heightmap = input.lines()
                        .map(|line|
                            line.chars()
                                .filter_map(|x| x.to_digit(BASE))
                                .collect::<Vec<_>>())
                        .collect::<Vec<_>>();

    let mut low_points: Vec<u32> = Vec::new();
    for (i, row) in heightmap.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            if i != 0 && num >= heightmap[i-1][j] {
                continue;
            }
            if j != 0 && num >= heightmap[i][j-1] {
                continue;
            }
            if i != heightmap.len() - 1 && num >= heightmap[i+1][j] {
                continue;
            }
            if j != row.len() - 1 && num >= heightmap[i][j+1] {
                continue;
            }

            low_points.push(num);
        }
    }

    Some(low_points.iter()
                .map(|x| x + 1)
                .sum())
}

pub fn part2(input: String) -> Option<u32> {
    let mut heightmap = input.lines()
                            .map(|line|
                                line.chars()
                                    .filter_map(|x| x.to_digit(BASE))
                                    .collect::<Vec<_>>())
                            .collect::<Vec<_>>();

    let mut basins: Vec<u32> = Vec::new();
    for (i, row) in heightmap.clone().iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            if i != 0 && num >= heightmap[i-1][j] {
                continue;
            }
            if j != 0 && num >= heightmap[i][j-1] {
                continue;
            }
            if i != heightmap.len() - 1 && num >= heightmap[i+1][j] {
                continue;
            }
            if j != row.len() - 1 && num >= heightmap[i][j+1] {
                continue;
            }

            basins.push(calculate_size(&mut heightmap, i, j));
        }
    }

    basins.sort();
    basins.reverse();
    Some(basins.iter()
            .take(3)
            .fold(1, |acc, x| acc * x))
}

fn calculate_size(heightmap: &mut Vec<Vec<u32>>, row: usize, col: usize) -> u32 {
    if heightmap[row][col] == 9 {
        return 0;
    }

    let mut size = 1;
    // prevent double counting
    heightmap[row][col] = 9;

    if row != 0 {
        size += calculate_size(heightmap, row - 1, col);
    }
    if col != 0 {
        size += calculate_size(heightmap, row, col - 1);
    }
    if row != heightmap.len() - 1 {
        size += calculate_size(heightmap, row + 1, col);
    }
    if col != heightmap[row].len() - 1 {
        size += calculate_size(heightmap, row, col + 1);
    }

    size
}
