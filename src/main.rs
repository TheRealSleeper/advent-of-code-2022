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
    let mut root = FsObject::new(0, FsType::Directory);

    todo!()
}

fn part2(inp: &str) -> isize {
    todo!()
}

struct FsObject {
    size: usize, 
    obj_type: FsType, 
    children: HashMap<String, FsObject>
}

enum FsType {
    File, 
    Directory, 
}

impl FsObject {
    fn new<'s, 'p>(size: usize, obj_type: FsType) -> FsObject {
        FsObject {
            size: size, 
            obj_type: obj_type, 
            children: HashMap::default()
        }
    }

    
}