use std::fs::read_to_string;
use std::collections::HashMap; 

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
    let mut root = FsObject {
        size: 0, 
        obj_type: FsType::Directory(HashMap::new()), 
        parent: None
    }; 

    todo!()
}

fn part2(inp: &str) -> isize {
    todo!()
}

struct FsObject<'itself, 'parent> {
    size: usize, 
    obj_type: FsType<'itself>, 
    parent: Option<&'itself FsObject<'parent, 'parent>>
}

enum FsType <'a> {
    File, 
    Directory(HashMap<String, FsObject<'a, 'a>>), 
}

enum StateMachine {
    Cd, 
    Ls, 
    Read
}