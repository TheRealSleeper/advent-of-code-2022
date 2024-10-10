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
    find_unique(inp, 4)
}

fn part2(inp: &str) -> isize {
    find_unique(inp, 14)
}

fn find_unique(inp: &str, size: usize) -> isize {
    let chars = inp.chars().collect::<Vec<char>>();
    let mut i = 0;

    'outer: while i < chars.len() {
        let mut found = [false; 256];

        for ii in (0..size).rev() {
            if found[chars[i + ii] as usize] {
                i += ii + 1;
                continue 'outer;
            }

            found[chars[i + ii] as usize] = true;
        }

        return (i + size) as isize;
    }

    -1
}
