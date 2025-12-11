type Data = Vec<(Vec<u64>, char)>;

fn parse_input(input: &str) -> Data {
    let mut table: Vec<Vec<&str>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    let operations = table.pop().unwrap();

    let mut data = Vec::new();
    for index in 0..table[0].len() {
        data.push((
            table
                .iter()
                .map(|row| row[index].parse().unwrap())
                .collect(),
            operations[index].chars().nth(0).unwrap(),
        ));
    }
    data
}

fn parse_input_part2(input: &str) -> Data {
    // Create a 2D vector to hold the characters
    let table: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // Pad all the rows to the same length
    let length = table.iter().map(|line| line.len()).max().unwrap();
    let table: Vec<Vec<char>> = table
        .into_iter()
        .map(|mut line| {
            line.resize(length, ' ');
            line
        })
        .collect();

    // Rotate the table 90 degrees anti-clockwise
    let mut columns: Vec<String> = Vec::new();
    for index in 0..table[0].len() {
        columns.push(
            table
                .iter()
                .map(|line| &line[index])
                .copied()
                .collect::<String>()
                .to_string(),
        );
    }

    let problems = columns.split(|c| c.trim().is_empty());
    let mut result : Data = Vec::new();
    for problem in problems {
        result.push((
            problem.iter().map(|l| l[..l.len() - 1].trim().parse().unwrap()).collect(),
            problem[0].chars().last().unwrap()));
    }

    result
}

fn part1(data: &Data) -> u64 {
    data.iter()
        .map(|(numbers, operator)| match operator {
            '+' => numbers.iter().sum(),
            '*' => numbers.iter().product(),
            _ => 0,
        })
        .sum()
}

fn main() {
    let input = include_str!("../../inputs/06.txt");
    let data = parse_input(input);
    println!("Part 1: {}", part1(&data));
    
    let data2 = parse_input_part2(input);
    println!("Part 2: {}", part1(&data2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_day6_part1() {
        let input = include_str!("../../examples/06.txt");
        let data = parse_input(input);
        assert_eq!(part1(&data), 4277556);
    }

    #[test]
    fn test_example_day6_part2() {
        let input = include_str!("../../examples/06.txt");
        let data = parse_input_part2(input);
        assert_eq!(part1(&data), 3263827);
    }
}
