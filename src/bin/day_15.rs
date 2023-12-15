aoc2023::solver!(part1);

fn part1(lines: &[String]) -> String {
    let total: u32 = lines[0]
        .split(',')
        .map(|instruction| {
            instruction
                .chars()
                .map(|c| c as u32)
                .fold(0, |a, b| ((a + b) * 17) % 256)
        })
        .sum();

    format!("{}", total)
}
