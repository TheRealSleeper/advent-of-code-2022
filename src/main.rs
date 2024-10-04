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
    inp.replace("\r\n", "\n")
        .split("\n\n")
        .map(|e| {
            e.lines()
                .map(|l| l.trim().parse::<u32>().unwrap())
                .sum::<u32>() as isize
        })
        .max()
        .unwrap()
}

fn part2(inp: &str) -> isize {
    let mut answer = inp.replace("\r\n", "\n")
        .split("\n\n")
        .map(|e| {
            e.lines()
                .map(|l| l.trim().parse::<u32>().unwrap())
                .sum::<u32>() as isize
        })
        .collect::<Vec<isize>>(); 
    
    answer.sort();

    answer.last_chunk::<3>().unwrap().iter().sum()
}
