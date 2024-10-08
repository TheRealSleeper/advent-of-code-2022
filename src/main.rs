use std::fs::read_to_string;

mod args;
#[cfg(test)]
mod tests;

fn main() {
    let options = args::get_args();
    let mut content = "".to_string();

    if let Some(i) = options.input {
        content = read_to_string(i).unwrap();
    }

    if options.part1 && options.use_sample {
        content = read_to_string("sample1.txt").unwrap();
    }

    if options.part2 && options.use_sample {
        content = read_to_string("sample2.txt").unwrap();
    }

    if options.part1 {
        println!("{}", part1(&content));
    }

    if options.part2 {
        println!("{}", part2(&content));
    }
}

fn part1(inp: &str) -> isize {
    inp.lines()
        .map(|l| {
            let (e1, e2) = l.split_once(',').unwrap();
            let r1 = get_range(e1);
            let r2 = get_range(e2);
            ranges_contained(r1, r2)
        })
        .filter(|b| *b)
        .count() as isize
}

fn part2(inp: &str) -> isize {
    inp.lines()
        .map(|l| {
            let (e1, e2) = l.split_once(',').unwrap();
            let r1 = get_range(e1);
            let r2 = get_range(e2);
            ranges_overlap(r1, r2)
        })
        .filter(|b| *b)
        .count() as isize
}

fn get_range(s: &str) -> (isize, isize) {
    let (lower, upper) = s.split_once('-').unwrap();
    (lower.parse().unwrap(), upper.parse().unwrap())
}

fn ranges_contained(r1: (isize, isize), r2: (isize, isize)) -> bool {
    (r1.0 >= r2.0 && r1.1 <= r2.1) || (r2.0 >= r1.0 && r2.1 <= r1.1)
}

fn ranges_overlap(r1: (isize, isize), r2: (isize, isize)) -> bool {
    (r1.0 <= r2.0 && r1.1 >= r2.0) || (r2.0 <= r1.0 && r2.1 >= r1.0)
}
