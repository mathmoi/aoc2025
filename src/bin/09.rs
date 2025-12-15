use std::{iter::once, ops::Range};

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split(',');
            let x = parts.next().unwrap().parse().unwrap();
            let y = parts.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect()
}

fn part1(data: &[(u64, u64)]) -> u64 {
    let mut max_area = u64::MIN;
    for index1 in 0..data.len() {
        for index2 in (index1 + 1)..data.len() {
            let area = (data[index1].0 as i64 - data[index2].0 as i64 + 1).abs()
                * (data[index1].1 as i64 - data[index2].1 as i64 + 1).abs();
            max_area = u64::max(max_area, area as u64);
        }
    }
    max_area
}

fn compact<I>(indices: I) -> Vec<Range<u64>>
where
    I: Iterator<Item = u64>,
{
    (once(0u64) // Add start and end bounds
        .chain(indices.sorted().dedup())
        .chain(once(100_000u64)))
    .tuple_windows() // Create pairs of consecutive elements
    .map(|(from, to)| from..to) // Create ranges from pairs
    .collect()
}

fn downgrade(compaction: &Vec<Range<u64>>, coord: u64) -> u64 {
    compaction
        .binary_search_by(|range| {
            if range.contains(&coord) {
                std::cmp::Ordering::Equal
            } else if range.start > coord {
                std::cmp::Ordering::Greater
            } else {
                std::cmp::Ordering::Less
            }
        })
        .unwrap() as u64
}

fn between(row1: u64, col1: u64, row2: u64, col2: u64) -> impl Iterator<Item = (u64, u64)> {
    if row1 == row2 {
        let start = col1.min(col2);
        let end = col1.max(col2);
        return ((start + 1)..end)
            .map(move |col| (row1, col))
            .collect::<Vec<_>>()
            .into_iter();
    } else if col1 == col2 {
        let start = row1.min(row2);
        let end = row1.max(row2);
        return ((start + 1)..end)
            .map(move |row| (row, col1))
            .collect::<Vec<_>>()
            .into_iter();
    } else {
        panic!("Not a straight line");
    }
}

fn fill(grid: &mut Vec<Vec<char>>, start_row: usize, start_col: usize) {
    let mut stack = vec![(start_row, start_col)];

    grid[start_row][start_col] = 'F';

    while stack.len() > 0 {
        let (row, col) = stack.pop().unwrap();
        for (drow, dcol) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            let new_row = row as isize + drow;
            let new_col = col as isize + dcol;
            if new_row >= 0
                && new_row < grid.len() as isize
                && new_col >= 0
                && new_col < grid[0].len() as isize
            {
                if grid[new_row as usize][new_col as usize] == '.' {
                    grid[new_row as usize][new_col as usize] = 'X';
                    stack.push((new_row as usize, new_col as usize));
                }
            }
        }
    }
}

fn part2(data: &[(u64, u64)]) -> u64 {
    // Create the compactions for rows and columns
    let rows_compaction = compact(data.iter().map(|(row, _)| *row));
    let cols_compaction = compact(data.iter().map(|(_, col)| *col));

    // Create the compacted grid and add the red and greed tiles
    let mut grid = vec![vec!['.'; rows_compaction.len()]; cols_compaction.len()];
    for ((row1, col1), (row2, col2)) in data.iter().chain(data.first()).tuple_windows() {
        let crow1 = downgrade(&rows_compaction, *row1);
        let ccol1 = downgrade(&cols_compaction, *col1);
        let crow2 = downgrade(&rows_compaction, *row2);
        let ccol2 = downgrade(&cols_compaction, *col2);
        grid[crow1 as usize][ccol1 as usize] = '#';

        for (crow, ccol) in between(crow1, ccol1, crow2, ccol2) {
            grid[crow as usize][ccol as usize] = 'X';
        }
    }

    // Find a point inside the zone
    let row = rows_compaction.len() / 2;
    let mut col = 0;
    while grid[row][col] == '.' {
        col += 1;
    }
    while grid[row][col] != '.' {
        col += 1;
    }

    // Fill the area
    fill(&mut grid, row, col);

    // Create the prefix map
    let mut px_map: Vec<Vec<u64>> = Vec::with_capacity(grid.len());
    for row in 0..grid.len() {
        px_map.push(Vec::with_capacity(grid[0].len()));
        for col in 0..grid[0].len() {
            let mut value = 0;
            if row > 0 {
                value += px_map[row - 1][col];
            }
            if col > 0 {
                value += px_map[row][col - 1];
            }
            if row > 0 && col > 0 {
                value -= px_map[row - 1][col - 1];
            }
            if grid[row][col] == '.' {
                value += 1;
            }
            px_map[row].push(value);
        }
    }

    // Find valid squares
    let mut max_area = u64::MIN;
    for index1 in 0..data.len() {
        for index2 in (index1 + 1)..data.len() {
            let top = data[index1].0.min(data[index2].0);
            let bottom = data[index1].0.max(data[index2].0);
            let left = data[index1].1.min(data[index2].1);
            let right = data[index1].1.max(data[index2].1);

            let area = (bottom - top + 1) * (right - left + 1);
            if max_area < area as u64 {
                // The area is bigger than the best so far. Check if it's valid.
                let ctop = downgrade(&rows_compaction, top);
                let cbottom = downgrade(&rows_compaction, bottom);
                let cleft = downgrade(&cols_compaction, left);
                let cright = downgrade(&cols_compaction, right);

                // Count the invalid cells in the rectangle using the prefix map.
                let mut invalid_count = 0u64;
                
                // Add the area above the rectangle
                if ctop > 0 {
                    invalid_count += px_map[(ctop - 1) as usize][cright as usize];
                }
                
                // Add the area to the left of the rectangle
                if cleft > 0 {
                    invalid_count += px_map[cbottom as usize][(cleft - 1) as usize];
                }
                
                // Subtract the area above-left (added twice)
                if ctop > 0 && cleft > 0 {
                    invalid_count -= px_map[(ctop - 1) as usize][(cleft - 1) as usize];
                }

                // If the invalid count at the bottom-right corner equals the total 
                // invalid count above and to the left it means there is no invalid cell 
                // inside the rectangle.
                if invalid_count == px_map[cbottom as usize][cright as usize] {
                    max_area = area;
                }
            }
        }
    }
    max_area
}

fn main() {
    let input = include_str!("../../inputs/09.txt");
    let data = parse_input(input);
    println!("Part 1: {}", part1(&data));
    println!("Part 2: {}", part2(&data));
}
