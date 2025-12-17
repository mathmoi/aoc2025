use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, space1},
    combinator::{recognize, value},
    multi::{many1, separated_list1},
    sequence::{delimited, terminated},
};

#[derive(Debug, Clone)]
enum Light {
    On,
    Off,
}

#[derive(Debug, Clone)]
struct Machine {
    light_diagram: u32,
    buttons: Vec<u32>,
}

fn parse_light(input: &str) -> IResult<&str, Light> {
    alt((value(Light::Off, tag(".")), value(Light::On, tag("#")))).parse(input)
}

fn parse_light_diagram(input: &str) -> IResult<&str, u32> {
    let (input, lights) = delimited(tag("["), many1(parse_light), tag("]")).parse(input)?;

    let result = lights
        .iter()
        .enumerate()
        .fold(0u32, |acc, (i, light)| match light {
            Light::On => acc | (1 << i),
            Light::Off => acc,
        });

    Ok((input, result))
}

fn parse_button(input: &str) -> IResult<&str, u32> {
    let (input, buttons_indexes) =
        delimited(tag("("), separated_list1(tag(","), complete::u32), tag(")")).parse(input)?;
    let result = buttons_indexes
        .iter()
        .fold(0u32, |acc, &index| acc | (1 << index));
    Ok((input, result))
}

fn parse_buttons(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, parse_button).parse(input)
}

fn parse_joltage(input: &str) -> IResult<&str, ()> {
    let (input, _) = recognize(delimited(
        tag("{"),
        separated_list1(tag(","), complete::u32),
        tag("}"),
    ))
    .parse(input)?;
    Ok((input, ()))
}

fn parse_machine(input: &str) -> IResult<&str, Machine> {
    let (input, (light_diagram, buttons, _joltages)) = (
        terminated(parse_light_diagram, space1),
        terminated(parse_buttons, space1),
        parse_joltage,
    )
        .parse(input)?;
    Ok((
        input,
        Machine {
            light_diagram,
            buttons,
        },
    ))
}

fn part1_recursive(machine: &Machine, status: u32, depth: u32) -> bool {
    if status == machine.light_diagram {
        return true;
    }

    if depth == 0 {
        return false;
    }

    for &button in &machine.buttons {
        let new_status = status ^ button;
        if part1_recursive(machine, new_status, depth - 1) {
            return true;
        }
    }

    false
}

fn part1(data: &[Machine]) -> u32 {
    let mut result = 0u32;
    for machine in data {
        let mut depth = 1u32;
        loop {
            if part1_recursive(machine, 0, depth) {
                break;
            }
            depth += 1;
        }
        result += depth;
    }
    result
}

fn parse_input(input: &str) -> Vec<Machine> {
    let (input, data) = separated_list1(tag("\n"), parse_machine)
        .parse(input)
        .expect("Failed to parse data");
    if !input.is_empty() {
        panic!("Unparsed input remaining: {}", input);
    }
    data
}

fn main() {
    let input = include_str!("../../inputs/10.txt");
    let data = parse_input(input);
    println!("Part 1: {}", part1(&data));
}
