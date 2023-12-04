fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let number_list: String = input
        .chars()
        .filter(|c| c.is_numeric() || c.eq(&'\n'))
        .collect::<String>();
    let mut number_list: Vec<&str> = number_list.split("\n").collect();
    number_list.pop();

    let number_list: i32 = number_list
        .into_iter()
        .map(|value| {
            (value.chars().nth(0).unwrap().to_string()
                + &value.chars().nth(value.len() - 1).unwrap().to_string())
                .parse::<i32>()
                .unwrap()
        })
        .sum();
    number_list
}
//test

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
