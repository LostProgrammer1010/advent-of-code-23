fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> u32 {
    input
        .split_terminator('\n')
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
mod tests {
    use super::*;
    #[test]
    fn example_test() {
        let result = part1(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
",
        );
        assert_eq!(result, 142);
    }
}
