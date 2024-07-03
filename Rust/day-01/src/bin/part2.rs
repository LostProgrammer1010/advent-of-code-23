fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> u32 {
    input
        .split_terminator('\n')
        .map(|entry| {
            entry
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e")
        })
        .filter_map(|entry| {
            let mut start = 0;
            let mut end = 0;
            for c in entry.chars() {
                if c.is_numeric() {
                    if start == 0 {
                        start = c.to_digit(10).unwrap() * 10;
                        end = c.to_digit(10).unwrap();
                    } else {
                        end = c.to_digit(10).unwrap();
                    }
                }
            }
            Some(start + end)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn example_test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        assert_eq!(part2(input), 281);
    }
}
