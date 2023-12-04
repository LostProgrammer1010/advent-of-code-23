fn main() {
    let input = include_str!("input2.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let input = input
        .split_terminator('\n')
        .map(|value| {
            let value = value.replace("one", "1");
            let value = value.replace("two", "2");
            let value = value.replace("three", "3");
            let value = value.replace("four", "4");
            let value = value.replace("five", "5");
            let value = value.replace("six", "6");
            let value = value.replace("seven", "7");
            let value = value.replace("eight", "8");
            let value = value.replace("nine", "9");
            value
        })
        .map(|entry| {
            entry
                .chars()
                .filter(|c| c.is_numeric() || c.eq(&'\n'))
                .collect::<String>()
        })
        .map(|value| {
            (value.chars().nth(0).unwrap().to_string()
                + &value.chars().nth(value.len() - 1).unwrap().to_string())
                .parse::<i32>()
                .unwrap()
        })
        .collect::<Vec<_>>();
    println!("{:?}", input);
    0
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
