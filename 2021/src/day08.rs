pub fn part1(input: String) -> Option<usize> {
    let input = input.lines()
                .map(|line|
                    line.split('|')
                    .nth(1).unwrap()
                    .split(' '));

    let unique_lens = vec![2, 3, 4, 7];
    let mut count = 0;
    for line in input {
        count += line.filter(|word| 
                        unique_lens.contains(&word.len())
                    ).count();
    }
    Some(count)
}

pub fn part2(input: String) -> Option<usize> {
    let input = input.lines()
                .map(|line|
                    line.split('|'));

    let mut mapping = vec!{'z'; 8};
    for mut line in input {
        let mut i = line.next()?.split(' ')
                   .filter(|x| x.len() > 0);
        println!("{:?}", i.next());

        let mut o = line.next()?.split(' ')
                   .filter(|x| x.len() > 0);
    }

    Some(0)
}
