fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .split(',')
        .map(|range| range.split('-'))
        .map(|mut bounds| {
            let start = bounds.next().unwrap().parse::<u64>().unwrap();
            let end = bounds.next().unwrap().parse::<u64>().unwrap();
            (start, end)
        })
        .collect()
}

// On coupe le nombre en deux parties : 12 et 34...
// la partie de gauche devient notre curseur. Le première nombre qu'on va considérer le curseur doublés
// Si la partie de gauche est plus petite que la partie de droite ajouter 1 au curseur

fn part1_single_range(begin_range: u64, end_range: u64) -> u64 {
    let num_digits = begin_range.ilog10() + 1;
    let divisor = 10u64.pow((num_digits + 1) / 2);
    let begin_right = begin_range % divisor;
    let mut begin_left = begin_range / divisor;
    // add a one if odd number of digits
    begin_left += (num_digits % 2 * 10u32.pow(num_digits / 2)) as u64;
    // Start is left part + 1 if left < right
    let start = if num_digits % 2 == 0 {
        if begin_left < begin_right {
            begin_left + 1
        } else {
            begin_left
        }
    } else {
        10u64.pow(num_digits / 2)
    };

    let num_digits = end_range.ilog10() + 1;
    let divisor = 10u64.pow((num_digits + 1) / 2);
    let end_right = end_range % divisor;
    let end_left = end_range / divisor;
    let end = if num_digits % 2 == 0 {
        if end_right < end_left {
            end_left - 1
        } else {
            end_left
        }
    } else {
        10u64.pow(num_digits / 2) - 1
    };

    (start..=end)
        .map(|x| double(x))
        .fold(0u64, |acc, x| acc + x)
}

fn part1(data: Vec<(u64, u64)>) -> u64 {
    data.into_iter()
        .map(|(start, end)| part1_single_range(start, end))
        .fold(0u64, |acc, x| acc + x)
}

fn double(x: u64) -> u64 {
    let num_digits = x.ilog10() + 1;
    let mul = 10u64.pow(num_digits);
    x * mul + x
}

fn main() {
    let input = include_str!("../../inputs/02.txt");
    let data = parse_input(input);
    println!("Part 1: {}", part1(data));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double() {
        assert_eq!(double(12), 1212);
        assert_eq!(double(123), 123123);
        assert_eq!(double(1), 11);
    }

    #[test]
    fn test_part1_single_range() {
        assert_eq!(part1_single_range(11, 22), 11 + 22);
        assert_eq!(part1_single_range(1698522, 1698528), 0);
    }

    #[test]
    fn test_example_part1() {
        let input = include_str!("../../examples/02.txt");
        let data = parse_input(input);
        assert_eq!(part1(data), 1227775554);
    }
}
