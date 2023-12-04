pub fn run<F1, F2>(part1: F1, part2: F2)
where
    F1: Fn(&[String]) -> String,
    F2: Fn(&[String]) -> String,
{
    env_logger::init();

    let args = match crate::input::Args::parse() {
        Ok(v) => v,
        Err(e) => panic!("Failed to parse args {}", e),
    };

    let lines = args.read_input_file();

    let result = if !args.part_two {
        part1(&lines)
    } else {
        part2(&lines)
    };
    println!("{}", result);
}
