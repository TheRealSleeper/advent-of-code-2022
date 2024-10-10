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

fn part1(inp: &str) -> String {
    let (stacks_str, moves_str) = inp.split_once("\n\n").unwrap();
    let mut stacks = get_stacks(stacks_str);
    let mut m = moves_str.lines().collect::<Vec<&str>>();
    m.pop();
    rearrange(moves_str, &mut stacks);

    // rearrange(m.iter().map(|s| *s).collect::<Vec<&str>>().join("\r\n").as_str(), &mut stacks);
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn part2(inp: &str) -> String {
    let (stacks_str, moves_str) = inp.split_once("\n\n").unwrap();
    let mut stacks = get_stacks(stacks_str);
    let mut m = moves_str.lines().collect::<Vec<&str>>();
    m.pop();
    rearrange_multi_pickup(moves_str, &mut stacks);

    // rearrange(m.iter().map(|s| *s).collect::<Vec<&str>>().join("\r\n").as_str(), &mut stacks);
    stacks.iter().map(|s| s.last().unwrap()).collect()
}

fn get_stacks(inp: &str) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::default();

    inp.lines().for_each(|l| {
        l.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .enumerate()
            .for_each(|(i, x)| {
                let stack = match stacks.get_mut(i) {
                    Some(s) => s,
                    None => {
                        stacks.push(Vec::new());
                        stacks.get_mut(i).unwrap()
                    }
                };

                if !x[1].is_whitespace() {
                    stack.push(x[1]);
                }
            });
    });

    for stack in &mut stacks {
        stack.reverse();
    }

    stacks
}

fn rearrange(inp: &str, stacks: &mut Vec<Vec<char>>) {
    inp.lines().for_each(|l| {
        let mut s = l.split_whitespace();
        let count = s.nth(1).unwrap().parse::<usize>().unwrap();
        let from = s.nth(1).unwrap().parse::<usize>().unwrap();
        let to = s.nth(1).unwrap().parse::<usize>().unwrap();
        
        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    });
}

fn rearrange_multi_pickup(inp: &str, stacks: &mut Vec<Vec<char>>) {
    inp.lines().for_each(|l| {
        let mut s = l.split_whitespace();
        let count = s.nth(1).unwrap().parse::<usize>().unwrap();
        let from = s.nth(1).unwrap().parse::<usize>().unwrap();
        let to = s.nth(1).unwrap().parse::<usize>().unwrap();
        let mut picked_up: Vec<char> = Vec::default(); 
        
        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            picked_up.push(c);
        }
        
        picked_up.reverse();
            stacks[to - 1].append(&mut picked_up);
    });
}
