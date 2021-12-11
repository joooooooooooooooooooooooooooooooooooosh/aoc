const BASE: u32 = 10;

pub fn part1(input: String) -> Option<u32> {
    // surely there's a better way than using charindices but i cbb
    let heightmap = input.lines()
                        .map(|line|
                            line.char_indices()
                                .filter_map(|x| (x.1).to_digit(BASE))
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

pub fn part2(input: String) -> Option<u64> {
    let result = 0;
    Some(result)
}
