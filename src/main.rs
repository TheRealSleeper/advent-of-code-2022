use std::collections::HashMap;
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

    part1(&content);
}

fn part1(inp: &str) -> isize {
    let priority_map: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|x| (x.1, x.0 + 1))
        .collect();

    inp.lines()
        .map(|l| {
            let first_compartment: HashMap<char, _> =
                l[0..l.len() / 2].chars().map(|c| (c, 0)).collect();

            let second_compartment: HashMap<char, _> =
                l[l.len() / 2..l.len()].chars().map(|c| (c, 0)).collect();

            for (c, _) in first_compartment {
                if second_compartment.contains_key(&c) {
                    return priority_map[&c] as isize;
                }
            }

            unreachable!("No duplicate was found!")
        })
        .sum::<isize>()
}

fn part2(inp: &str) -> isize {
    let priority_map: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|x| (x.1, x.0 + 1))
        .collect();

    inp.lines()
        .collect::<Vec<&str>>()
        .chunks_exact(3)
        .map(|group| {
            for c in group[0].chars() {
                if group[1].contains(c) && group[2].contains(c) {
                    return priority_map[&c] as isize;
                }
            }
            unreachable!("No badge found!")
        })
        .sum::<isize>()
}
