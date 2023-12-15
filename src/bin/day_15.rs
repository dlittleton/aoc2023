use regex::Regex;

aoc2023::solver!(part1, part2);

fn part1(lines: &[String]) -> String {
    let total: u32 = lines[0].split(',').map(hash).sum();
    format!("{}", total)
}

fn part2(lines: &[String]) -> String {
    let re_instruction = Regex::new(r"(\w+)([=-])(\d*)").unwrap();

    let mut boxes: Vec<Vec<Lens>> = (0..256).map(|_| Vec::new()).collect();

    for inst in re_instruction.captures_iter(&lines[0]) {
        let label = inst.get(1).unwrap().as_str();
        let op = inst.get(2).unwrap().as_str();
        let h = hash(label) as usize;

        match op {
            "-" => {
                boxes[h].retain(|e| e.key != label);
            }
            "=" => {
                let value = inst.get(3).unwrap().as_str().parse::<u32>().unwrap();

                if let Some(entry) = boxes[h].iter_mut().find(|e| e.key == label) {
                    entry.value = value;
                } else {
                    boxes[h].push(Lens {
                        key: String::from(label),
                        value,
                    });
                }
            }
            x => panic!("Unexpected operation {}", x),
        };
    }

    let mut total = 0;
    for (box_num, lenses) in boxes.iter().enumerate() {
        for (lens_num, lens) in lenses.iter().enumerate() {
            total += (box_num + 1) * (lens_num + 1) * lens.value as usize;
        }
    }

    format!("{}", total)
}

#[derive(Debug)]
struct Lens {
    key: String,
    value: u32,
}

fn hash(s: &str) -> u32 {
    s.chars()
        .map(|c| c as u32)
        .fold(0, |a, b| ((a + b) * 17) % 256)
}
