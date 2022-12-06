struct Path {
    start: Cave,
    end: Cave,
}

struct Cave {
    name: String,
    is_visited: bool,
}

fn can_visit(cave: Cave) -> bool {
    cave.is_visited == false || cave.name.chars()
                                    .next()
                                    .unwrap()
                                    .is_uppercase()
}

fn paths_from_cave(paths: Vec<Path>, cave: String) -> Vec<Path> {
    let mut new_paths = Vec::new();
    for path in paths {
        if path.start.name == cave {
            // TODO: might need to clone
            new_paths.push(path);
        }
    }
    new_paths
}

fn find_paths(paths: Vec<Path>, cave: String) -> Vec<Path> {
    // TODO: might need to clone
    let mut paths = paths;
    let mut full_paths = Vec::new();
    // for path in paths_from_cave(paths, cave) {
    //     if can_visit(path.end) {
    //         for _path in find_paths(paths, path.end.name) {
    //             full_paths.push(_path);
    //         }
    //     }
    // }

    full_paths
}

pub fn part1(input: String) -> Option<usize> {
    let paths = input.lines().map(|line|
        Path {
            start: Cave {
                name: line.split("-").next().unwrap().to_string(),
                is_visited: false,
            },
            end: Cave {
                name: line.split("-").nth(1).unwrap().to_string(),
                is_visited: false,
            },
        }
    ).collect::<Vec<Path>>();

    let all_paths = find_paths(paths, String::from("start"));

    Some(all_paths.len())
}

pub fn part2(input: String) -> Option<u64> {
    let result = 0;
    Some(result)
}
