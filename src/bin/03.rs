fn parse_input(input: &str) -> Vec<&str> {
    input.lines().collect()
}

fn largest_joltage(bank: &str, number_battery: usize) -> u64 {
    let mut slice = &bank[..];
    let mut result = 0u64;
    for battery_index in 0..number_battery {
        let (index, digit) = slice
            .char_indices()
            .rev()
            .skip(number_battery - battery_index - 1)
            .max_by_key(|x| x.1)
            .unwrap();
        result = result * 10 + (digit.to_digit(10).unwrap() as u64);
        slice = &slice[index + 1..];
    }
    result
}

fn part1(data: &Vec<&str>) -> u64 {
    data.iter().map(|x| largest_joltage(x, 2)).sum()
}

fn part2(data: &Vec<&str>) -> u64 {
    data.iter().map(|x| largest_joltage(x, 12)).sum()
}

fn main() {
    let input = include_str!("../../inputs/03.txt");
    let data = parse_input(input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    mod part1 {
        use super::*;

        #[test]
        fn test_exemple_day3_part1() {
            let input = include_str!("../../examples/03.txt");
            let data = parse_input(input);
            assert_eq!(part1(&data), 357);
        }

        #[test]
        fn test_exemple_day3_part2() {
            let input = include_str!("../../examples/03.txt");
            let data = parse_input(input);
            assert_eq!(part2(&data), 3121910778619);
        }

        #[test]
        fn test_largest_joltage() {
            assert_eq!(largest_joltage("987654321111111", 2), 98);
            assert_eq!(largest_joltage("811111111111119", 2), 89);
            assert_eq!(largest_joltage("234234234234278", 2), 78);
            assert_eq!(largest_joltage("818181911112111", 2), 92);
        }
    }
}
