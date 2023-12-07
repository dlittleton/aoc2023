use aoc2023::util::get_all_numbers;

aoc2023::solver!(part1);

fn part1(lines: &[String]) -> String {
    let times: Vec<i32> = get_all_numbers(&lines[0]);
    let distances: Vec<i32> = get_all_numbers(&lines[1]);
    let mut wins: Vec<_> = Vec::new();

    // Distance Traveled
    // (t - x) * x

    for (i, t) in times.into_iter().enumerate() {
        let target = distances[i];
        let mut count = 0;

        for x in 0..t {
            if (t - x) * x > target {
                count += 1;
            }
        }

        wins.push(count)
    }

    let result: i32 = wins.iter().product();

    format!("{}", result)
}
