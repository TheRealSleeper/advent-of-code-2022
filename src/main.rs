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
}

fn part1(inp: &str) -> isize {
    todo!()
}

fn part2(inp: &str) -> isize {
    todo!()
}

struct FsObject<'p> {
    name: String,
    size: usize,
    children: Option<HashMap<String, FsObject<'p>>>,
    parent: Option<&'p FsObject<'p>>
}

impl FsObject<'_> {
    fn new<'p>(name: String, size: usize) -> FsObject<'p> {
        FsObject {
            name,
            size,
            children: None,
            parent: None
        }
    }
    
    fn new_child<'p>(name: String, size: usize, parent: &'p FsObject) -> FsObject<'p> {
        FsObject {
            name,
            size,
            children: None,
            parent: Some(parent)
        }
    }

    fn add_child(&mut self, name: String, size: usize) {
        match &mut self.children {
            Some(c) => {
                c.insert(name.clone(), FsObject::new(name, size));
            }
            None => {
                self.children = Some(HashMap::from([(name.clone(), FsObject::new(name, size))]))
            }
        }
        
        todo!()
    }

    fn children_size(&self) -> usize {
        match &self.children {
            Some(c) => c
                .iter()
                .map(|(_, o)| {
                    if o.size > 0 {
                        o.size
                    } else {
                        o.children_size()
                    }
                })
                .sum::<usize>(),
            None => 0,
        }
    }

    fn sizes_recurse(&mut self) {
        match &mut self.children {
            Some(c) => c.values_mut().for_each(|o| o.sizes_recurse()),
            None => {}
        }

        self.size = self.children_size();
    }
}
