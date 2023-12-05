fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    input
        .split_terminator('\n')
        .map(|entry| {
            let mut value = String::from(entry);
            value = value
                .replace("one", "o1e")
                .replace("two", "t2o")
                .replace("three", "t3e")
                .replace("four", "f4r")
                .replace("five", "f5e")
                .replace("six", "s6x")
                .replace("seven", "s7n")
                .replace("eight", "e8t")
                .replace("nine", "n9e");
            value + "\n"
        })
        .collect::<String>()
        .chars()
        .filter(|c| c.is_numeric() || c.eq(&'\n'))
        .collect::<String>()
        .split_terminator('\n')
        .map(|value: &str| {
            (value.chars().nth(0).unwrap().to_string()
                + &value.chars().nth(value.len() - 1).unwrap().to_string())
                .parse::<i32>()
                .unwrap()
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
