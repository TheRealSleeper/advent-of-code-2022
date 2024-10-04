#[derive(Debug, Clone)]
pub struct Args {
    pub part1: bool,
    pub part2: bool,
    pub use_sample: bool,
    pub input: Option<String>,
}

pub fn get_args() -> Args {
    let mut env_args = std::env::args().skip(1);

    let mut args = Args {
        part1: false,
        part2: false,
        use_sample: false,
        input: None,
    };

    let help =
        "This program should solve an Advent Of Code challenge. Use the following commands: \
                        -i | --input {path}: Provides a path the a file to be used as input
                        -1 | --part1: Solves part 1
                        -2 | --part2: Solves solving part 2
                        -s | --sample: Uses sample input
                        -h | --help: Prints this help document";

    while let Some(arg) = env_args.next() {
        let mut chars = arg.chars();
        if chars.nth(0).unwrap() != '-' {
            panic!("Invalid argument!");
        }

        while let Some(c) = chars.next() {
            if c == '-' {
                match chars.collect::<String>().as_str() {
                    "input" => {
                        args.input = Some(
                            env_args
                                .next()
                                .expect("Expected path for input to be provided"),
                        )
                    }
                    "sample" => args.use_sample = true,
                    "part1" => args.part1 = true,
                    "part2" => args.part2 = true,
                    "help" => println!("{}", help),
                    _ => panic!("Invalid argument!"),
                }
                break;
            } else {
                match c {
                    'i' => {
                        args.input = Some(
                            env_args
                                .next()
                                .expect("Expected path for input to be provided"),
                        )
                    }
                    's' => args.use_sample = true,
                    '1' => args.part1 = true,
                    '2' => args.part2 = true,
                    'h' => println!("{}", help),
                    _ => panic!("Invalid argument!"),
                }
            }
        }
    }

    args
}
