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
}
