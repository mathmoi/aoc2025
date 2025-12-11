fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

fn accessible(line: usize, column: usize, data: &Vec<Vec<bool>>) -> bool {
    #[rustfmt::skip]
    let directions = [(-1, -1), (-1,  0), (-1,  1),
                      ( 0, -1),           ( 0,  1),
                      ( 1, -1), ( 1,  0), ( 1,  1)];
    let mut adjacent = 0;
    for (delta_line, delta_column) in directions {
        let target_line = line as isize + delta_line;
        let target_column = column as isize + delta_column;
        if 0 <= target_line
            && target_line < data.len() as isize
            && 0 <= target_column
            && target_column < data[line].len() as isize
        {
            if data[target_line as usize][target_column as usize] {
                adjacent += 1;
            }
        }
    }

    adjacent < 4
}

fn part1(data: &Vec<Vec<bool>>) -> usize {
    let mut result = 0;
    for line in 0..data.len() {
        for column in 0..data[line].len() {
            if data[line][column] && accessible(line, column, data) {
                result += 1;
            }
        }
    }
    result
}

fn part2(data: &Vec<Vec<bool>>) -> usize {
    let mut data = data.clone();
    let mut result = 0;
    loop {
        let mut remove: Vec<(usize, usize)> = Vec::new();
        for line in 0..data.len() {
            for column in 0..data[line].len() {
                if data[line][column] && accessible(line, column, &data) {
                    remove.push((line, column));
                }
            }
        }

        if remove.is_empty() {
            break;
        }

        result += remove.len();

        for (line, column) in remove {
            data[line][column] = false;
        }
    }
    result
}

fn main() {
    let input = include_str!("../../inputs/04.txt");
    let data = parse_input(input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exemple_day4_part1() {
        let input = include_str!("../../examples/04.txt");
        let data = parse_input(input);
        assert_eq!(part1(&data), 13);
    }

    #[test]
    fn test_exemple_day4_part2() {
        let input = include_str!("../../examples/04.txt");
        let data = parse_input(input);
        assert_eq!(part2(&data), 43);
    }
}
