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
            let mut game = l.split(' ');
            game_score_p1((
                game.next().unwrap().chars().nth(0).unwrap(),
                game.next().unwrap().chars().nth(0).unwrap(),
            ))
        })
        .sum::<isize>()
}

fn part2(inp: &str) -> isize {
    inp.lines()
        .map(|l| {
            let mut game = l.split(' ');
            game_score_p2((
                game.next().unwrap().chars().nth(0).unwrap(),
                game.next().unwrap().chars().nth(0).unwrap(),
            ))
        })
        .sum::<isize>()
}

fn game_score_p1(game: (char, char)) -> isize {
    match game {
        ('A', 'X') => 4,
        ('A', 'Y') => 8,
        ('A', 'Z') => 3,
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,
        ('C', 'X') => 7,
        ('C', 'Y') => 2,
        ('C', 'Z') => 6,
        _ => panic!("Impossible game!"),
    }
}

fn game_score_p2(game: (char, char)) -> isize {
    match game {
        ('A', 'X') => 3,
        ('A', 'Y') => 4,
        ('A', 'Z') => 8,
        ('B', 'X') => 1,
        ('B', 'Y') => 5,
        ('B', 'Z') => 9,
        ('C', 'X') => 2,
        ('C', 'Y') => 6,
        ('C', 'Z') => 7,
        _ => panic!("Impossible game!"),
    }
}
