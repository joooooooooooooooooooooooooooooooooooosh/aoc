const BASE: u32 = 10;

pub fn part1(input: String) -> Option<u64> {
    let mut octopi = input.lines()
                          .map(|line|
                                line.chars()
                                    .map(|c| (c.to_digit(BASE).unwrap(), false))
                                    .collect::<Vec<_>>())
                          .collect::<Vec<_>>();

    let mut flashes = 0;
    const STEPS: usize = 100;
    for _ in 0..STEPS {
        let mut new_flash = true;
        octopi.iter_mut().for_each(|line|
            line.iter_mut().for_each(|octopus|{
                // bool represents if the octopus has flashed that round
                octopus.0 += 1;
                octopus.1 = false;
            })
        );

        while new_flash {
            new_flash = false;
            for (i, row) in octopi.clone().iter().enumerate() {
                for (j, octopus) in row.iter().enumerate() {
                    if octopus.0 > 9 && !octopus.1 {
                        new_flash = true;
                        flashes += 1;
                        octopi[i][j].0 += 1;
                        octopi[i][j].1 = true;
                        
                        if i > 0 {
                            octopi[i - 1][j].0 += 1;
                            if j > 0 {
                                octopi[i - 1][j - 1].0 += 1;
                            }
                            if j < row.len() - 1 {
                                octopi[i - 1][j + 1].0 += 1;
                            }
                        }
                        if i < octopi.len() - 1 {
                            octopi[i + 1][j].0 += 1;
                            if j > 0 {
                                octopi[i + 1][j - 1].0 += 1;
                            }
                            if j < row.len() - 1 {
                                octopi[i + 1][j + 1].0 += 1;
                            }
                        }
                        if j > 0 {
                            octopi[i][j - 1].0 += 1;
                        }
                        if j < row.len() - 1 {
                            octopi[i][j + 1].0 += 1;
                        }
                    }
                }
            }
        }

        octopi.iter_mut().for_each(|line|
            line.iter_mut().for_each(|octopus|{
                if octopus.0 > 9 {
                    octopus.0 = 0;
                }
            })
        );
    }

    Some(flashes)
}

pub fn part2(input: String) -> Option<usize> {
    let mut octopi = input.lines()
                          .map(|line|
                                line.chars()
                                    .map(|c| (c.to_digit(BASE).unwrap(), false))
                                    .collect::<Vec<_>>())
                          .collect::<Vec<_>>();

    for i in 1.. {
        let mut new_flash = true;
        let mut all_flash = true;
        octopi.iter_mut().for_each(|line|
            line.iter_mut().for_each(|octopus|{
                // bool represents if the octopus has flashed that round
                octopus.0 += 1;
                octopus.1 = false;
            })
        );

        while new_flash {
            new_flash = false;
            for (i, row) in octopi.clone().iter().enumerate() {
                for (j, octopus) in row.iter().enumerate() {
                    if octopus.0 > 9 && !octopus.1 {
                        new_flash = true;
                        octopi[i][j].0 += 1;
                        octopi[i][j].1 = true;
                        
                        if i > 0 {
                            octopi[i - 1][j].0 += 1;
                            if j > 0 {
                                octopi[i - 1][j - 1].0 += 1;
                            }
                            if j < row.len() - 1 {
                                octopi[i - 1][j + 1].0 += 1;
                            }
                        }
                        if i < octopi.len() - 1 {
                            octopi[i + 1][j].0 += 1;
                            if j > 0 {
                                octopi[i + 1][j - 1].0 += 1;
                            }
                            if j < row.len() - 1 {
                                octopi[i + 1][j + 1].0 += 1;
                            }
                        }
                        if j > 0 {
                            octopi[i][j - 1].0 += 1;
                        }
                        if j < row.len() - 1 {
                            octopi[i][j + 1].0 += 1;
                        }
                    }
                }
            }
        }

        octopi.iter_mut().for_each(|line|
            line.iter_mut().for_each(|octopus|{
                if octopus.0 > 9 {
                    octopus.0 = 0;
                } else {
                    all_flash = false;
                }
            })
        );

        if all_flash {
            return Some(i);
        }
    }

    None
}
