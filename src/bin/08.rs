use std::collections::HashMap;

type CircuitId = u16;
type BoxId = u16;
type Position = (u64, u64, u64);

#[derive(Debug, Clone)]
struct Box {
    circuit_id: CircuitId,
    position: Position,
}

#[derive(Debug, Clone)]
struct Data {
    boxes: Vec<Box>,
    circuits: HashMap<CircuitId, Vec<BoxId>>,
}

fn parse_input(input: &str) -> Data {
    let boxes: Vec<Box> = input
        .lines()
        .enumerate()
        .map(|(id, line)| Box {
            circuit_id: id as CircuitId,
            position: {
                let mut iter = line.split(',').map(|s| s.parse::<u64>().unwrap());
                (
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                )
            },
        })
        .collect();

    let circuits: HashMap<CircuitId, Vec<BoxId>> = boxes
        .iter()
        .map(|b| (b.circuit_id, vec![b.circuit_id]))
        .collect();

    Data { boxes, circuits }
}

fn calculate_distance(box1: &Box, box2: &Box) -> u64 {
    let dx = (box1.position.0 as i64 - box2.position.0 as i64).abs() as u64;
    let dy = (box1.position.1 as i64 - box2.position.1 as i64).abs() as u64;
    let dz = (box1.position.2 as i64 - box2.position.2 as i64).abs() as u64;
    dx * dx + dy * dy + dz * dz
}

fn part1(mut data: Data, connections: usize) -> usize {
    let mut pairs: Vec<(BoxId, BoxId, u64)> = Vec::new();
    for index1 in 0..data.boxes.len() {
        for index2 in (index1 + 1)..data.boxes.len() {
            pairs.push((
                index1 as BoxId,
                index2 as BoxId,
                calculate_distance(&data.boxes[index1], &data.boxes[index2]),
            ));
        }
    }

    pairs.sort_by(|&(_, _, dist_a), &(_, _, dist_b)| dist_a.cmp(&dist_b));

    for (box_id1, box_id2, _) in pairs.iter().take(connections) {
        let to_circuit_id = data.boxes[*box_id1 as usize].circuit_id;
        let from_circuit_id = data.boxes[*box_id2 as usize].circuit_id;

        if to_circuit_id == from_circuit_id {
            continue;
        }
        
        for box_id in data.circuits.get(&from_circuit_id).unwrap().iter() {
            data.boxes[*box_id as usize].circuit_id = to_circuit_id;
        }

        if let Some(from_circuit) = data.circuits.remove(&from_circuit_id) {
            data.circuits
                .get_mut(&to_circuit_id)
                .unwrap()
                .extend(from_circuit);
        }
    }

    let mut lengths: Vec<usize> = data
        .circuits
        .iter()
        .map(|(_, circuit)| circuit.len())
        .collect();
    lengths.sort();
    lengths.iter().rev().take(3).product()
}

fn part2(mut data: Data) -> u64 {
    let mut pairs: Vec<(BoxId, BoxId, u64)> = Vec::new();
    for index1 in 0..data.boxes.len() {
        for index2 in (index1 + 1)..data.boxes.len() {
            pairs.push((
                index1 as BoxId,
                index2 as BoxId,
                calculate_distance(&data.boxes[index1], &data.boxes[index2]),
            ));
        }
    }

    pairs.sort_by(|&(_, _, dist_a), &(_, _, dist_b)| dist_a.cmp(&dist_b));

    for (box_id1, box_id2, _) in pairs.iter() {
        let to_circuit_id = data.boxes[*box_id1 as usize].circuit_id;
        let from_circuit_id = data.boxes[*box_id2 as usize].circuit_id;

        if to_circuit_id == from_circuit_id {
            continue;
        }
        
        for box_id in data.circuits.get(&from_circuit_id).unwrap().iter() {
            data.boxes[*box_id as usize].circuit_id = to_circuit_id;
        }

        if let Some(from_circuit) = data.circuits.remove(&from_circuit_id) {
            data.circuits
                .get_mut(&to_circuit_id)
                .unwrap()
                .extend(from_circuit);
        }
        
        if data.circuits.get(&to_circuit_id).unwrap().len() == data.boxes.len() {
            return data.boxes[*box_id1 as usize].position.0 * data.boxes[*box_id2 as usize].position.0;
        }
    }
    
    unreachable!()
}

fn main() {
    let input = include_str!("../../inputs/08.txt");
    let data = parse_input(input);
    //println!("Data: {:?}", data.clone());
    println!("Part 1: {}", part1(data.clone(), 1000));
    println!("Part 2: {}", part2(data.clone()));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../../examples/08.txt");
        let data = parse_input(input);
        assert_eq!(part1(data, 10), 40);
    }
    
    #[test]
    fn test_part2() {
        let input = include_str!("../../examples/08.txt");
        let data = parse_input(input);
        assert_eq!(part2(data), 25272);
    }
}
