use std::collections::HashMap;

type Data = Vec<Vec<State>>;
type Memoize = HashMap<(usize, usize), u64>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum State {
    Start,
    Beam,
    Empty,
    Splitter,
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            'S' => State::Start,
            '|' => State::Beam,
            '.' => State::Empty,
            '^' => State::Splitter,
            _ => panic!("Unknown state character: {}", c),
        }
    }
}

impl From<State> for char {
    fn from(s: State) -> Self {
        match s {
            State::Start => 'S',
            State::Beam => '|',
            State::Empty => '.',
            State::Splitter => '^',
        }
    }
}

fn parse_input(input: &str) -> Data {
    input
        .lines()
        .map(|l| l.chars().map(|c| c.into()).collect())
        .collect()
}

fn part1(data: &Data) -> u32 {
    let mut data = data.clone();

    // Propagate beams downwards
    let mut splits = 0u32;
    for line in 1..data.len() {
        for column in 1..data[line].len() - 1 {
            // Match on (top_left, top, top_right, left, self, right)
            data[line][column] = match (
                data[line - 1][column - 1],
                data[line - 1][column],
                data[line - 1][column + 1],
                data[line][column - 1],
                data[line][column],
                data[line][column + 1],
            ) {
                (_, State::Start, _, _, State::Empty, _) => State::Beam,
                (_, _, State::Beam, _, State::Empty, State::Splitter) => State::Beam,
                (State::Beam, _, _, State::Splitter, State::Empty, _) => State::Beam,
                (_, State::Beam, _, _, State::Empty, _) => State::Beam,
                (_, State::Beam, _, _, State::Splitter, _) => {
                    splits += 1;
                    State::Splitter
                }
                _ => data[line][column],
            }
        }
    }
    splits
}

fn part2_recursive(data: &Data, line: usize, column: usize, memoize: &mut Memoize) -> u64 {
    if data.len() <= line {
        return 1;
    }

    if let Some(memoized_result) = memoize.get(&(line, column)) {
        return *memoized_result;
    }

    let result = match data[line][column] {
        State::Empty | State::Start => part2_recursive(data, line + 1, column, memoize),
        State::Splitter => {
            part2_recursive(data, line + 1, column - 1, memoize)
                + part2_recursive(data, line + 1, column + 1, memoize)
        }
        _ => panic!("Unexpected state found"),
    };

    memoize.insert((line, column), result);
    result
}

fn part2(data: &Data) -> u64 {
    let mut memoize: HashMap<(usize, usize), u64> = HashMap::new();

    let start_column = data[0]
        .iter()
        .position(|&s| s == State::Start)
        .expect("No start found in the first row");
    part2_recursive(&data, 0, start_column, &mut memoize)
}

fn main() {
    let input = include_str!("../../inputs/07.txt");
    let data = parse_input(input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day7_part1() {
        let input = include_str!("../../examples/07.txt");
        let data = parse_input(input);
        assert_eq!(part1(&data), 21);
    }

    #[test]
    fn test_day7_part2() {
        let input = include_str!("../../examples/07.txt");
        let data = parse_input(input);
        assert_eq!(part2(&data), 40);
    }
}
