#[derive(Debug)]
struct Range {
    start: u64,
    end: u64,
}

#[derive(Debug)]
struct Data {
    ranges: Vec<Range>,
    ingredients: Vec<u64>,
}

fn parse_input(input: &str) -> Data {
    let lines: Vec<&str> = input.lines().collect();
    let mut index = 0;
    let mut ranges = Vec::new();
    while !lines[index].is_empty() {
        let (start_str, end_str) = lines[index].split_once('-').unwrap();
        ranges.push(Range {
            start: start_str.parse().unwrap(),
            end: end_str.parse().unwrap(),
        });
        index += 1;
    }

    index += 1; // Skip the empty line

    let ingredients: Vec<u64> = lines[index..]
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();

    Data {
        ranges: ranges,
        ingredients: ingredients,
    }
}

fn part1(data: &Data) -> u64 {
    data.ingredients
        .iter()
        .filter(|ingredient| {
            data.ranges
                .iter()
                .any(|range| range.start <= **ingredient && **ingredient <= range.end)
        })
        .count() as u64
}

fn part2(mut ranges: Vec<Range>) -> u64 {
    let mut result = 0;
    let mut cursor = 0;

    ranges.sort_by_key(|r| r.start);
    for range in ranges {
        let start = cursor.max(range.start);
        if start <= range.end {
            result += range.end - start + 1;
        }
        cursor = cursor.max(range.end + 1);
    }
    result
}

fn main() {
    let input = include_str!("../../inputs/05.txt");
    let data = parse_input(input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(data.ranges));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_day5_part1() {
        let input = include_str!("../../examples/05.txt");
        let data = parse_input(input);
        assert_eq!(part1(&data), 3);
    }

    #[test]
    fn test_example_day5_part2() {
        let input = include_str!("../../examples/05.txt");
        let data = parse_input(input);
        assert_eq!(part2(data.ranges), 14);
    }
}
