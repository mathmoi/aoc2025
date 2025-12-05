#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i16,
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let direction = match line.chars().next().expect("Line should never be empty") {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => panic!("Invalid direction"),
            };
            let distance: i16 = line[1..].parse().expect("Invalid distance");
            Instruction {
                direction,
                distance,
            }
        })
        .collect()
}

fn part1(data: &Vec<Instruction>) -> i16 {
    let mut result = 0;
    let mut dial: i16 = 50;
    for instruction in data {
        match instruction.direction {
            Direction::Left => dial = (dial - instruction.distance).rem_euclid(100),
            Direction::Right => dial = (dial + instruction.distance).rem_euclid(100),
        };
        if dial == 0 {
            result += 1;
        }
    }
    result
}

fn part2(data: &Vec<Instruction>) -> i16 {
    let mut result = 0;
    let mut dial: i16 = 50;
    for instruction in data {
        match instruction.direction {
            Direction::Left => {
                result += ((100 - dial).rem_euclid(100) + instruction.distance) / 100;
                dial = (dial - instruction.distance).rem_euclid(100)
            }
            Direction::Right => {
                result += (dial + instruction.distance) / 100;
                dial = (dial + instruction.distance).rem_euclid(100)
            }
        };
    }
    if dial == 0 {
        result += 1;
    }
    result
}

fn main() {
    let input = include_str!("../../inputs/01.txt");
    let data = parse_input(input);

    println!("Part 1 : {}", part1(&data));
    println!("Part 2 : {}", part2(&data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part1() {
        let input = include_str!("../../examples/01.txt");
        let data = parse_input(input);
        assert_eq!(part1(&data), 3);
    }

    #[test]
    fn test_example_part2() {
        let input = include_str!("../../examples/01.txt");
        let data = parse_input(input);
        assert_eq!(part2(&data), 6);
    }

    fn l(dial: i16, dist: i16) -> i16 {
        ((100 - dial).rem_euclid(100) + dist) / 100
    }

    fn r(dial: i16, dist: i16) -> i16 {
        (dial + dist) / 100
    }

    #[test]
    fn my_test() {
        assert_eq!(l(0, 1), 0);
        assert_eq!(l(0, 99), 0);
        assert_eq!(l(0, 100), 1);

        assert_eq!(l(10, 9), 0);
        assert_eq!(l(10, 10), 1);
        assert_eq!(l(10, 11), 1);

        assert_eq!(l(10, 109), 1);
        assert_eq!(l(10, 110), 2);
        assert_eq!(l(10, 111), 2);

        assert_eq!(r(10, 89), 0);
        assert_eq!(r(10, 90), 1);
        assert_eq!(r(10, 91), 1);

        assert_eq!(r(10, 189), 1);
        assert_eq!(r(10, 190), 2);
        assert_eq!(r(10, 191), 2);

        assert_eq!(r(0, 1), 0);
        assert_eq!(r(0, 99), 0);
        assert_eq!(r(0, 100), 1);
    }
}
